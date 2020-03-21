use std::cmp::Ordering;
use std::convert::TryInto;
use std::ffi::CStr;
use std::iter;
use std::marker::PhantomData;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::{self, NonNull};
use std::time::Duration;

use libc::time_t;
use time::OffsetDateTime;

use crate::context::{Context, ContextHandle};
use crate::events::{Event, EventAttrs, PrintEvent, ServerEvent};
use crate::ffi::{
    hexchat_event_attrs, hexchat_list, hexchat_plugin, int_to_result, word_to_iter, ListElem,
    StrExt,
};
use crate::gui::FakePluginHandle;
use crate::hook::{self, HookHandle};
use crate::info::{FromInfoValue, FromListElem, FromPrefValue, Info, List, Pref, PrefValue};
use crate::mode;
use crate::state::{catch_and_log_unwind, with_plugin_state};
use crate::strip;

/// Must be implemented by all HexChat plugins.
///
/// # Examples
///
/// ```rust
/// use std::cell::Cell;
/// use std::time::SystemTime;
/// use hexavalent::{Plugin, PluginHandle};
/// use hexavalent::events::{Event, EventAttrs, PrintEvent};
/// use hexavalent::events::print::ChannelMessage;
/// use hexavalent::hook::{Eat, Priority};
///
/// struct StatsPlugin {
///     start: Cell<SystemTime>,
///     messages: Cell<usize>,
///     characters: Cell<usize>,
/// }
///
/// impl Default for StatsPlugin {
///     fn default() -> Self {
///         Self {
///             start: Cell::new(SystemTime::now()),
///             messages: Cell::new(0),
///             characters: Cell::new(0),
///         }
///     }
/// }
///
/// impl StatsPlugin {
///     fn message_cb(
///         &self,
///         ph: PluginHandle<'_, Self>,
///         [_, text, _, _]: <ChannelMessage as Event<'_>>::Args,
///     ) -> Eat {
///         self.messages.set(self.messages.get() + 1);
///         self.characters.set(self.characters.get() + text.chars().count());
///         Eat::None
///     }
///
///     fn print_stats(&self, ph: PluginHandle<'_, Self>) {
///         let elapsed = self.start.get().elapsed().unwrap();
///
///         let messages = self.messages.get();
///         let avg_msgs = messages as f64 / (elapsed.as_secs_f64() / 60.);
///         ph.print(&format!("Messages: {} ({:.1}/min).\0", messages, avg_msgs));
///
///         let characters = self.characters.get();
///         let avg_chars = characters as f64 / messages as f64;
///         ph.print(&format!("Characters: {} ({:.1}/msg).\0", characters, avg_chars));
///     }
/// }
///
/// impl Plugin for StatsPlugin {
///     fn init(&self, ph: PluginHandle<'_, Self>) {
///         ph.hook_command(
///             "stats\0",
///             "Usage: STATS, print message statistics\0",
///             Priority::Normal,
///             |plugin, ph, words| {
///                 plugin.print_stats(ph);
///                 Eat::All
///             },
///         );
///         ph.hook_print(ChannelMessage, Priority::Normal, Self::message_cb);
///     }
///
///     fn deinit(&self, ph: PluginHandle<'_, Self>) {
///         ph.print("Overall stats:\0");
///         self.print_stats(ph);
///     }
/// }
/// ```
pub trait Plugin: Default + 'static {
    /// Initialize your plugin.
    ///
    /// Use this function to perform any work that should be done when your plugin is loaded,
    /// such as registering hooks or printing startup messages.
    ///
    /// Analogous to [`hexchat_plugin_init`](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::{Plugin, PluginHandle};
    ///
    /// #[derive(Default)]
    /// struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn init(&self, ph: PluginHandle<'_, Self>) {
    ///         ph.print("Plugin loaded successfully!\0");
    ///     }
    /// }
    /// ```
    fn init(&self, ph: PluginHandle<'_, Self>);

    /// Deinitialize your plugin.
    ///
    /// Use this function to perform any work that should be done when your plugin is unloaded,
    /// such as printing shutdown messages or statistics.
    ///
    /// You do not need to call [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) in this function,
    /// as remaining hooks are automatically removed by HexChat when your plugin finishes unloading.
    ///
    /// Analogous to [`hexchat_plugin_deinit`](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::{Plugin, PluginHandle};
    ///
    /// #[derive(Default)]
    /// struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn init(&self, _: PluginHandle<'_, Self>) {}
    ///
    ///     fn deinit(&self, ph: PluginHandle<'_, Self>) {
    ///         ph.print("Plugin unloading...\0");
    ///     }
    /// }
    /// ```
    fn deinit(&self, ph: PluginHandle<'_, Self>) {
        let _ = ph;
    }
}

/// Interacts with HexChat's plugin API.
///
/// Cannot be constructed in user code, but is passed into
/// [`Plugin::init`](trait.Plugin.html#tymethod.init),
/// [`Plugin::deinit`](trait.Plugin.html#method.deinit),
/// and hook callbacks such as [`PluginHandle::hook_command`](struct.PluginHandle.html#method.hook_command).
///
/// Most of HexChat's [functions](https://hexchat.readthedocs.io/en/latest/plugins.html#functions) are available as associated functions,
/// without the `hexchat_` prefix.
///
/// # Examples
///
/// All functions which take `&str`/`impl AsRef<str>` arguments will allocate if the string is not null-terminated,
/// and panic if the string contains interior nulls.
///
/// ```rust
/// # use hexavalent::PluginHandle;
/// # fn print_some_stuff<P>(ph: PluginHandle<'_, P>) {
/// // for example, this would not allocate
/// ph.print("hello\0");
/// // ...this would allocate
/// ph.print("hello");
/// // ...and this would panic
/// ph.print("hel\0lo");
/// # }
/// ```
#[derive(Debug)]
pub struct PluginHandle<'ph, P> {
    /// Always points to a valid instance of `hexchat_plugin`.
    handle: *mut hexchat_plugin,
    _lifetime: PhantomData<&'ph hexchat_plugin>,
    _plugin: PhantomData<P>,
}

impl<'ph, P> Copy for PluginHandle<'ph, P> {}
impl<'ph, P> Clone for PluginHandle<'ph, P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'ph, P> PluginHandle<'ph, P> {
    /// Creates a new `PluginHandle` from a native `hexchat_plugin`.
    ///
    /// # Safety
    ///
    /// `plugin_handle` must point to a valid instance of `hexchat_plugin`.
    pub(crate) unsafe fn new(plugin_handle: *mut hexchat_plugin) -> Self {
        Self {
            handle: plugin_handle,
            _lifetime: PhantomData,
            _plugin: PhantomData,
        }
    }
}

/// [General Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#general-functions)
///
/// General functions allow printing text, running commands, creating events, and other miscellaneous operations.
impl<'ph, P> PluginHandle<'ph, P> {
    /// Prints text to the current [context](struct.PluginHandle.html#impl-3). Text may contain mIRC color codes and formatting.
    ///
    /// Analogous to [`hexchat_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn say_hello<P>(ph: PluginHandle<'_, P>) {
    ///     // null-termination is not required, but avoids allocation
    ///     ph.print("hello!\0");
    /// }
    /// ```
    pub fn print(self, text: &str) {
        let text = text.into_cstr();
        // Safety: `handle` is always valid
        unsafe {
            ((*self.handle).hexchat_print)(self.handle, text.as_ptr());
        }
    }

    /// Executes a command in the current [context](struct.PluginHandle.html#impl-3) as if it were typed into HexChat's input box after a `/`.
    ///
    /// Analogous to [`hexchat_command`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_command).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn op_user<P>(ph: PluginHandle<'_, P>, username: &str) {
    ///     // do not include the leading slash
    ///     ph.command(&format!("OP {}\0", username));
    /// }
    /// ```
    pub fn command(self, cmd: &str) {
        let cmd = cmd.into_cstr();
        // Safety: `handle` is always valid
        unsafe {
            ((*self.handle).hexchat_command)(self.handle, cmd.as_ptr());
        }
    }

    /// Emits a print event in the current [context](struct.PluginHandle.html#impl-3).
    ///
    /// See the [`events::print`](events/print/index.html) submodule for a list of print events.
    ///
    /// Note that this triggers any print hooks registered for the event, so be careful to avoid infinite recursion
    /// when calling this function from hook callbacks such as [`PluginHandle::hook_print`](struct.PluginHandle.html#method.hook_print).
    ///
    /// Analogous to [`hexchat_emit_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::events::print::ChannelMessage;
    ///
    /// fn print_fake_message<P>(ph: PluginHandle<'_, P>, user: &str, text: &str) -> Result<(), ()> {
    ///     ph.emit_print(ChannelMessage, [user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print<E: PrintEvent>(
        self,
        event: E,
        args: <E as Event<'_>>::Args,
    ) -> Result<(), ()> {
        let _ = event;
        E::args_to_c(args, |args| {
            assert!(
                args.len() <= 4,
                "bug in hexavalent - more than 4 args from PrintEvent"
            );

            let args: [*const c_char; 4] = [
                args.get(0).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(1).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(2).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(3).map_or_else(ptr::null, |a| a.as_ptr()),
            ];

            // Safety: `handle` is always valid
            int_to_result(unsafe {
                ((*self.handle).hexchat_emit_print)(
                    self.handle,
                    E::NAME,
                    args[0],
                    args[1],
                    args[2],
                    args[3],
                    ptr::null::<c_char>(),
                )
            })
        })
    }

    /// Emits a print event in the current [context](struct.PluginHandle.html#impl-3), specifying its attributes.
    ///
    /// See the [`events::print`](events/print/index.html) submodule for a list of print events.
    ///
    /// Note that this triggers any print hooks registered for the event, so be careful to avoid infinite recursion
    /// when calling this function from hook callbacks such as [`PluginHandle::hook_print_attrs`](struct.PluginHandle.html#method.hook_print_attrs).
    ///
    /// Analogous to [`hexchat_emit_print_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print_attrs).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::events::EventAttrs;
    /// use hexavalent::events::print::ChannelMessage;
    /// use time::OffsetDateTime;
    ///
    /// fn print_fake_message_like_its_1979<P>(ph: PluginHandle<'_, P>, user: &str, text: &str) -> Result<(), ()> {
    ///     let attrs = EventAttrs::new(OffsetDateTime::from_unix_timestamp(86400 * 365 * 10));
    ///     ph.emit_print_attrs(ChannelMessage, attrs, [user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print_attrs<E: PrintEvent>(
        self,
        event: E,
        attrs: EventAttrs<'_>,
        args: <E as Event<'_>>::Args,
    ) -> Result<(), ()> {
        let _ = event;
        E::args_to_c(args, |args| {
            assert!(
                args.len() <= 4,
                "bug in hexavalent - more than 4 args from PrintEvent"
            );

            let args: [*const c_char; 4] = [
                args.get(0).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(1).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(2).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(3).map_or_else(ptr::null, |a| a.as_ptr()),
            ];

            // Safety: `handle` is always valid
            int_to_result(unsafe {
                let event_attrs = ((*self.handle).hexchat_event_attrs_create)(self.handle);
                defer! { ((*self.handle).hexchat_event_attrs_free)(self.handle, event_attrs) };

                ptr::write(
                    &mut (*event_attrs).server_time_utc as *mut _,
                    attrs.time().timestamp() as time_t,
                );

                ((*self.handle).hexchat_emit_print_attrs)(
                    self.handle,
                    event_attrs,
                    E::NAME,
                    args[0],
                    args[1],
                    args[2],
                    args[3],
                    ptr::null::<c_char>(),
                )
            })
        })
    }

    /// Sends channel mode changes to targets in the current [context](struct.PluginHandle.html#impl-3).
    ///
    /// Analogous to [`hexchat_send_modes`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_send_modes).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::mode::Sign;
    ///
    /// fn op_users<P>(ph: PluginHandle<'_, P>, users: &[&str]) {
    ///     // sends `MODE <users> +o`
    ///     ph.send_modes(users, Sign::Add, b'o');
    /// }
    ///
    /// fn unban_user<P>(ph: PluginHandle<'_, P>, user: &str) {
    ///     // sends `MODE <user> -b`
    ///     ph.send_modes(&[user], Sign::Remove, b'b');
    /// }
    /// ```
    pub fn send_modes(self, targets: &[impl AsRef<str>], sign: mode::Sign, mode_char: u8) {
        let targets: Vec<_> = targets.iter().map(|t| t.as_ref().into_cstr()).collect();
        let mut targets: Vec<*const c_char> = targets.iter().map(|t| t.as_ptr()).collect();
        let ntargets = targets
            .len()
            .try_into()
            .unwrap_or_else(|e| panic!("Too many send_modes targets: {}", e));

        let sign = match sign {
            mode::Sign::Add => b'+',
            mode::Sign::Remove => b'-',
        } as c_char;

        let mode = mode_char as c_char;

        // Safety: handle is always valid
        unsafe {
            ((*self.handle).hexchat_send_modes)(
                self.handle,
                targets.as_mut_ptr(),
                ntargets,
                0,
                sign,
                mode,
            )
        }
    }

    /// Performs a comparison of nicknames or channel names, compliant with RFC1459.
    ///
    /// [RFC1459 says](https://tools.ietf.org/html/rfc1459#section-2.2):
    ///
    /// > Because of IRC's scandanavian origin, the characters {}| are
    /// > considered to be the lower case equivalents of the characters \[\]\\,
    /// > respectively. This is a critical issue when determining the
    /// > equivalence of two nicknames.
    ///
    /// Note that, like other functions taking `&str`, this function will allocate if the provided strings are not already null-terminated.
    /// This may be expensive; if you are calling this function in a loop, consider implementing your own RFC1459 string comparison.
    /// (This function is provided mainly for API completeness.)
    ///
    /// Analogous to [`hexchat_nickcmp`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_nickcmp).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn sort_nicknames<P>(ph: PluginHandle<'_, P>, nicks: &mut [impl AsRef<str>]) {
    ///     nicks.sort_by(|n1, n2| ph.nickcmp(n1.as_ref(), n2.as_ref()));
    /// }
    /// ```
    pub fn nickcmp(self, s1: &str, s2: &str) -> Ordering {
        let s1 = s1.into_cstr();
        let s2 = s2.into_cstr();

        // Safety: handle is always valid
        let ordering =
            unsafe { ((*self.handle).hexchat_nickcmp)(self.handle, s1.as_ptr(), s2.as_ptr()) };

        ordering.cmp(&0)
    }

    /// Strips mIRC colors and/or text attributes (bold, underline, etc.) from a string.
    ///
    /// Analogous to [`hexchat_strip`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_strip).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::strip::{MircColors, TextAttrs};
    ///
    /// fn strip_example<P>(ph: PluginHandle<'_, P>) {
    ///     let orig = "\x0312Blue\x03 \x02Bold!\x02";
    ///
    ///     let strip_all = ph.strip(orig, MircColors::Remove, TextAttrs::Remove);
    ///     assert_eq!(strip_all.unwrap(), "Blue Bold!");
    ///
    ///     let strip_colors = ph.strip(orig, MircColors::Remove, TextAttrs::Keep);
    ///     assert_eq!(strip_colors.unwrap(), "Blue \x02Bold!\x02");
    /// }
    /// ```
    pub fn strip(
        self,
        str: &str,
        mirc: strip::MircColors,
        attrs: strip::TextAttrs,
    ) -> Result<String, ()> {
        self.strip_with(str, mirc, attrs, |s| s.map(ToOwned::to_owned))
    }

    /// Strips mIRC colors and/or text attributes (bold, underline, etc.) from a string.
    ///
    /// Behaves the same as [`PluginHandle::strip`](struct.PluginHandle.html#method.strip),
    /// but avoids allocating a `String` to hold the stripped string.
    ///
    /// Analogous to [`hexchat_strip`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_strip).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::strip::{MircColors, TextAttrs};
    ///
    /// fn strip_example<P>(ph: PluginHandle<'_, P>) {
    ///     let orig = "\x0312Blue\x03 \x02Bold!\x02";
    ///
    ///     ph.strip_with(orig, MircColors::Remove, TextAttrs::Remove, |strip_all| {
    ///         assert_eq!(strip_all, Ok("Blue Bold!"));
    ///     });
    ///
    ///     ph.strip_with(orig, MircColors::Remove, TextAttrs::Keep, |strip_colors| {
    ///         assert_eq!(strip_colors, Ok("Blue \x02Bold!\x02"));
    ///     });
    /// }
    /// ```
    pub fn strip_with<R>(
        self,
        str: &str,
        mirc: strip::MircColors,
        attrs: strip::TextAttrs,
        f: impl FnOnce(Result<&str, ()>) -> R,
    ) -> R {
        let str = str.into_cstr();

        let mirc_flag = match mirc {
            strip::MircColors::Keep => 0,
            strip::MircColors::Remove => 1,
        };
        let attrs_flag = match attrs {
            strip::TextAttrs::Keep => 0,
            strip::TextAttrs::Remove => 1,
        } << 1;
        let flags = mirc_flag | attrs_flag;

        // Safety: handle is always valid
        let stripped_ptr =
            unsafe { ((*self.handle).hexchat_strip)(self.handle, str.as_ptr(), -1, flags) };

        if stripped_ptr.is_null() {
            return f(Err(()));
        }

        // Safety: handle is always valid; stripped_ptr was returned from hexchat_strip
        defer! { unsafe { ((*self.handle).hexchat_free)(self.handle, stripped_ptr as *mut _) } };

        // Safety: hexchat_strip returns a valid pointer or null; temporary does not outlive this function
        let stripped = unsafe { CStr::from_ptr(stripped_ptr) }
            .to_str()
            .unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_strip`: {}", e));

        f(Ok(stripped))
    }
}

/// [Getting Information](https://hexchat.readthedocs.io/en/latest/plugins.html#getting-information)
///
/// Allows you get information about the current [context](struct.PluginHandle.html#impl-3) or HexChat's settings.
impl<'ph, P> PluginHandle<'ph, P> {
    /// Gets information based on the current [context](struct.PluginHandle.html#impl-3).
    ///
    /// See the [`info::types`](info/types/index.html) submodule for a list of info types.
    ///
    /// Analogous to [`hexchat_get_info`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_info).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::info::types::{AwayReason, Channel};
    ///
    /// fn current_channel<P>(ph: PluginHandle<'_, P>) -> String {
    ///     ph.get_info(Channel)
    /// }
    ///
    /// fn current_away_reason<P>(ph: PluginHandle<'_, P>) -> Option<String> {
    ///     ph.get_info(AwayReason)
    /// }
    /// ```
    pub fn get_info<I: Info>(self, info: I) -> <I as Info>::Type {
        self.get_info_with(info, FromInfoValue::from_info_value)
    }

    fn get_info_with<I: Info, R>(
        self,
        info: I,
        // Note: this must be a fn pointer as this api returns a pointer to memory owned by hexchat,
        // which could be invalidated by the closure otherwise (e.g. by interacting with hexchat in basically any way).
        f: fn(Option<&str>) -> R,
    ) -> R {
        let _ = info;

        // Safety: handle is always valid
        let ptr = unsafe { ((*self.handle).hexchat_get_info)(self.handle, I::NAME) };

        if ptr.is_null() {
            return f(None);
        }

        // Safety: pointer returned from hexchat_get_info is null or valid; str does not outlive this function
        let str = unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_get_info`: {}", e));

        f(Some(str))
    }

    /// Gets settings information from HexChat, as available with `/set`.
    ///
    /// See the [`info::prefs`](info/prefs/index.html) submodule for a list of preferences.
    /// See also the [`info::prefs_special`](info/prefs_special/index.html) submodule for a list of special "preferences" that do not show up under `/set`.
    ///
    /// Analogous to [`hexchat_get_prefs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_prefs).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::info::prefs::IrcNick1;
    ///
    /// fn print_nick_setting<P>(ph: PluginHandle<'_, P>) {
    ///     match ph.get_pref(IrcNick1) {
    ///         Ok(nick) => ph.print(&format!("Current nickname setting is: {}\0", nick)),
    ///         Err(()) => ph.print("Failed to get nickname!\0"),
    ///     }
    /// }
    ///
    /// ```
    pub fn get_pref<Pr: Pref>(self, pref: Pr) -> Result<<Pr as Pref>::Type, ()> {
        self.get_pref_value_with(pref, |value| value.and_then(FromPrefValue::from_pref_value))
    }

    fn get_pref_value_with<Pr: Pref, R>(
        self,
        pref: Pr,
        // Note: this must be a fn pointer as this api returns a pointer to memory owned by hexchat,
        // which could be invalidated by the closure otherwise (e.g. by running a /set command).
        f: fn(Result<PrefValue<'_>, ()>) -> R,
    ) -> R {
        let _ = pref;

        let mut string = ptr::null();
        let mut int = 0;

        // Safety: handle is always valid
        let result = unsafe {
            ((*self.handle).hexchat_get_prefs)(self.handle, Pr::NAME, &mut string, &mut int)
        };

        // https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_prefs
        let value = match result {
            1 => {
                assert!(!string.is_null());

                // Safety: hexchat_get_prefs sets a valid string or null, temporary does not outlive this function
                let str = unsafe { CStr::from_ptr(string) }
                    .to_str()
                    .unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_get_prefs`: {}", e));

                PrefValue::Str(str)
            }
            2 => PrefValue::Int(int),
            3 => PrefValue::Bool(int != 0),
            _ => return f(Err(())),
        };

        f(Ok(value))
    }

    /// Gets a list of information, possibly specific to the current [context](struct.PluginHandle.html#impl-3).
    ///
    /// See the [`info::lists`](info/lists/index.html) submodule for a list of lists.
    ///
    /// Analogous to [`hexchat_list_get`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_list_get) and related functions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::context::Context;
    /// use hexavalent::info::lists::{Channels, Users};
    ///
    /// fn print_all_users_in_all_channels<P>(ph: PluginHandle<'_, P>) {
    ///     let channels = match ph.get_list(Channels) {
    ///         Ok(channels) => channels,
    ///         Err(()) => return ph.print("Failed to get channels!\0"),
    ///     };
    ///     for channel in channels {
    ///         let ctxt = match ph.find_context(Context::FullyQualified { servname: channel.servname(), channel: channel.name() }) {
    ///             Some(ctxt) => ctxt,
    ///             None => {
    ///                 ph.print(&format!("Failed to find channel {} in server {}, skipping.\0", channel.name(), channel.servname()));
    ///                 continue;
    ///             }
    ///         };
    ///         ph.print(&format!("Users in {} on {}:\0", channel.name(), channel.servname()));
    ///         let users = ph.with_context(ctxt, || ph.get_list(Users).unwrap_or_default());
    ///         for user in users {
    ///             ph.print(&format!("  {}{}", user.prefix().unwrap_or(' '), user.nick()));
    ///         }
    ///     }
    /// }
    /// ```
    pub fn get_list<L: List>(self, list: L) -> Result<Vec<<L as List>::Elem>, ()> {
        self.get_list_with(
            list,
            #[inline(always)]
            |list| list.map(|l| l.collect()),
        )
    }

    /// Gets a list of information, possibly specific to the current [context](struct.PluginHandle.html#impl-3).
    ///
    /// See the [`info::lists`](info/lists/index.html) submodule for a list of lists.
    ///
    /// Behaves the same as [`PluginHandle::get_list`](struct.PluginHandle.html#method.get_list),
    /// but avoids allocating a `Vec` to hold the list.
    ///
    /// Analogous to [`hexchat_list_get`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_list_get) and related functions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::context::Context;
    /// use hexavalent::info::lists::{Channels, Users};
    ///
    /// fn print_all_users_in_all_channels<P>(ph: PluginHandle<'_, P>) {
    ///     ph.get_list_with(Channels, |channels| {
    ///         let channels = match channels {
    ///             Ok(channels) => channels,
    ///             Err(()) => return ph.print("Failed to get channels!\0"),
    ///         };
    ///         for channel in channels {
    ///             let ctxt = match ph.find_context(Context::FullyQualified { servname: channel.servname(), channel: channel.name() }) {
    ///                 Some(ctxt) => ctxt,
    ///                 None => {
    ///                     ph.print(&format!("Failed to find channel {} in server {}, skipping.\0", channel.name(), channel.servname()));
    ///                     continue;
    ///                 }
    ///             };
    ///             ph.print(&format!("Users in {} on {}:\0", channel.name(), channel.servname()));
    ///             let users = ph.with_context(ctxt, || ph.get_list(Users).unwrap_or_default());
    ///             for user in users {
    ///                 ph.print(&format!("  {}{}", user.prefix().unwrap_or(' '), user.nick()));
    ///             }
    ///         }
    ///     });
    /// }
    /// ```
    pub fn get_list_with<L: List, R>(
        self,
        list: L,
        f: impl FnOnce(Result<&mut dyn Iterator<Item = <L as List>::Elem>, ()>) -> R,
    ) -> R {
        let _ = list;

        // Safety: handle is always valid
        let list_ptr = unsafe { ((*self.handle).hexchat_list_get)(self.handle, L::NAME) };

        if list_ptr.is_null() {
            return f(Err(()));
        }

        // Safety: handle is always valid; list_ptr was returned from hexchat_list_get
        // Safety: ListIter does not outlive this function, so there are no dangling pointers
        defer! { unsafe { ((*self.handle).hexchat_list_free)(self.handle, list_ptr) } }

        struct ListIter<E: FromListElem> {
            handle: *mut hexchat_plugin,
            list: *mut hexchat_list,
            _elem: PhantomData<E>,
        }

        impl<E: FromListElem> Iterator for ListIter<E> {
            type Item = E;

            fn next(&mut self) -> Option<Self::Item> {
                // Safety: handle is always valid; list is valid
                // Safety: hexchat_list_next can safely be called multiple times at the end of a list
                if unsafe { ((*self.handle).hexchat_list_next)(self.handle, self.list) } == 0 {
                    return None;
                }

                // Safety: handle/list are valid for the entire lifetime of this iterator, and hexchat_list_next returned true
                let elem = unsafe { ListElem::new(&self.handle, &self.list) };

                Some(FromListElem::from_list_elem(elem))
            }
        }

        f(Ok(&mut ListIter {
            handle: self.handle,
            list: list_ptr,
            _elem: PhantomData,
        }))
    }
}

/// [Hook Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#hook-functions)
///
/// Hook functions register hook callbacks with HexChat.
/// You can execute code when the user runs a command, when print or server events happen, or on a timer interval.
///
/// # Examples
///
/// The `callback` passed into each hook function is a function pointer (`fn(X) -> Y`)
/// and not a type implementing a function trait (`impl Fn(X) -> Y`), unlike most higher-order functions in Rust.
/// This means that no allocation is required to register a hook, so the plugin cannot leak memory on unload.
/// However, it also means that you cannot capture local variables in hook callbacks.
///
/// For example, the following does not compile, because `count` is captured by the closure.
///
/// ```rust,compile_fail
/// use hexavalent::{Plugin, PluginHandle};
/// use hexavalent::hook::{Eat, HookHandle, Priority};
///
/// struct MyPlugin;
///
/// fn add_counting_command(ph: PluginHandle<'_, MyPlugin>) {
///     let mut count = 0;
///     ph.hook_command(
///         "count\0",
///         "Usage: COUNT, counts the number of times this command was used\0",
///         Priority::Normal,
///         |plugin, ph, words| {
///             count += 1;
///             ph.print(&format!("Called {} time(s)!\0", count));
///             Eat::All
///         }
///     );
/// }
/// ```
///
/// Instead, store state on the plugin struct. Each hook callback gets a shared reference to the plugin.
///
/// Use `Cell` to store simple `Copy` types, as in the following (working) example of a count command.
/// Also use `Cell` when a non-`Copy` type should be moved in and out of the state without mutation,
/// as in [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook)'s example of storing [`HookHandle`](hook/struct.HookHandle.html).
///
/// ```rust
/// use std::cell::Cell;
/// use hexavalent::{Plugin, PluginHandle};
/// use hexavalent::hook::{Eat, HookHandle, Priority};
///
/// struct MyPlugin {
///     count: Cell<u32>,
/// }
///
/// fn add_counting_command(ph: PluginHandle<'_, MyPlugin>) {
///     ph.hook_command(
///         "count\0",
///         "Usage: COUNT, counts the number of times this command was used\0",
///         Priority::Normal,
///         |plugin, ph, words| {
///             plugin.count.set(plugin.count.get() + 1);
///             ph.print(&format!("Called {} time(s)!\0", plugin.count.get()));
///             Eat::All
///         }
///     );
/// }
/// ```
///
/// Use `RefCell` to store values which are mutated while in the state, as in the following example of a map.
///
/// ```rust
/// use std::cell::RefCell;
/// use std::collections::HashMap;
/// use hexavalent::{Plugin, PluginHandle};
/// use hexavalent::hook::{Eat, HookHandle, Priority};
///
/// struct MyPlugin {
///     map: RefCell<HashMap<String, String>>,
/// }
///
/// fn add_map_command(ph: PluginHandle<'_, MyPlugin>) {
///     ph.hook_command("map_set\0", "Usage: MAP_SET <k> <v>\0", Priority::Normal, |plugin, ph, words| {
///         let key = words[1].to_string();
///         let val = words[2].to_string();
///         plugin.map.borrow_mut().insert(key, val);
///         Eat::All
///     });
///     ph.hook_command("map_del\0", "Usage: MAP_DEL <k>\0", Priority::Normal, |plugin, ph, words| {
///         let key = words[1];
///         plugin.map.borrow_mut().remove(key);
///         Eat::All
///     });
///     ph.hook_command("map_get\0", "Usage: MAP_GET <k>\0", Priority::Normal, |plugin, ph, words| {
///         let key = words[1];
///         match plugin.map.borrow().get(key) {
///             Some(val) => ph.print(&format!("map['{}']: '{}'\0", key, val)),
///             None => ph.print(&format!("map['{}']: <not found>\0", key)),
///         }
///         Eat::All
///     });
/// }
/// ```
///
impl<'ph, P: 'static> PluginHandle<'ph, P> {
    /// Registers a command hook with HexChat.
    ///
    /// The command is usable by typing `/command <words...>`.
    /// Command names starting with `.` are hidden in `/help`.
    /// Hooking the special command `""` (empty string) captures non-commands, i.e. input without a `/` at the beginning.
    ///
    /// Each element of `words` is an argument to the command.
    /// `words[0]`  is the name of the command, so `words[1]` is the first user-provided argument.
    /// `words` is limited to 32 elements, and HexChat may provide excess elements, so the length of `words` is not meaningful.
    ///
    /// Note that `callback` is a function pointer and not an `impl Fn()`.
    /// This means that it cannot capture any variables; instead, use `plugin` to store state.
    /// See the [impl header](struct.PluginHandle.html#impl-2) for more details.
    ///
    /// Returns a [`HookHandle`](hook/struct.HookHandle.html) which can be passed to
    /// [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) to unregister the hook.
    ///
    /// Analogous to [`hexchat_hook_command`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_command).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hexavalent::{Plugin, PluginHandle};
    /// use hexavalent::hook::{Eat, HookHandle, Priority};
    ///
    /// struct MyPlugin;
    ///
    /// fn add_greeting_command(ph: PluginHandle<'_, MyPlugin>) {
    ///     ph.hook_command(
    ///         "greet\0",
    ///         "Usage: GREET <name>, prints a greeting locally\0",
    ///         Priority::Normal,
    ///         |plugin, ph, words| {
    ///             ph.print(&format!("Hello {}!\0", words[1]));
    ///             Eat::All
    ///         }
    ///     );
    /// }
    /// ```
    pub fn hook_command(
        self,
        name: &str,
        help_text: &str,
        priority: hook::Priority,
        callback: fn(plugin: &P, ph: PluginHandle<'_, P>, words: &[&str]) -> hook::Eat,
    ) -> HookHandle {
        extern "C" fn hook_command_callback<P: 'static>(
            word: *mut *mut c_char,
            _word_eol: *mut *mut c_char,
            user_data: *mut c_void,
        ) -> c_int {
            catch_and_log_unwind("hook_command_callback", || {
                // Safety: this is exactly the type we pass into user_data below
                let callback: fn(plugin: &P, ph: PluginHandle<'_, P>, words: &[&str]) -> hook::Eat =
                    unsafe { mem::transmute(user_data) };

                // Safety: `word` is a valid word pointer for this entire callback
                let word = unsafe { word_to_iter(&word) };

                let mut words = [""; 32];

                for (i, (ws, w)) in words.iter_mut().zip(word).enumerate() {
                    *ws = w
                        .to_str()
                        .unwrap_or_else(|e| panic!("Invalid UTF8 in field index {}: {}", i, e));
                }

                with_plugin_state(|plugin, ph| callback(plugin, ph, &words))
            })
            .unwrap_or(hook::Eat::None) as c_int
        }

        let name = name.into_cstr();
        let help_text = help_text.into_cstr();

        // Safety: handle is always valid
        let hook = unsafe {
            ((*self.handle).hexchat_hook_command)(
                self.handle,
                name.as_ptr(),
                priority as c_int,
                hook_command_callback::<P>,
                help_text.as_ptr(),
                callback as *mut c_void,
            )
        };

        let hook = NonNull::new(hook)
            .unwrap_or_else(|| panic!("Hook handle was null, should be infallible"));

        // Safety: hook was returned by HexChat; hook is not used after this
        unsafe { HookHandle::new(hook) }
    }

    /// Registers a print event hook with HexChat.
    ///
    /// See the [`events::print`](events/print/index.html) submodule for a list of print events.
    /// See also the [`events::print_special`](events/print_special/index.html) submodule for a list of special hook-only print events.
    ///
    /// Note that `callback` is a function pointer and not an `impl Fn()`.
    /// This means that it cannot capture any variables; instead, use `plugin` to store state.
    /// See the [impl header](struct.PluginHandle.html#impl-2) for more details.
    ///
    /// _Also_ note that passing a closure as `callback` will ICE the compiler,
    /// due to `rustc` [bug #62529](https://github.com/rust-lang/rust/issues/62529).
    /// A `fn` item must be used instead, as in the example below.
    ///
    /// Returns a [`HookHandle`](hook/struct.HookHandle.html) which can be passed to
    /// [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) to unregister the hook.
    ///
    /// Analogous to [`hexchat_hook_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::events::Event;
    /// use hexavalent::events::print::YouPartWithReason;
    /// use hexavalent::hook::{Eat, Priority};
    ///
    /// struct MyPlugin;
    ///
    /// fn hook_you_part(ph: PluginHandle<'_, MyPlugin>) {
    ///     ph.hook_print(YouPartWithReason, Priority::Normal, you_part_cb);
    /// }
    ///
    /// fn you_part_cb(
    ///     plugin: &MyPlugin,
    ///     ph: PluginHandle<'_, MyPlugin>,
    ///     args: <YouPartWithReason as Event<'_>>::Args
    /// ) -> Eat {
    ///     let [your_nick, your_host, channel, reason] = args;
    ///     ph.print(&format!("You left channel {}: {}.", channel, reason));
    ///     Eat::HexChat
    /// }
    /// ```
    pub fn hook_print<E: PrintEvent>(
        self,
        event: E,
        priority: hook::Priority,
        callback: fn(
            plugin: &P,
            ph: PluginHandle<'_, P>,
            args: <E as Event<'_>>::Args,
        ) -> hook::Eat,
    ) -> HookHandle {
        extern "C" fn hook_print_callback<P: 'static, E: PrintEvent>(
            word: *mut *mut c_char,
            user_data: *mut c_void,
        ) -> c_int {
            catch_and_log_unwind("hook_print_callback", || {
                // Safety: this is exactly the type we pass into user_data below
                let callback: fn(
                    plugin: &P,
                    ph: PluginHandle<'_, P>,
                    args: <E as Event<'_>>::Args,
                ) -> hook::Eat = unsafe { mem::transmute(user_data) };

                // Safety: `word` is a valid word pointer for this entire callback
                let word = unsafe { word_to_iter(&word) };
                let args = E::args_from_words(word, iter::empty());

                with_plugin_state(|plugin, ph| callback(plugin, ph, args))
            })
            .unwrap_or(hook::Eat::None) as c_int
        }

        let _ = event;

        // Safety: handle is always valid
        let hook = unsafe {
            ((*self.handle).hexchat_hook_print)(
                self.handle,
                E::NAME,
                priority as c_int,
                hook_print_callback::<P, E>,
                callback as *mut c_void,
            )
        };

        let hook = NonNull::new(hook)
            .unwrap_or_else(|| panic!("Hook handle was null, should be infallible"));

        // Safety: hook was returned by HexChat; hook is not used after this
        unsafe { HookHandle::new(hook) }
    }

    /// Registers a print event hook with HexChat, capturing the event's attributes.
    ///
    /// See the [`events::print`](events/print/index.html) submodule for a list of print events.
    /// See also the [`events::print_special`](events/print_special/index.html) submodule for a list of special hook-only print events.
    ///
    /// Note that `callback` is a function pointer and not an `impl Fn()`.
    /// This means that it cannot capture any variables; instead, use `plugin` to store state.
    /// See the [impl header](struct.PluginHandle.html#impl-2) for more details.
    ///
    /// _Also_ note that passing a closure as `callback` will ICE the compiler,
    /// due to `rustc` [bug #62529](https://github.com/rust-lang/rust/issues/62529).
    /// A `fn` item must be used instead, as in the example below.
    ///
    /// Returns a [`HookHandle`](hook/struct.HookHandle.html) which can be passed to
    /// [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) to unregister the hook.
    ///
    /// Analogous to [`hexchat_hook_print_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_print_attrs).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::events::{Event, EventAttrs};
    /// use hexavalent::events::print::YouPartWithReason;
    /// use hexavalent::hook::{Eat, Priority};
    ///
    /// struct MyPlugin;
    ///
    /// fn hook_you_part(ph: PluginHandle<'_, MyPlugin>) {
    ///     ph.hook_print_attrs(YouPartWithReason, Priority::Normal, you_part_cb);
    /// }
    ///
    /// fn you_part_cb(
    ///     plugin: &MyPlugin,
    ///     ph: PluginHandle<'_, MyPlugin>,
    ///     attrs: EventAttrs,
    ///     args: <YouPartWithReason as Event<'_>>::Args
    /// ) -> Eat {
    ///     let [your_nick, your_host, channel, reason] = args;
    ///     ph.print(&format!("You left channel {} at {}: {}.", channel, attrs.time(), reason));
    ///     Eat::HexChat
    /// }
    /// ```
    pub fn hook_print_attrs<E: PrintEvent>(
        self,
        event: E,
        priority: hook::Priority,
        callback: fn(
            plugin: &P,
            ph: PluginHandle<'_, P>,
            attrs: EventAttrs<'_>,
            args: <E as Event<'_>>::Args,
        ) -> hook::Eat,
    ) -> HookHandle {
        extern "C" fn hook_print_attrs_callback<P: 'static, E: PrintEvent>(
            word: *mut *mut c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut c_void,
        ) -> c_int {
            catch_and_log_unwind("hook_print_attrs_callback", || {
                // Safety: this is exactly the type we pass into user_data below
                let callback: fn(
                    plugin: &P,
                    ph: PluginHandle<'_, P>,
                    attrs: EventAttrs<'_>,
                    args: <E as Event<'_>>::Args,
                ) -> hook::Eat = unsafe { mem::transmute(user_data) };

                // Safety: attrs is a valid hexchat_event_attrs pointer
                let timestamp = unsafe { (*attrs).server_time_utc };
                let attrs = EventAttrs::new(OffsetDateTime::from_unix_timestamp(timestamp));

                // Safety: `word` is a valid word pointer for this entire callback
                let word = unsafe { word_to_iter(&word) };
                let args = E::args_from_words(word, iter::empty());

                with_plugin_state(|plugin, ph| callback(plugin, ph, attrs, args))
            })
            .unwrap_or(hook::Eat::None) as c_int
        }

        let _ = event;

        // Safety: handle is always valid
        let hook = unsafe {
            ((*self.handle).hexchat_hook_print_attrs)(
                self.handle,
                E::NAME,
                priority as c_int,
                hook_print_attrs_callback::<P, E>,
                callback as *mut c_void,
            )
        };

        let hook = NonNull::new(hook)
            .unwrap_or_else(|| panic!("Hook handle was null, should be infallible"));

        // Safety: hook was returned by HexChat; hook is not used after this
        unsafe { HookHandle::new(hook) }
    }

    /// Registers a server event hook with HexChat.
    ///
    /// See the [`events::server`](events/server/index.html) submodule for a list of server events.
    /// See also the [`events::server_special`](events/server_special/index.html) submodule for a list of special server events.
    ///
    /// Note that `callback` is a function pointer and not an `impl Fn()`.
    /// This means that it cannot capture any variables; instead, use `plugin` to store state.
    /// See the [impl header](struct.PluginHandle.html#impl-2) for more details.
    ///
    /// _Also_ note that passing a closure as `callback` will ICE the compiler,
    /// due to `rustc` [bug #62529](https://github.com/rust-lang/rust/issues/62529).
    /// A `fn` item must be used instead, as in the example below.
    ///
    /// Returns a [`HookHandle`](hook/struct.HookHandle.html) which can be passed to
    /// [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) to unregister the hook.
    ///
    /// Analogous to [`hexchat_hook_server`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_server).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::events::{Event, ServerEvent};
    /// use hexavalent::events::server::Part;
    /// use hexavalent::hook::{Eat, Priority};
    ///
    /// struct MyPlugin;
    ///
    /// fn hook_part(ph: PluginHandle<'_, MyPlugin>) {
    ///     ph.hook_server(Part, Priority::Normal, part_cb);
    /// }
    ///
    /// fn part_cb(
    ///     plugin: &MyPlugin,
    ///     ph: PluginHandle<'_, MyPlugin>,
    ///     args: <Part as Event<'_>>::Args
    /// ) -> Eat {
    ///     let [sender, _, channel, reason] = args;
    ///     ph.print(&format!("{} left channel {}: {}.", sender, channel, reason));
    ///     Eat::None
    /// }
    /// ```
    pub fn hook_server<E: ServerEvent>(
        self,
        event: E,
        priority: hook::Priority,
        callback: fn(
            plugin: &P,
            ph: PluginHandle<'_, P>,
            args: <E as Event<'_>>::Args,
        ) -> hook::Eat,
    ) -> HookHandle {
        extern "C" fn hook_server_callback<P: 'static, E: ServerEvent>(
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            user_data: *mut c_void,
        ) -> c_int {
            catch_and_log_unwind("hook_server_callback", || {
                // Safety: this is exactly the type we pass into user_data below
                let callback: fn(
                    plugin: &P,
                    ph: PluginHandle<'_, P>,
                    args: <E as Event<'_>>::Args,
                ) -> hook::Eat = unsafe { mem::transmute(user_data) };

                // Safety: `word` is a valid word pointer for this entire callback
                let word = unsafe { word_to_iter(&word) };
                // Safety: `word_eol` is a valid word pointer for this entire callback
                let word_eol = unsafe { word_to_iter(&word_eol) };
                let args = E::args_from_words(word, word_eol);

                with_plugin_state(|plugin, ph| callback(plugin, ph, args))
            })
            .unwrap_or(hook::Eat::None) as c_int
        }

        let _ = event;

        // Safety: handle is always valid
        let hook = unsafe {
            ((*self.handle).hexchat_hook_server)(
                self.handle,
                E::NAME,
                priority as c_int,
                hook_server_callback::<P, E>,
                callback as *mut c_void,
            )
        };

        let hook = NonNull::new(hook)
            .unwrap_or_else(|| panic!("Hook handle was null, should be infallible"));

        // Safety: hook was returned by HexChat; hook is not used after this
        unsafe { HookHandle::new(hook) }
    }

    /// Registers a server event hook with HexChat, capturing the event's attributes.
    ///
    /// See the [`events::server`](events/server/index.html) submodule for a list of server events.
    /// See also the [`events::server_special`](events/server_special/index.html) submodule for a list of special server events.
    ///
    /// Note that `callback` is a function pointer and not an `impl Fn()`.
    /// This means that it cannot capture any variables; instead, use `plugin` to store state.
    /// See the [impl header](struct.PluginHandle.html#impl-2) for more details.
    ///
    /// _Also_ note that passing a closure as `callback` will ICE the compiler,
    /// due to `rustc` [bug #62529](https://github.com/rust-lang/rust/issues/62529).
    /// A `fn` item must be used instead, as in the example below.
    ///
    /// Returns a [`HookHandle`](hook/struct.HookHandle.html) which can be passed to
    /// [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) to unregister the hook.
    ///
    /// Analogous to [`hexchat_hook_server_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_server_attrs).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::events::{Event, EventAttrs, ServerEvent};
    /// use hexavalent::events::server::Part;
    /// use hexavalent::hook::{Eat, Priority};
    ///
    /// struct MyPlugin;
    ///
    /// fn hook_part(ph: PluginHandle<'_, MyPlugin>) {
    ///     ph.hook_server_attrs(Part, Priority::Normal, part_cb);
    /// }
    ///
    /// fn part_cb(
    ///     plugin: &MyPlugin,
    ///     ph: PluginHandle<'_, MyPlugin>,
    ///     attrs: EventAttrs<'_>,
    ///     args: <Part as Event<'_>>::Args
    /// ) -> Eat {
    ///     let [sender, _, channel, reason] = args;
    ///     ph.print(&format!("{} left channel {} at {}: {}.", sender, channel, attrs.time(), reason));
    ///     Eat::None
    /// }
    /// ```
    pub fn hook_server_attrs<E: ServerEvent>(
        self,
        event: E,
        priority: hook::Priority,
        callback: fn(
            plugin: &P,
            ph: PluginHandle<'_, P>,
            attrs: EventAttrs<'_>,
            args: <E as Event<'_>>::Args,
        ) -> hook::Eat,
    ) -> HookHandle {
        extern "C" fn hook_server_attrs_callback<P: 'static, E: ServerEvent>(
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut c_void,
        ) -> c_int {
            catch_and_log_unwind("hook_server_attrs_callback", || {
                // Safety: this is exactly the type we pass into user_data below
                let callback: fn(
                    plugin: &P,
                    ph: PluginHandle<'_, P>,
                    attrs: EventAttrs<'_>,
                    args: <E as Event<'_>>::Args,
                ) -> hook::Eat = unsafe { mem::transmute(user_data) };

                // Safety: attrs is a valid hexchat_event_attrs pointer
                let timestamp = unsafe { (*attrs).server_time_utc };
                let attrs = EventAttrs::new(OffsetDateTime::from_unix_timestamp(timestamp));

                // Safety: `word` is a valid word pointer for this entire callback
                let word = unsafe { word_to_iter(&word) };
                // Safety: `word_eol` is a valid word pointer for this entire callback
                let word_eol = unsafe { word_to_iter(&word_eol) };
                let args = E::args_from_words(word, word_eol);

                with_plugin_state(|plugin, ph| callback(plugin, ph, attrs, args))
            })
            .unwrap_or(hook::Eat::None) as c_int
        }

        let _ = event;

        // Safety: handle is always valid
        let hook = unsafe {
            ((*self.handle).hexchat_hook_server_attrs)(
                self.handle,
                E::NAME,
                priority as c_int,
                hook_server_attrs_callback::<P, E>,
                callback as *mut c_void,
            )
        };

        let hook = NonNull::new(hook)
            .unwrap_or_else(|| panic!("Hook handle was null, should be infallible"));

        // Safety: hook was returned by HexChat; hook is not used after this
        unsafe { HookHandle::new(hook) }
    }

    /// Registers a timer hook with HexChat.
    ///
    /// `callback` will be called at the interval specified by `timeout`, with a resolution of 1 millisecond.
    ///
    /// Note that `callback` is a function pointer and not an `impl Fn()`.
    /// This means that it cannot capture any variables; instead, use `plugin` to store state.
    /// See the [impl header](struct.PluginHandle.html#impl-2) for more details.
    ///
    /// Returns a [`HookHandle`](hook/struct.HookHandle.html) which can be passed to
    /// [`PluginHandle::unhook`](struct.PluginHandle.html#method.unhook) to unregister the hook.
    ///
    /// Analogous to [`hexchat_hook_timer`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_timer).
    ///
    /// # Panics
    ///
    /// If `timeout` is more than `i32::MAX` milliseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use std::time::Duration;
    /// use hexavalent::{Plugin, PluginHandle};
    /// use hexavalent::hook::{Eat, HookHandle, Priority, Timer};
    ///
    /// struct MyPlugin {
    ///     should_run: Cell<bool>,
    /// }
    ///
    /// fn add_annoying_command(plugin: &MyPlugin, ph: PluginHandle<'_, MyPlugin>) {
    ///     plugin.should_run.set(true);
    ///
    ///     ph.hook_timer(Duration::from_secs(5), |plugin, ph| {
    ///         if plugin.should_run.get() {
    ///             ph.print("Annoying message! Type /stop to stop.\0");
    ///             Timer::Continue
    ///         } else {
    ///             ph.print("This is the last annoying message!\0");
    ///             Timer::Stop
    ///         }
    ///     });
    ///
    ///     ph.hook_command(
    ///         "stop\0",
    ///         "Usage: STOP, stops being annoying\0",
    ///         Priority::Normal,
    ///         |plugin, ph, words| {
    ///             if plugin.should_run.get() {
    ///                 // Instead of using this `Cell<bool>` flag,
    ///                 // it would make more sense to store a `HookHandle`
    ///                 // and call `ph.unhook(hook)` here,
    ///                 // but this demonstrates the use of `Timer::Stop`.
    ///                 plugin.should_run.set(false);
    ///             }
    ///             Eat::All
    ///         }
    ///     );
    /// }
    /// ```
    pub fn hook_timer(
        self,
        timeout: Duration,
        callback: fn(plugin: &P, ph: PluginHandle<'_, P>) -> hook::Timer,
    ) -> HookHandle {
        extern "C" fn hook_timer_callback<P: 'static>(user_data: *mut c_void) -> c_int {
            catch_and_log_unwind("hook_timer_callback", || {
                // Safety: this is exactly the type we pass into user_data below
                let callback: fn(plugin: &P, ph: PluginHandle<'_, P>) -> hook::Timer =
                    unsafe { mem::transmute(user_data) };

                with_plugin_state(|plugin, ph| callback(plugin, ph))
            })
            .unwrap_or(hook::Timer::Stop) as c_int
        }

        let milliseconds = timeout
            .as_millis()
            .try_into()
            .unwrap_or_else(|e| panic!("Timeout duration too long: {}", e));

        // Safety: handle is always valid
        let hook = unsafe {
            ((*self.handle).hexchat_hook_timer)(
                self.handle,
                milliseconds,
                hook_timer_callback::<P>,
                callback as *mut c_void,
            )
        };

        let hook = NonNull::new(hook)
            .unwrap_or_else(|| panic!("Hook handle was null, should be infallible"));

        // Safety: hook was returned by HexChat; hook is not used after this
        unsafe { HookHandle::new(hook) }
    }

    /// Unregisters a hook from HexChat.
    ///
    /// Used with hook registrations functions such as [`PluginHandle::hook_command`](struct.PluginHandle.html#method.hook_command).
    ///
    /// HexChat automatically unhooks any remaining hooks after your plugin finishes unloading,
    /// so this function is only useful if you need to unhook a hook while your plugin is running.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::cell::Cell;
    /// use hexavalent::{Plugin, PluginHandle};
    /// use hexavalent::hook::{Eat, HookHandle, Priority};
    ///
    /// #[derive(Default)]
    /// struct MyPlugin {
    ///     cmd_handle: Cell<Option<HookHandle>>,
    /// }
    ///
    /// impl Plugin for MyPlugin {
    ///     fn init(&self, ph: PluginHandle<'_, Self>) {
    ///         let hook = ph.hook_command(
    ///             "thisCommandOnlyWorksOnce\0",
    ///             "Usage: THISCOMMANDONLYWORKSONCE <args...>, this command only works once\0",
    ///             Priority::Normal,
    ///             |plugin, ph, words| {
    ///                 ph.print(&format!("You'll only see this once: {}\0", words.join("|")));
    ///                 if let Some(hook) = plugin.cmd_handle.take() {
    ///                     ph.unhook(hook);
    ///                 }
    ///                 Eat::All
    ///             }
    ///         );
    ///         self.cmd_handle.set(Some(hook));
    ///     }
    /// }
    /// ```
    pub fn unhook(self, hook: HookHandle) {
        let hook = hook.into_raw();

        // Safety: handle is always valid; hook is valid due to HookHandle invariant
        let _ = unsafe { ((*self.handle).hexchat_unhook)(self.handle, hook.as_ptr()) };
    }
}

/// [Context Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#context-functions)
///
/// Allows you to work with server/channel contexts.
///
/// It is not always necessary to change context, as hook callbacks usually execute in a context related to the event.
/// For example:
/// - [`PluginHandle::hook_command`](struct.PluginHandle.html#method.hook_command) callbacks run in the context where the command was executed.
/// - [`PluginHandle::hook_print`](struct.PluginHandle.html#method.hook_print) callbacks run in the context where the print event was emitted.
/// - [`PluginHandle::hook_server`](struct.PluginHandle.html#method.hook_server) callbacks run in the server (but not channel) context where the server event was received.
impl<'ph, P> PluginHandle<'ph, P> {
    /// Finds a server/channel context based on various criteria.
    ///
    /// See [`Context`](context/enum.Context.html) for available criteria.
    /// These include: the currently-focused tab, a specified channel, or the frontmost tab in a server.
    ///
    /// Returns a [`ContextHandle`](context/struct.ContextHandle.html) which can be passed to
    /// [`PluginHandle::with_context`](struct.PluginHandle.html#method.with_context) to enter the context.
    ///
    /// Analogous to [`hexchat_find_context`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_find_context).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::context::Context;
    ///
    /// fn find_context_example<P>(ph: PluginHandle<'_, P>) {
    ///     if let Some(ctxt) = ph.find_context(Context::Focused) {
    ///         ph.with_context(ctxt, || ph.print("This tab is focused!\0"));
    ///     }
    ///     if let Some(ctxt) = ph.find_context(Context::Nearby { channel: "#help\0" }) {
    ///         ph.with_context(ctxt, || ph.print("This tab is #help!\0"));
    ///     }
    ///     if let Some(ctxt) = ph.find_context(Context::Frontmost { servname: "Snoonet\0" }) {
    ///         ph.with_context(ctxt, || ph.print("This tab is frontmost on snoonet!\0"));
    ///     }
    /// }
    /// ```
    pub fn find_context(self, find: Context<'_>) -> Option<ContextHandle<'ph>> {
        let (servname, channel) = match find {
            Context::Focused => (None, None),
            Context::Nearby { channel } => (None, Some(channel.into_cstr())),
            Context::Frontmost { servname } => (Some(servname.into_cstr()), None),
            Context::FullyQualified { servname, channel } => {
                (Some(servname.into_cstr()), Some(channel.into_cstr()))
            }
        };

        let servname = servname.as_ref().map_or_else(ptr::null, |s| s.as_ptr());
        let channel = channel.as_ref().map_or_else(ptr::null, |c| c.as_ptr());

        // Safety: handle is always valid
        let context =
            unsafe { ((*self.handle).hexchat_find_context)(self.handle, servname, channel) };

        // Safety: context is either a valid hexchat_context pointer or null
        NonNull::new(context).map(|c| unsafe { ContextHandle::new(c) })
    }

    /// Executes a function in a different server/channel context.
    ///
    /// Used with [`PluginHandle::find_context`](struct.PluginHandle.html#method.find_context).
    ///
    /// Analogous to [`hexchat_get_context`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_context) and
    /// [`hexchat_set_context`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_set_context).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::context::Context;
    ///
    /// fn send_message_to_channel<P>(
    ///     ph: PluginHandle<'_, P>,
    ///     channel: &str,
    ///     message: &str,
    /// ) -> Result<(), ()> {
    ///     let ctxt = match ph.find_context(Context::Nearby { channel }) {
    ///         Some(ctxt) => ctxt,
    ///         None => return Err(()),
    ///     };
    ///     ph.with_context(ctxt, || {
    ///         ph.print(message);
    ///         Ok(())
    ///     })
    /// }
    /// ```
    pub fn with_context<R>(self, context: ContextHandle<'_>, f: impl FnOnce() -> R) -> R {
        // Safety: handle is always valid
        let old_context = unsafe { ((*self.handle).hexchat_get_context)(self.handle) };

        // Safety: handle is always valid; context contains a valid pointer
        int_to_result(unsafe {
            ((*self.handle).hexchat_set_context)(self.handle, context.as_ptr().as_ptr())
        })
        // this should be infallible, since the lifetime on ContextHandle prevents it from being stored,
        // and it should not be invalidated while our code is running
        .unwrap_or_else(|_| panic!("Channel invalidated while plugin running"));

        // Safety: handle is always valid; old_context is a valid pointer
        defer! {
            int_to_result(unsafe { ((*self.handle).hexchat_set_context)(self.handle, old_context) })
                .unwrap_or_else(|_| panic!("Failed to switch back to original context"))
        };

        f()
    }
}

/// [Plugin Preferences](https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-preferences)
///
/// Allows you to get and set preferences associated with your plugin.
impl<'ph, P> PluginHandle<'ph, P> {
    /// Sets a plugin-specific string preference.
    ///
    /// Fails if `value` exceeds 511 bytes in length.
    ///
    /// Analogous to [`hexchat_pluginpref_set_str`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_set_str).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn save_str<P>(ph: PluginHandle<'_, P>) -> Result<(), ()> {
    ///     ph.pluginpref_set_str("myvar1\0", "something important\0")
    /// }
    /// ```
    pub fn pluginpref_set_str(self, name: &str, value: &str) -> Result<(), ()> {
        let name = name.into_cstr();
        let value = value.into_cstr();

        // Undocumented limit of 512 characters
        // https://github.com/hexchat/hexchat/blob/57478b65758e6b697b1d82ce21075e74aa475efc/src/common/plugin.c#L1950
        // https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_list
        if value.to_bytes_with_nul().len() > 512 {
            return Err(());
        }

        // Safety: handle is always valid
        int_to_result(unsafe {
            ((*self.handle).hexchat_pluginpref_set_str)(self.handle, name.as_ptr(), value.as_ptr())
        })
    }

    /// Gets a plugin-specific string preference.
    ///
    /// Note that int preferences can be successfully loaded as strings.
    ///
    /// Analogous to [`hexchat_pluginpref_get_str`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_get_str).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn load_str<P>(ph: PluginHandle<'_, P>) {
    ///     let pref = ph.pluginpref_get_str("myvar1\0");
    ///     assert_eq!(pref.unwrap(), "something important");
    /// }
    /// ```
    pub fn pluginpref_get_str(self, name: &str) -> Result<String, ()> {
        self.pluginpref_get_str_with(name, |pref| pref.map(ToOwned::to_owned))
    }

    /// Gets a plugin-specific string preference, passing the result to a closure.
    ///
    /// Note that int preferences can be successfully loaded as strings.
    ///
    /// Behaves the same as [`PluginHandle::pluginpref_get_str`](struct.PluginHandle.html#method.pluginpref_get_str),
    /// but avoids allocating a `String` to hold the preference value.
    ///
    /// Analogous to [`hexchat_pluginpref_get_str`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_get_str).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn load_str<P>(ph: PluginHandle<'_, P>) {
    ///     ph.pluginpref_get_str_with("myvar1\0", |pref| {
    ///         assert_eq!(pref, Ok("something important"));
    ///     });
    /// }
    /// ```
    pub fn pluginpref_get_str_with<R>(
        self,
        name: &str,
        f: impl FnOnce(Result<&str, ()>) -> R,
    ) -> R {
        let name = name.into_cstr();

        // Undocumented limit of 512 characters
        // https://github.com/hexchat/hexchat/blob/57478b65758e6b697b1d82ce21075e74aa475efc/src/common/plugin.c#L1950
        // https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_list
        let mut buf = [0; 512];

        // Safety: handle is always valid
        // (Un)Safety: no length argument, better hope they never change the 512 max length
        let res = int_to_result(unsafe {
            ((*self.handle).hexchat_pluginpref_get_str)(
                self.handle,
                name.as_ptr(),
                buf.as_mut_ptr(),
            )
        });

        if let Err(e) = res {
            return f(Err(e));
        }

        *buf.last_mut().unwrap() = 0;
        // Safety: buf is definitely null-terminated; temporary does not outlive buf
        let str = unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_str()
            .unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_pluginpref_get_str`: {}", e));

        f(Ok(str))
    }

    /// Sets a plugin-specific int preference.
    ///
    /// `-1` is a reserved value and cannot be used.
    ///
    /// Analogous to [`hexchat_pluginpref_set_int`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_set_int).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn save_int<P>(ph: PluginHandle<'_, P>) -> Result<(), ()> {
    ///     ph.pluginpref_set_int("answer\0", 42)
    /// }
    /// ```
    pub fn pluginpref_set_int(self, name: &str, value: i32) -> Result<(), ()> {
        let name = name.into_cstr();

        // Safety: handle is always valid
        int_to_result(unsafe {
            ((*self.handle).hexchat_pluginpref_set_int)(self.handle, name.as_ptr(), value)
        })
    }

    /// Gets a plugin-specific int preference.
    ///
    /// Analogous to [`hexchat_pluginpref_get_int`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_get_int).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn load_int<P>(ph: PluginHandle<'_, P>) {
    ///     let pref = ph.pluginpref_get_int("answer\0");
    ///     assert_eq!(pref, Ok(42));
    /// }
    /// ```
    pub fn pluginpref_get_int(self, name: &str) -> Result<i32, ()> {
        let name = name.into_cstr();

        // Safety: handle is always valid
        let value =
            unsafe { ((*self.handle).hexchat_pluginpref_get_int)(self.handle, name.as_ptr()) };

        match value {
            -1 => Err(()),
            _ => Ok(value),
        }
    }

    /// Deletes a plugin-specific preference.
    ///
    /// Returns `Ok(())` both when an existing preference is deleted and when no preference with `name` exists.
    ///
    /// Analogous to [`hexchat_pluginpref_delete`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_delete).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn remove_answer<P>(ph: PluginHandle<'_, P>) -> Result<(), ()> {
    ///     ph.pluginpref_delete("answer\0")
    /// }
    /// ```
    pub fn pluginpref_delete(self, name: &str) -> Result<(), ()> {
        let name = name.into_cstr();

        // Safety: handle is always valid
        int_to_result(unsafe {
            ((*self.handle).hexchat_pluginpref_delete)(self.handle, name.as_ptr())
        })
    }

    /// Lists the names of all plugin-specific preferences.
    ///
    /// Note that the total length of all preference names is limited to about 4095 bytes.
    ///
    /// Analogous to [`hexchat_pluginpref_list`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_list).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn print_all_prefs<P>(ph: PluginHandle<'_, P>) {
    ///     let prefs = match ph.pluginpref_list() {
    ///         Ok(prefs) => prefs,
    ///         Err(()) => return ph.print("Failed to list plugin preferences!\0"),
    ///     };
    ///     ph.print("All plugin preferences:\0");
    ///     for pref in prefs {
    ///         let val = ph.pluginpref_get_str(&pref);
    ///         let val = match &val {
    ///             Ok(v) => v,
    ///             Err(()) => "<not found>",
    ///         };
    ///         ph.print(&format!("{} = {}\0", pref, val));
    ///     }
    /// }
    /// ```
    pub fn pluginpref_list(self) -> Result<Vec<String>, ()> {
        self.pluginpref_list_with(
            #[inline(always)]
            |prefs| prefs.map(|p| p.map(ToOwned::to_owned).collect()),
        )
    }

    /// Lists the names of all plugin-specific preferences, passing the result to a closure.
    ///
    /// Note that the total length of all preference names is limited to about 4095 bytes.
    ///
    /// Behaves the same as [`PluginHandle::pluginpref_list`](struct.PluginHandle.html#method.pluginpref_list),
    /// but avoids allocating a `Vec` and `String`s to hold each preference name.
    ///
    /// Analogous to [`hexchat_pluginpref_list`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_list).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn print_all_prefs<P>(ph: PluginHandle<'_, P>) {
    ///     ph.pluginpref_list_with(|prefs| {
    ///         let prefs = match prefs {
    ///             Ok(prefs) => prefs,
    ///             Err(()) => return ph.print("Failed to list plugin preferences!\0"),
    ///         };
    ///         ph.print("All plugin preferences:\0");
    ///         for pref in prefs {
    ///             ph.pluginpref_get_str_with(pref, |val| {
    ///                 let val = val.unwrap_or("<not found>");
    ///                 ph.print(&format!("{} = {}\0", pref, val));
    ///             });
    ///         }
    ///     });
    /// }
    /// ```
    pub fn pluginpref_list_with<R>(
        self,
        f: impl FnOnce(Result<&mut dyn Iterator<Item = &str>, ()>) -> R,
    ) -> R {
        // Documented limit of 4096 characters
        // https://github.com/hexchat/hexchat/blob/57478b65758e6b697b1d82ce21075e74aa475efc/src/common/plugin.c#L2016
        // https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_pluginpref_list
        let mut buf = [0; 4096];

        // Safety: handle is always valid
        let res = int_to_result(unsafe {
            ((*self.handle).hexchat_pluginpref_list)(self.handle, buf.as_mut_ptr())
        });

        if let Err(e) = res {
            return f(Err(e));
        }

        *buf.last_mut().unwrap() = 0;
        // Safety: buf is definitely null-terminated; str does not outlive buf
        let str = unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_str()
            .unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_pluginpref_list`: {}", e));

        let str = str.trim_end_matches(',');

        match str {
            "" => f(Ok(&mut iter::empty())),
            _ => f(Ok(&mut str.split(','))),
        }
    }
}

/// [Plugin GUI](https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-gui)
///
/// Allows you to add and remove fake plugins from the plugin GUI.
impl<'ph, P> PluginHandle<'ph, P> {
    /// Adds a fake plugin to the plugin GUI.
    ///
    /// Only useful if your plugin loads other plugins.
    /// Do not call this function with the same arguments you pass to [`export_plugin`](macro.export_plugin.html).
    ///
    /// Returns a [`FakePluginHandle`](gui/struct.FakePluginHandle.html) which can be passed to
    /// [`PluginHandle::plugingui_remove`](struct.PluginHandle.html#method.plugingui_remove) to remove the fake plugin.
    ///
    /// Analogous to [`hexchat_plugingui_add`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_plugingui_add).
    pub fn plugingui_add(
        self,
        filename: &str,
        name: &str,
        desc: &str,
        version: &str,
    ) -> FakePluginHandle {
        let filename = filename.into_cstr();
        let name = name.into_cstr();
        let desc = desc.into_cstr();
        let version = version.into_cstr();

        // Safety: handle is always valid
        let gui = unsafe {
            ((*self.handle).hexchat_plugingui_add)(
                self.handle,
                filename.as_ptr(),
                name.as_ptr(),
                desc.as_ptr(),
                version.as_ptr(),
                ptr::null_mut(),
            )
        };

        let gui = NonNull::new(gui)
            .unwrap_or_else(|| panic!("GUI handle was null, should be infallible"));

        // Safety: gui was returned by HexChat; gui is not used after this
        unsafe { FakePluginHandle::new(gui) }
    }

    /// Removes a fake plugin from the plugin GUI.
    ///
    /// Used with [`PluginHandle::plugingui_add`](struct.PluginHandle.html#method.plugingui_add).
    ///
    /// Analogous to [`hexchat_plugingui_remove`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_plugingui_remove).
    pub fn plugingui_remove(self, gui: FakePluginHandle) {
        let gui = gui.into_raw();

        // Safety: handle is always valid; hook is valid due to HookHandle invariant
        unsafe { ((*self.handle).hexchat_plugingui_remove)(self.handle, gui.as_ptr()) };
    }
}

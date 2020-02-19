use std::cmp::Ordering;
use std::convert::TryInto;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;
use std::time::UNIX_EPOCH;

use libc::time_t;

use crate::ffi::{hexchat_plugin, int_to_result, StrExt};
use crate::mode;
use crate::print::{EventAttrs, PrintEvent};
use crate::strip;

/// Must be implemented by all HexChat plugins.
///
/// Plugins must also implement `Default`, although it is not a superclass due to object safety restrictions.
///
/// # Examples
///
/// TODO add example when more stuff works
///  printing statistics would be good here
pub trait Plugin: 'static {
    /// Initialize your plugin.
    ///
    /// Use this method to perform any work that should be done when your plugin is loaded,
    /// such as registering hooks or printing startup messages.
    ///
    /// Analogous to [`hexchat_plugin_init`](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::{Plugin, PluginHandle};
    ///
    /// struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn init(&self, ph: PluginHandle<'_>) {
    ///         ph.print("Plugin loaded successfully!\0");
    ///     }
    /// }
    /// ```
    fn init(&self, ph: PluginHandle<'_>);

    /// Deinitialize your plugin.
    ///
    /// Use this method to perform any work that should be done when your plugin is unloaded,
    /// such as printing shutdown messages or statistics.
    ///
    /// You do not need to explicitly [`unhook`](struct.PluginHandle.html#method.unhook) any hooks in this method, as remaining hooks are
    /// automatically removed by HexChat when your plugin finishes unloading.
    ///
    /// Analogous to [`hexchat_plugin_deinit`](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::{Plugin, PluginHandle};
    ///
    /// struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn init(&self, _: PluginHandle<'_>) {}
    ///
    ///     fn deinit(&self, ph: PluginHandle<'_>) {
    ///         ph.print("Plugin unloading...\0");
    ///     }
    /// }
    /// ```
    fn deinit(&self, ph: PluginHandle<'_>) {
        let _ = ph;
    }
}

/// Interacts with HexChat's plugin API.
///
/// Cannot be constructed in user code, but is passed into
/// [`Plugin::init`](trait.Plugin.html#tymethod.init),
/// [`Plugin::deinit`](trait.Plugin.html#method.deinit),
/// and hook callbacks such as [`hook_command`](struct.PluginHandle.html#method.hook_command).
///
/// Most of HexChat's [functions](https://hexchat.readthedocs.io/en/latest/plugins.html#functions) are available as struct methods,
/// without the `hexchat_` prefix.
///
/// # Examples
///
/// All functions which take `&str`/`impl AsRef<str>` arguments will allocate if the string is not null-terminated,
/// and panic if the string contains interior nulls.
///
/// ```rust
/// # fn print_some_stuff(ph: hexavalent::PluginHandle<'_>) {
/// // for example, this would not allocate
/// ph.print("hello\0");
/// // ...this would allocate
/// ph.print("hello");
/// // ...and this would panic
/// ph.print("hel\0lo");
/// # }
/// ```
///
/// TODO add basic hook example
#[derive(Copy, Clone)]
pub struct PluginHandle<'ph> {
    /// Always points to a valid instance of `hexchat_plugin`.
    handle: *mut hexchat_plugin,
    _lifetime: PhantomData<&'ph hexchat_plugin>,
}

impl<'ph> PluginHandle<'ph> {
    /// Creates a new `PluginHandle` from a native `hexchat_plugin`.
    ///
    /// # Safety
    ///
    /// `plugin_handle` must point to a valid instance of `hexchat_plugin`.
    pub(crate) unsafe fn new(plugin_handle: *mut hexchat_plugin) -> Self {
        Self {
            handle: plugin_handle,
            _lifetime: PhantomData,
        }
    }
}

/// [General Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#general-functions)
impl<'ph> PluginHandle<'ph> {
    /// Prints text to the current tab. Text may contain mIRC color codes and formatting.
    ///
    /// Analogous to [`hexchat_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn say_hello(ph: PluginHandle<'_>) {
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

    /// Executes a command as if it were typed into HexChat's input box after a `/`.
    ///
    /// Analogous to [`hexchat_command`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_command).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn op_user(ph: PluginHandle<'_>, username: &str) {
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

    /// Emits a print event.
    ///
    /// If you do not know the print event's type statically, use [`emit_print_dyn`](struct.PluginHandle.html#method.emit_print_dyn).
    ///
    /// Analogous to [`hexchat_emit_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::print::events::ChannelMessage;
    ///
    /// fn print_fake_message(ph: PluginHandle<'_>, user: &str, text: &str) -> Result<(), ()> {
    ///     ph.emit_print(ChannelMessage, [user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print<'a, E: PrintEvent<'a>>(
        self,
        event: E,
        args: <E as PrintEvent<'a>>::Args,
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

    /// Emits a print event, with dynamic type.
    ///
    /// Prefer [`emit_print`](struct.PluginHandle.html#method.emit_print) if you know the print event's type statically.
    ///
    /// Analogous to [`hexchat_emit_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print).
    ///
    /// # Panics
    ///
    /// If `args` contains more than 4 elements. (No text event takes more than 4 arguments.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn print_fake_message(ph: PluginHandle<'_>, user: &str, text: &str) -> Result<(), ()> {
    ///     ph.emit_print_dyn("Channel Message\0", &[user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print_dyn(self, event: &str, args: &[impl AsRef<str>]) -> Result<(), ()> {
        assert!(
            args.len() <= 4,
            "passed {} args to text event {}, but no text event takes more than 4 args",
            args.len(),
            event
        );

        let event = event.into_cstr();
        let args = [
            args.get(0).map(|s| s.as_ref().into_cstr()),
            args.get(1).map(|s| s.as_ref().into_cstr()),
            args.get(2).map(|s| s.as_ref().into_cstr()),
            args.get(3).map(|s| s.as_ref().into_cstr()),
        ];
        let args: [*const c_char; 4] = [
            args[0].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            args[1].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            args[2].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            args[3].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
        ];

        // Safety: `handle` is always valid
        int_to_result(unsafe {
            ((*self.handle).hexchat_emit_print)(
                self.handle,
                event.as_ptr(),
                args[0],
                args[1],
                args[2],
                args[3],
                ptr::null::<c_char>(),
            )
        })
    }

    /// Emits a print event, specifying its attributes.
    ///
    /// If you do not know the print event's type statically, use [`emit_print_attrs_dyn`](struct.PluginHandle.html#method.emit_print_attrs_dyn).
    ///
    /// Analogous to [`hexchat_emit_print_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print_attrs).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::print::EventAttrs;
    /// use hexavalent::print::events::ChannelMessage;
    ///
    /// fn print_fake_message_like_its_1979(ph: PluginHandle<'_>, user: &str, text: &str) -> Result<(), ()> {
    ///     let attrs = EventAttrs::new(std::time::UNIX_EPOCH + std::time::Duration::from_secs(86400 * 365 * 9));
    ///     ph.emit_print_attrs(ChannelMessage, attrs, [user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print_attrs<'a, E: PrintEvent<'a>>(
        self,
        event: E,
        attrs: EventAttrs<'_>,
        args: <E as PrintEvent<'a>>::Args,
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

            let since_unix_epoch = attrs
                .time()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|e| panic!("Invalid date in event attrs: {}", e))
                .as_secs() as time_t;

            // Safety: `handle` is always valid
            int_to_result(unsafe {
                let event_attrs = ((*self.handle).hexchat_event_attrs_create)(self.handle);
                defer! { ((*self.handle).hexchat_event_attrs_free)(self.handle, event_attrs) };

                ptr::write(
                    &mut (*event_attrs).server_time_utc as *mut _,
                    since_unix_epoch,
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

    /// Emits a print event, specifying its attributes, with dynamic type.
    ///
    /// Prefer [`emit_print_attrs`](struct.PluginHandle.html#method.emit_print_attrs) if you know the print event's type statically.
    ///
    /// Analogous to [`hexchat_emit_print_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print_attrs).
    ///
    /// # Panics
    ///
    /// If `args` contains more than 4 elements. (No text event takes more than 4 arguments.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::print::EventAttrs;
    ///
    /// fn print_fake_message_like_its_1979(ph: PluginHandle<'_>, user: &str, text: &str) -> Result<(), ()> {
    ///     let attrs = EventAttrs::new(std::time::UNIX_EPOCH + std::time::Duration::from_secs(86400 * 365 * 9));
    ///     ph.emit_print_attrs_dyn("Channel Message\0", attrs, &[user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print_attrs_dyn(
        self,
        event: &str,
        attrs: EventAttrs<'_>,
        args: &[impl AsRef<str>],
    ) -> Result<(), ()> {
        assert!(
            args.len() <= 4,
            "passed {} args to text event {}, but no text event takes more than 4 args",
            args.len(),
            event
        );

        let event = event.into_cstr();
        let args = [
            args.get(0).map(|s| s.as_ref().into_cstr()),
            args.get(1).map(|s| s.as_ref().into_cstr()),
            args.get(2).map(|s| s.as_ref().into_cstr()),
            args.get(3).map(|s| s.as_ref().into_cstr()),
        ];
        let args: [*const c_char; 4] = [
            args[0].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            args[1].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            args[2].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            args[3].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
        ];

        let since_unix_epoch = attrs
            .time()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("Invalid date in event attrs: {}", e))
            .as_secs() as time_t;

        // Safety: `handle` is always valid
        int_to_result(unsafe {
            let event_attrs = ((*self.handle).hexchat_event_attrs_create)(self.handle);
            defer! { ((*self.handle).hexchat_event_attrs_free)(self.handle, event_attrs) };

            ptr::write(
                &mut (*event_attrs).server_time_utc as *mut _,
                since_unix_epoch,
            );

            ((*self.handle).hexchat_emit_print_attrs)(
                self.handle,
                event_attrs,
                event.as_ptr(),
                args[0],
                args[1],
                args[2],
                args[3],
                ptr::null::<c_char>(),
            )
        })
    }

    /// Sends channel mode changes to targets in the current channel.
    ///
    /// Analogous to [`hexchat_send_modes`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_send_modes).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::mode::Sign;
    ///
    /// fn op_users(ph: PluginHandle<'_>, users: &[&str]) {
    ///     // sends `MODE <users> +o`
    ///     ph.send_modes(users, Sign::Add, b'o');
    /// }
    ///
    /// fn unban_user(ph: PluginHandle<'_>, user: &str) {
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
    /// fn sort_nicknames(ph: PluginHandle<'_>, nicks: &mut [impl AsRef<str>]) {
    ///     nicks.sort_by(|n1, n2| ph.nickcmp(n1.as_ref(), n2.as_ref()));
    /// }
    /// ```
    pub fn nickcmp(self, s1: &str, s2: &str) -> Ordering {
        let s1 = s1.into_cstr();
        let s2 = s2.into_cstr();

        // Safety: handle is always valid
        let ordering =
            unsafe { ((*self.handle).hexchat_nickcmp)(self.handle, s1.as_ptr(), s2.as_ptr()) };

        if ordering < 0 {
            Ordering::Less
        } else if ordering > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
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
    /// fn strip_example(ph: PluginHandle<'_>) {
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
        let str = str.into_cstr();

        let mirc_flag = match mirc {
            strip::MircColors::Keep => 0,
            strip::MircColors::Remove => 1,
        } << 0;
        let attrs_flag = match attrs {
            strip::TextAttrs::Keep => 0,
            strip::TextAttrs::Remove => 1,
        } << 1;
        let flags = mirc_flag | attrs_flag;

        // Safety: handle is always valid
        let stripped_ptr =
            unsafe { ((*self.handle).hexchat_strip)(self.handle, str.as_ptr(), -1, flags) };

        if stripped_ptr.is_null() {
            return Err(());
        }

        // Safety: handle is always valid; stripped_ptr was returned from hexchat_strip
        defer! { unsafe { ((*self.handle).hexchat_free)(self.handle, stripped_ptr as *mut _) } };

        // Safety: hexchat_strip returns a valid pointer or null; temporary is immediately copied to an owned string
        let stripped = unsafe { CStr::from_ptr(stripped_ptr).to_str().map(|s| s.to_owned()) };

        Ok(stripped.unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_strip`: {}", e)))
    }
}

/// [Getting Information](https://hexchat.readthedocs.io/en/latest/plugins.html#getting-information)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_get_info,
        hexchat_get_prefs,
        hexchat_list_get,
        hexchat_list_fields,
        hexchat_list_next,
        hexchat_list_str,
        hexchat_list_int,
        hexchat_list_time,
        hexchat_list_free,
    */
}

/// [Hook Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#hook-functions)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_hook_command,
        hexchat_hook_fd,
        hexchat_hook_print,
        hexchat_hook_print_attrs,
        hexchat_hook_server,
        hexchat_hook_server_attrs,
        hexchat_hook_timer,
        hexchat_unhook,
    */
}

/// [Context Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#context-functions)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_find_context,
        hexchat_get_context,
        hexchat_set_context,
    */
}

/// [Plugin Preferences](https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-preferences)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_pluginpref_set_str,
        hexchat_pluginpref_get_str,
        hexchat_pluginpref_set_int,
        hexchat_pluginpref_get_int,
        hexchat_pluginpref_delete,
        hexchat_pluginpref_list,
    */
}

/// [Plugin GUI](https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-gui)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_plugingui_add,
        hexchat_plugingui_remove,
    */
}

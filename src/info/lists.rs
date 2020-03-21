use std::net::{Ipv4Addr, SocketAddrV4};
use std::num::NonZeroU64;

use bitflags::bitflags;
use time::OffsetDateTime;

list!(
    Channels,
    "channels",
    "List of channels, queries and their servers.",
    "A channel.",
    Channel {
        ["channel", "Channel or query name.", string] name: String,
        ["channelkey", "Channel key. (HexChat 2.9.6+)", string] key: Option<String>,
        ["chanmodes", "Available channel modes e.g. `\"beI,k,l\"`. (HexChat 2.12.2+)", string] modes: String,
        ["chantypes", "Available channel types e.g. `\"#!&\"`.", string] types: String,
        ["flags", "Info flags.", int] flags: ChannelFlags,
        ["id", "Unique server ID.", int] server_id: i32,
        ["lag", "Lag in milliseconds.", int] lag_ms: i32,
        ["maxmodes", "Maximum modes per line.", int] max_modes_per_line: u32,
        ["network", "Name of network.", string] network: String,
        ["nickprefixes", "Nickname prefixes e.g. `\"@+\"`.", string] nick_prefixes: String,
        ["nickmodes", "Nickname mode chars e.g. `\"ov\"`.", string] nick_modes: String,
        ["queue", "Number of bytes in the send-queue.", int] queue: u32,
        ["server", "Server name to which this channel belongs.", string] servname: String,
        ["type", "Channel type.", int] ty: ChannelType,
        ["users", "Number of users in this channel.", int] num_users: u32,
    }
);

bitflags! {
    /// Flags related to channel state.
    ///
    /// Part of [`Channel`](struct.Channel.html).
    pub struct ChannelFlags: i32 {
        /// The client is connected to the channel.
        const CONNECTED = 1;
        /// The client is connecting to the channel.
        const CONNECTING = 2;
        /// The current user is marked away.
        const MARKED_AWAY = 4;
        /// The MOTD has ended.
        const END_OF_MOTD = 8;
        /// The channel supports Undernet's `WHOX` features.
        const HAS_WHOX = 16;
        /// The channel supports Freenode's `IDENTIFY-MSG`.
        const HAS_IDMSG = 32;
        /// Join/part events are hidden.
        const HIDE_JOIN_PARTS = 64;
        /// `HIDE_JOIN_PARTS` has the default value (i.e. was not set explicitly).
        const HIDE_JOIN_PARTS_UNSET = 128;
        /// Messages beep.
        const BEEP_ON_MESSAGE = 256;
        /// Messages blink the tray icon.
        const BLINK_TRAY = 512;
        /// Messages blink the taskbar icon.
        const BLINK_TASKBAR = 1024;
        /// Messages are logged.
        const LOGGING = 2048;
        /// `LOGGING` has the default value (i.e. was not set explicitly).
        const LOGGING_UNSET = 4096;
        /// Scrollback is enabled.
        const SCROLLBACK = 8192;
        /// `SCROLLBACK` has the default value (i.e. was not set explicitly).
        const SCROLLBACK_UNSET = 16384;
        /// Colors are stripped from messages.
        const STRIP_COLORS = 32768;
        /// `STRIP_COLORS` has the default value (i.e. was not set explicitly).
        const STRIP_COLORS_UNSET = 65536;
    }
}

impl super::FromListElemField<i32> for ChannelFlags {
    fn from_list_elem_field(field: i32) -> Self {
        Self::from_bits_truncate(field)
    }
}

/// The type of a channel.
///
/// Part of [`Channel`](struct.Channel.html).
#[non_exhaustive]
pub enum ChannelType {
    /// The toplevel server "channel".
    Server = 1,
    /// A normal channel.
    Channel = 2,
    /// A dialog (direct message) channel.
    Dialog = 3,
    /// A notice channel.
    Notice = 4,
    /// A server notice channel.
    ServerNotice = 5,
}

impl super::FromListElemField<i32> for ChannelType {
    fn from_list_elem_field(field: i32) -> Self {
        match () {
            _ if field == Self::Server as _ => Self::Server,
            _ if field == Self::Channel as _ => Self::Channel,
            _ if field == Self::Dialog as _ => Self::Dialog,
            _ if field == Self::Notice as _ => Self::Notice,
            _ if field == Self::ServerNotice as _ => Self::ServerNotice,
            _ => panic!("Unexpected channel type: {}", field),
        }
    }
}

list!(
    DccTransfers,
    "dcc",
    "List of DCC file transfers.",
    "A DCC file transfer.",
    DccTransfer {
        [
            custom,
            "Socket of the remote user.",
            |elem| SocketAddrV4::new(Ipv4Addr::from(elem.int("address32\0") as u32), elem.int("port\0") as u16)
        ] socket_addr: SocketAddrV4,
        ["cps", "Bytes per second (speed).", int] bytes_per_second: u32,
        ["destfile", "Destination full pathname.", string] dest_file: String,
        ["file", "Filename.", string] file_name: String,
        ["nick", "Nickname of person who the file is from/to.", string] nick: String,
        [
            custom,
            "Bytes sent/received.",
            |elem| (elem.int("poshigh\0") as u64) << 32 | (elem.int("pos\0") as u64)
        ] position: u64,
        [
            custom,
            "Point at which this file was resumed.",
            |elem| NonZeroU64::new((elem.int("resumehigh\0") as u64) << 32 | (elem.int("resume\0") as u64))
        ] resumed_at: Option<NonZeroU64>,
        [
            custom,
            "File size in bytes.",
            |elem| (elem.int("sizehigh\0") as u64) << 32 | (elem.int("size\0") as u64)
        ] size: u64
    }
);

list!(
    Ignores,
    "ignore",
    "List of ignores.",
    "An ignored mask.",
    Ignore {
        ["mask", "Ignore mask, e.g. `\"*!*@*.aol.com\"`.", string] mask: String,
        ["flags", "Info flags.", int] flags: IgnoreFlags,
    }
);

bitflags! {
    /// Flags related to ignore state.
    ///
    /// Part of [`Ignore`](struct.Ignore.html).
    pub struct IgnoreFlags: i32 {
        /// Private messages are ignored.
        const PRIVATE = 1 << 0;
        /// Notice messages are ignored.
        const NOTICE = 1 << 1;
        /// Channel messages are ignored.
        const CHANNEL = 1 << 2;
        /// CTCP commands are ignored.
        const CTCP = 1 << 3;
        /// Invitations are ignored.
        const INVITE = 1 << 4;
        /// This is an "unignore" entry.
        const UNIGNORE = 1 << 5;
        /// This ignore entry is temporary.
        const NO_SAVE = 1 << 6;
        /// DCC transfers are ignored.
        const DCC = 1 << 7;
    }
}

impl super::FromListElemField<i32> for IgnoreFlags {
    fn from_list_elem_field(field: i32) -> Self {
        Self::from_bits_truncate(field)
    }
}

list!(
    Notifies,
    "notify",
    "List of people on notify in the current server [context](../../struct.PluginHandle.html#impl-3).",
    "A nick on notify.",
    Notify {
        [
            custom,
            "Networks to which this nick applies.",
            |elem| elem.string("networks\0").map_or_else(Vec::new, |s| s.split(',').map(ToOwned::to_owned).collect())
        ] networks: Vec<String>,
        ["nick", "Nickname.", string] nick: String,
        ["flags", "Info flags.", int] flags: NotifyFlags,
        ["on", "Time when user came online.", time] online: OffsetDateTime,
        ["off", "Time when user went offline.", time] offline: OffsetDateTime,
        ["seen", "Time when user the user was last verified still online.", time] seen: OffsetDateTime,
    }
);

bitflags! {
    /// Flags related to notify state.
    ///
    /// Part of [`Notify`](struct.Notify.html).
    pub struct NotifyFlags: i32 {
        /// The nick is online.
        const IS_ONLINE = 1 << 0;
    }
}

impl super::FromListElemField<i32> for NotifyFlags {
    fn from_list_elem_field(field: i32) -> Self {
        Self::from_bits_truncate(field)
    }
}

list!(
    Users,
    "users",
    "List of users in the current [context](../../struct.PluginHandle.html#impl-3).",
    "A user.",
    User {
        ["account", "Account name. (HexChat 2.9.6+)", string] account: Option<String>,
        ["away", "Away status.", int] is_away: bool,
        ["lasttalk", "Last time the user was seen talking.", time] last_talk: OffsetDateTime,
        ["nick", "Nickname.", string] nick: String,
        ["host", "Hostname e.g. `\"user@host\"`.", string] host: Option<String>,
        ["prefix", "Prefix character e.g. `'@'` or `'+'`.", string] prefix: Option<char>,
        ["realname", "Realname.", string] realname: Option<String>,
        ["selected", "Selected status in the user list, only works in the focused tab.", int] is_selected: bool,
    }
);

use crate::str::HexString;

info!(
    AwayReason,
    "away", Option::<HexString>, "Your current away reason."
);
info!(Channel, "channel", HexString, "Current channel name.");
info!(
    Hostname,
    "host", HexString, "Real hostname of the server you are connected to."
);
info!(
    Modes,
    "modes", Option::<HexString>, "Channel modes, if known."
);
info!(
    Network,
    "network", Option::<HexString>, "Current network name."
);
info!(Nick, "nick", HexString, "Your current nickname.");
info!(
    NickservPassword,
    "nickserv", Option::<HexString>, "Nickserv password for this network."
);
info!(
    Server,
    "server", Option::<HexString>, "Current server name (what the server claims to be)."
);
info!(
    Topic,
    "topic", Option::<HexString>, "Current channel topic."
);

// less useful ones
info!(
    Inputbox,
    "inputbox", HexString, "Input-box contents, what the user has typed."
);
info!(Version, "version", HexString, "HexChat version number.");
info!(
    WinStatus,
    "win_status", HexString, "Window status: \"active\", \"hidden\" or \"normal\"."
);

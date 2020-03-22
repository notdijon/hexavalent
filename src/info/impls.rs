info!(
    AwayReason,
    "away", Option::<String>, "Your current away reason."
);
info!(Channel, "channel", String, "Current channel name.");
info!(
    Hostname,
    "host", String, "Real hostname of the server you are connected to."
);
info!(Modes, "modes", Option::<String>, "Channel modes, if known.");
info!(
    Network,
    "network", Option::<String>, "Current network name."
);
info!(Nick, "nick", String, "Your current nickname.");
info!(
    NickservPassword,
    "nickserv", Option::<String>, "Nickserv password for this network."
);
info!(
    Server,
    "server", Option::<String>, "Current server name (what the server claims to be)."
);
info!(Topic, "topic", Option::<String>, "Current channel topic.");

// less useful ones
info!(
    Inputbox,
    "inputbox", String, "Input-box contents, what the user has typed."
);
info!(Version, "version", String, "HexChat version number.");
info!(
    WinStatus,
    "win_status", String, "Window status: \"active\", \"hidden\" or \"normal\"."
);

server_event!(
    RawLine,
    "RAW LINE",
    "Every line that comes from the IRC server.",
    0: "Sender",
    1: "Command",
    eol 2: "Arguments"
);

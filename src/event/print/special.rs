print_event!(
    OpenContext,
    "Open Context",
    "Called when a new hexchat_context is created.",
);
print_event!(
    CloseContext,
    "Close Context",
    "Called when a hexchat_context pointer is closed.",
);
print_event!(
    FocusTab,
    "Focus Tab",
    "Called when a tab is brought to front.",
);
print_event!(
    FocusWindow,
    "Focus Window",
    "Called a toplevel window is focused, or the main tab-window is focused by the window manager.",
);
print_event!(DccChatText, "DCC Chat Text", "Called when some text from a DCC Chat arrives.", 0: "Address", 1: "Port", 2: "Nick", 3: "The Message");
print_event!(KeyPress, "Key Press", "Called when some keys are pressed in the input box.", 0: "Key Value", 1: "State Bitfield (shift, capslock, alt)", 2: "String version of the key", 3: "Length of the string (may be 0 for unprintable keys)");

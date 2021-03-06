print_event!(AddNotify, "Add Notify", "`%C18*%O$t%C18$1%O added to notify list.`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(BanList, "Ban List", "`%C22*%O$t%C22$1%O: %C18$2%O on %C24$4%O by %C26$3%O`", 0: "Channel", 1: "Banmask", 2: "Who set the ban", 3: "Ban time");
print_event!(Banned, "Banned", "`%C22*%O$tCannot join %C22$1 %O(%C20You are banned%O).`", 0: "Channel Name");
print_event!(Beep, "Beep", "``",);
print_event!(CapabilityAcknowledgement, "Capability Acknowledgement", "`%C29*%O$tCapabilities acknowledged: %C29$2%O`", 0: "Server Name", 1: "Acknowledged Capabilities");
print_event!(CapabilityDeleted, "Capability Deleted", "`%C29*%O$tCapabilities removed: %C29$2%O`", 0: "Server Name", 1: "Removed Capabilities");
print_event!(CapabilityList, "Capability List", "`%C23*%O$tCapabilities supported: %C29$2%O`", 0: "Server Name", 1: "Server Capabilities");
print_event!(CapabilityRequest, "Capability Request", "`%C23*%O$tCapabilities requested: %C29$1%O`", 0: "Requested Capabilities");
print_event!(ChangeNick, "Change Nick", "`%C24*%O$t%C28$1%O is now known as %C18$2%O`", 0: "Old nickname", 1: "New nickname");
print_event!(ChannelAction, "Channel Action", "`%C18*$t%B$1%O $2`", 0: "Nickname", 1: "The action", 2: "Mode char", 3: "Identified text");
print_event!(ChannelActionHilight, "Channel Action Hilight", "`%C19*$t%B$1%B $2%O`", 0: "Nickname", 1: "The action", 2: "Mode char", 3: "Identified text");
print_event!(ChannelBan, "Channel Ban", "`%C22*%O$t%C26$1%O sets ban on %C18$2%O`", 0: "The nick of the person who did the banning", 1: "The ban mask");
print_event!(ChannelCreation, "Channel Creation", "`%C22*%O$tChannel %C22$1%O created on %C24$2%O`", 0: "The channel", 1: "The time");
print_event!(ChannelDehalfop, "Channel DeHalfOp", "`%C22*%O$t%C26$1%O removes channel half-operator status from %C18$2%O`", 0: "The nick of the person who did the dehalfop'ing", 1: "The nick of the person who has been dehalfop'ed");
print_event!(ChannelDeop, "Channel DeOp", "`%C22*%O$t%C26$1%O removes channel operator status from %C18$2%O`", 0: "The nick of the person who did the deop'ing", 1: "The nick of the person who has been deop'ed");
print_event!(ChannelDevoice, "Channel DeVoice", "`%C22*%O$t%C26$1%O removes voice from %C18$2%O`", 0: "The nick of the person who did the devoice'ing", 1: "The nick of the person who has been devoice'ed");
print_event!(ChannelExempt, "Channel Exempt", "`%C22*%O$t%C26$1%C sets exempt on %C18$2%O`", 0: "The nick of the person who did the exempt", 1: "The exempt mask");
print_event!(ChannelHalfOperator, "Channel Half-Operator", "`%C22*%O$t%C26$1%O gives channel half-operator status to %C18$2%O`", 0: "The nick of the person who has been halfop'ed", 1: "The nick of the person who did the halfop'ing");
print_event!(ChannelInvite, "Channel INVITE", "`%C22*%O$t%C26$1%C sets invite exempt on %C18$2%O`", 0: "The nick of the person who did the invite", 1: "The invite mask");
print_event!(
    ChannelList,
    "Channel List",
    "`%UChannel          Users   Topic`",
);
print_event!(ChannelMessage, "Channel Message", "`%C18%H<%H$4$1%C18%H>%H%O$t$2`", 0: "Nickname", 1: "The text", 2: "Mode char", 3: "Identified text");
print_event!(ChannelModeGeneric, "Channel Mode Generic", "`%C22*%O$t%C26$1%O sets mode %C24$2$3%O on %C22$4%O`", 0: "The nick of the person setting the mode", 1: "The mode's sign (+/-)", 2: "The mode letter", 3: "The channel it's being set on");
print_event!(ChannelModes, "Channel Modes", "`%C22*%O$tChannel %C22$1%O modes: %C24$2`", 0: "Channel Name", 1: "Modes string");
print_event!(ChannelMsgHilight, "Channel Msg Hilight", "`%C19%H<%H$4%B$1%B%H>%H$t$2%O`", 0: "Nickname", 1: "The text", 2: "Mode char", 3: "Identified text");
print_event!(ChannelNotice, "Channel Notice", "`-%C18$1%C/%C22$2%C-$t$3%O`", 0: "Who it's from", 1: "The Channel it's going to", 2: "The message");
print_event!(ChannelOperator, "Channel Operator", "`%C22*%O$t%C26$1%O gives channel operator status to %C18$2%O`", 0: "The nick of the person who did the op'ing", 1: "The nick of the person who has been op'ed");
print_event!(ChannelQuiet, "Channel Quiet", "`%C22*%O$t%C26$1%O sets quiet on %C18$2%O`", 0: "The nick of the person who did the quieting", 1: "The quiet mask");
print_event!(ChannelRemoveExempt, "Channel Remove Exempt", "`%C22*%O$t%C26$1%O removes exempt on %C18$2%O`", 0: "The nick of the person removed the exempt", 1: "The exempt mask");
print_event!(ChannelRemoveInvite, "Channel Remove Invite", "`%C22*%O$t%C26$1%O removes invite exempt on %C18$2%O`", 0: "The nick of the person removed the invite", 1: "The invite mask");
print_event!(ChannelRemoveKeyword, "Channel Remove Keyword", "`%C22*%O$t%C26$1%O removes channel keyword`", 0: "The nick who removed the key");
print_event!(ChannelRemoveLimit, "Channel Remove Limit", "`%C22*%O$t%C26$1%O removes user limit`", 0: "The nick who removed the limit");
print_event!(ChannelSetKey, "Channel Set Key", "`%C22*%O$t%C26$1%O sets channel keyword to %C24$2%O`", 0: "The nick of the person who set the key", 1: "The key");
print_event!(ChannelSetLimit, "Channel Set Limit", "`%C22*%O$t%C26$1%O sets channel limit to %C24$2%O`", 0: "The nick of the person who set the limit", 1: "The limit");
print_event!(ChannelUnban, "Channel UnBan", "`%C22*%O$t%C26$1%O removes ban on %C18$2%O`", 0: "The nick of the person who did the unban'ing", 1: "The ban mask");
print_event!(ChannelUnquiet, "Channel UnQuiet", "`%C22*%O$t%C26$1%O removes quiet on %C18$2%O`", 0: "The nick of the person who did the unquiet'ing", 1: "The quiet mask");
print_event!(ChannelUrl, "Channel Url", "`%C22*%O$tChannel %C22$1%O url: %C24$2`", 0: "Channel Name", 1: "URL");
print_event!(ChannelVoice, "Channel Voice", "`%C22*%O$t%C26$1%O gives voice to %C18$2%O`", 0: "The nick of the person who did the voice'ing", 1: "The nick of the person who has been voice'ed");
print_event!(
    Connected,
    "Connected",
    "`%C23*%O$tConnected. Now logging in.`",
);
print_event!(Connecting, "Connecting", "`%C23*%O$tConnecting to %C29$1%C (%C23$2:$3%O)`", 0: "Host", 1: "IP", 2: "Port");
print_event!(ConnectionFailed, "Connection Failed", "`%C20*%O$tConnection failed (%C20$1%O)`", 0: "Error");
print_event!(CtcpGeneric, "CTCP Generic", "`%C24*%O$tReceived a CTCP %C24$1%C from %C18$2%O`", 0: "The CTCP event", 1: "The nick of the person");
print_event!(CtcpGenericToChannel, "CTCP Generic to Channel", "`%C24*%C$tReceived a CTCP %C24$1%C from %C18$2%C (to %C22$3%C)%O`", 0: "The CTCP event", 1: "The nick of the person", 2: "The Channel it's going to");
print_event!(CtcpSend, "CTCP Send", "`>%C18$1%C<$tCTCP %C24$2%O`", 0: "Receiver", 1: "Message");
print_event!(CtcpSound, "CTCP Sound", "`%C24*%O$tReceived a CTCP Sound %C24$1%C from %C18$2%O`", 0: "The sound", 1: "The nick of the person", 2: "The channel");
print_event!(CtcpSoundToChannel, "CTCP Sound to Channel", "`%C24*%O$tReceived a CTCP Sound %C24$1%C from %C18$2%C (to %C22$3%O)`", 0: "The sound", 1: "The nick of the person", 2: "The channel");
print_event!(DccChatAbort, "DCC CHAT Abort", "`%C23*%O$tDCC CHAT to %C18$1%O aborted.`", 0: "Nickname");
print_event!(DccChatConnect, "DCC CHAT Connect", "`%C24*%O$tDCC CHAT connection established to %C18$1%C %C30[%C24$2%C30]%O`", 0: "Nickname", 1: "IP address");
print_event!(DccChatFailed, "DCC CHAT Failed", "`%C20*%O$tDCC CHAT to %C18$1%O lost (%C20$4%O)`", 0: "Nickname", 1: "IP address", 2: "Port", 3: "Error");
print_event!(DccChatOffer, "DCC CHAT Offer", "`%C24*%O$tReceived a DCC CHAT offer from %C18$1%O`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(DccChatOffering, "DCC CHAT Offering", "`%C24*%O$tOffering DCC CHAT to %C18$1%O`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(DccChatReoffer, "DCC CHAT Reoffer", "`%C24*%O$tAlready offering CHAT to %C18$1%O`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(DccConectionFailed, "DCC Conection Failed", "`%C20*%O$tDCC $1 connect attempt to %C18$2%O failed (%C20$3%O)`", 0: "DCC Type", 1: "Nickname", 2: "Error");
print_event!(DccGenericOffer, "DCC Generic Offer", "`%C23*%O$tReceived '%C23$1%C' from %C18$2%O`", 0: "DCC String", 1: "Nickname");
print_event!(
    DccHeader,
    "DCC Header",
    "`%C16,17 Type  To/From    Status  Size    Pos     File`",
);
print_event!(DccMalformed, "DCC Malformed", "`%C20*%O$tReceived a malformed DCC request from %C18$1%O.$a010%C23*%O$tContents of packet: %C23$2%O`", 0: "Nickname", 1: "The Packet");
print_event!(DccOffer, "DCC Offer", "`%C24*%O$tOffering '%C24$1%O' to %C18$2%O`", 0: "Filename", 1: "Nickname", 2: "Pathname");
print_event!(
    DccOfferNotValid,
    "DCC Offer Not Valid",
    "`%C23*%O$tNo such DCC offer.`",
);
print_event!(DccRecvAbort, "DCC RECV Abort", "`%C23*%O$tDCC RECV '%C23$2%O' to %C18$1%O aborted.`", 0: "Nickname", 1: "Filename");
print_event!(DccRecvComplete, "DCC RECV Complete", "`%C24*%O$tDCC RECV '%C23$1%O' from %C18$3%O complete %C30[%C24$4%O cps%C30]%O`", 0: "Filename", 1: "Destination filename", 2: "Nickname", 3: "CPS");
print_event!(DccRecvConnect, "DCC RECV Connect", "`%C24*%O$tDCC RECV connection established to %C18$1 %C30[%O%C24$2%C30]%O`", 0: "Nickname", 1: "IP address", 2: "Filename");
print_event!(DccRecvFailed, "DCC RECV Failed", "`%C20*%O$tDCC RECV '%C23$1%O' from %C18$3%O failed (%C20$4%O)`", 0: "Filename", 1: "Destination filename", 2: "Nickname", 3: "Error");
print_event!(DccRecvFileOpenError, "DCC RECV File Open Error", "`%C20*%O$tDCC RECV: Cannot open '%C23$1%C' for writing (%C20$2%O)`", 0: "Filename", 1: "Error");
print_event!(DccRename, "DCC Rename", "`%C23*%O$tThe file '%C24$1%C' already exists, saving it as '%C23$2%O' instead.`", 0: "Old Filename", 1: "New Filename");
print_event!(DccResumeRequest, "DCC RESUME Request", "`%C24*%O$t%C18$1%C has requested to resume '%C23$2%C' from %C24$3%O.`", 0: "Nickname", 1: "Filename", 2: "Position");
print_event!(DccSendAbort, "DCC SEND Abort", "`%C23*%O$tDCC SEND '%C23$2%C' to %C18$1%O aborted.`", 0: "Nickname", 1: "Filename");
print_event!(DccSendComplete, "DCC SEND Complete", "`%C24*%O$tDCC SEND '%C23$1%C' to %C18$2%C complete %C30[%C24$3%C cps%C30]%O`", 0: "Filename", 1: "Nickname", 2: "CPS");
print_event!(DccSendConnect, "DCC SEND Connect", "`%C24*%O$tDCC SEND connection established to %C18$1 %C30[%O%C24$2%C30]%O`", 0: "Nickname", 1: "IP address", 2: "Filename");
print_event!(DccSendFailed, "DCC SEND Failed", "`%C20*%O$tDCC SEND '%C23$1%C' to %C18$2%C failed (%C20$3%O)`", 0: "Filename", 1: "Nickname", 2: "Error");
print_event!(DccSendOffer, "DCC SEND Offer", "`%C24*%O$t%C18$1%C has offered '%C23$2%C' (%C24$3%O bytes)`", 0: "Nickname", 1: "Filename", 2: "Size", 3: "IP address");
print_event!(DccStall, "DCC Stall", "`%C20*%O$tDCC $1 '%C23$2%C' to %C18$3%O stalled, aborting.`", 0: "DCC Type", 1: "Filename", 2: "Nickname");
print_event!(DccTimeout, "DCC Timeout", "`%C20*%O$tDCC $1 '%C23$2%C' to %C18$3%O timed out, aborting.`", 0: "DCC Type", 1: "Filename", 2: "Nickname");
print_event!(DeleteNotify, "Delete Notify", "`%C24*%O$t%C18$1%O deleted from notify list.`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(Disconnected, "Disconnected", "`%C20*%O$tDisconnected (%C20$1%O)`", 0: "Error");
print_event!(FoundIp, "Found IP", "`%C24*%O$tFound your IP: %C30[%C24$1%C30]%O`", 0: "IP");
print_event!(GenericMessage, "Generic Message", "`$1$t$2`", 0: "Left message", 1: "Right message");
print_event!(IgnoreAdd, "Ignore Add", "`%O%C18$1%O added to ignore list.`", 0: "Hostmask");
print_event!(IgnoreChanged, "Ignore Changed", "`%OIgnore on %C18$1%O changed.`", 0: "Hostmask");
print_event!(IgnoreFooter, "Ignore Footer", "`%C16,17`",);
print_event!(
    IgnoreHeader,
    "Ignore Header",
    "`%C16,17 Hostmask                  PRIV NOTI CHAN CTCP DCC  INVI UNIG`",
);
print_event!(IgnoreRemove, "Ignore Remove", "`%O%C18$1%O removed from ignore list.`", 0: "Hostmask");
print_event!(
    IgnorelistEmpty,
    "Ignorelist Empty",
    "`%OIgnore list is empty.`",
);
print_event!(Invite, "Invite", "`%C20*%O$tCannot join %C22$1%C (%C20Channel is invite only%O)`", 0: "Channel Name");
print_event!(Invited, "Invited", "`%C24*%O$tYou have been invited to %C22$1%O by %C18$2%O (%C29$3%O)`", 0: "Channel Name", 1: "Nick of person who invited you", 2: "Server Name");
print_event!(Join, "Join", "`%C23*$t$1 ($3%C23) has joined`", 0: "The nick of the joining person", 1: "The channel being joined", 2: "The host of the person", 3: "The account of the person");
print_event!(Keyword, "Keyword", "`%C20*%O$tCannot join %C22$1%C (%C20Requires keyword%O)`", 0: "Channel Name");
print_event!(Kick, "Kick", "`%C22*%O$t%C26$1%C has kicked %C18$2%C from %C22$3%C (%C24$4%O)`", 0: "The nickname of the kicker", 1: "The person being kicked", 2: "The channel", 3: "The reason");
print_event!(Killed, "Killed", "`%C19*%O$t%C19You have been killed by %C26$1%C (%C20$2%O)`", 0: "Nickname", 1: "Reason");
print_event!(MessageSend, "Message Send", "`%O>%C18$1%C<%O$t$2`", 0: "Receiver", 1: "Message");
print_event!(Motd, "Motd", "`%C29*%O$t%C29$1%O`", 0: "Text", 1: "Server Name", 2: "Raw Numeric or Identifier");
print_event!(MotdSkipped, "MOTD Skipped", "`%C29*%O$t%C29MOTD Skipped%O`",);
print_event!(NickClash, "Nick Clash", "`%C23*%O$t%C28$1%C is already in use. Retrying with %C18$2%O...`", 0: "Nickname in use", 1: "Nick being tried");
print_event!(NickErroneous, "Nick Erroneous", "`%C23*%O$t%C28$1%C is erroneous. Retrying with %C18$2%O...`", 0: "Nickname in use", 1: "Nick being tried");
print_event!(
    NickFailed,
    "Nick Failed",
    "`%C20*%O$tNickname is erroneous or already in use. Use /NICK to try another.`",
);
print_event!(NoDcc, "No DCC", "`%C20*%O$tNo such DCC.`",);
print_event!(
    NoRunningProcess,
    "No Running Process",
    "`%C23*%O$tNo process is currently running`",
);
print_event!(Notice, "Notice", "`%O-%C18$1%O-$t$2`", 0: "Who it's from", 1: "The message");
print_event!(NoticeSend, "Notice Send", "`%O->%C18$1%O<-$t$2`", 0: "Receiver", 1: "Message");
print_event!(NotifyAway, "Notify Away", "`%C23*%O$tNotify: %C18$1%C is away (%C24$2%O)`", 0: "Nickname", 1: "Away Reason");
print_event!(NotifyBack, "Notify Back", "`%C23*%O$tNotify: %C18$1%C is back`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(NotifyEmpty, "Notify Empty", "`$tNotify list is empty.`",);
print_event!(NotifyHeader, "Notify Header", "`%C16,17  Notify List`",);
print_event!(NotifyNumber, "Notify Number", "`%C23*%O$t%C23$1%O users in notify list.`", 0: "Number of notify items");
print_event!(NotifyOffline, "Notify Offline", "`%C23*%O$tNotify: %C18$1%C is offline (%C29$3%O)`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(NotifyOnline, "Notify Online", "`%C23*%O$tNotify: %C18$1%C is online (%C29$3%O)`", 0: "Nickname", 1: "Server Name", 2: "Network");
print_event!(OpenDialog, "Open Dialog", "``",);
print_event!(Part, "Part", "`%C24*$t$1 ($2%C24) has left`", 0: "The nick of the person leaving", 1: "The host of the person", 2: "The channel");
print_event!(PartWithReason, "Part with Reason", "`%C24*$t$1 ($2%C24) has left ($4)`", 0: "The nick of the person leaving", 1: "The host of the person", 2: "The channel", 3: "The reason");
print_event!(PingReply, "Ping Reply", "`%C24*%O$tPing reply from %C18$1%C: %C24$2%O second(s)`", 0: "Who it's from", 1: "The time in x.x format (see below)");
print_event!(PingTimeout, "Ping Timeout", "`%C20*%O$tNo ping reply for %C24$1%O seconds, disconnecting.`", 0: "Seconds");
print_event!(PrivateAction, "Private Action", "`%C18**$t$3$1%O $2 %C18**`", 0: "Nickname", 1: "The message", 2: "Identified text");
print_event!(PrivateActionToDialog, "Private Action to Dialog", "`%C18*$t$3$1%O $2`", 0: "Nickname", 1: "The message", 2: "Identified text");
print_event!(PrivateMessage, "Private Message", "`%C18*%C18$3$1*%O$t$2`", 0: "Nickname", 1: "The message", 2: "Identified text");
print_event!(PrivateMessageToDialog, "Private Message to Dialog", "`%C18%H<%H$3$1%H>%H%O$t$2`", 0: "Nickname", 1: "The message", 2: "Identified text");
print_event!(
    ProcessAlreadyRunning,
    "Process Already Running",
    "`%C24*%O$tA process is already running`",
);
print_event!(Quit, "Quit", "`%C24*$t$1 has quit ($2)`", 0: "Nick", 1: "Reason", 2: "Host");
print_event!(RawModes, "Raw Modes", "`%C24*%O$t%C26$1%C sets modes %C30[%C24$2%C30]%O`", 0: "Nickname", 1: "Modes string");
print_event!(ReceiveWallops, "Receive Wallops", "`%O-%C29$1/Wallops%O-$t$2`", 0: "Nickname", 1: "The message", 2: "Identified text");
print_event!(ResolvingUser, "Resolving User", "`%C24*%O$tLooking up IP number for %C18$1%O...`", 0: "Nickname", 1: "Hostname");
print_event!(SaslAuthenticating, "SASL Authenticating", "`%C23*%O$tAuthenticating via SASL as %C18$1%O (%C24$2%O)`", 0: "Username", 1: "Mechanism");
print_event!(SaslResponse, "SASL Response", "`%C29*%O$t$4`", 0: "Server Name", 1: "Raw Numeric or Identifier", 2: "Username", 3: "Message");
print_event!(ServerConnected, "Server Connected", "`%C29*%O$tConnected.`",);
print_event!(ServerError, "Server Error", "`%C29*%O$t%C20$1%O`", 0: "Text");
print_event!(ServerLookup, "Server Lookup", "`%C29*%O$tLooking up %C29$1%O`", 0: "Server Name");
print_event!(ServerNotice, "Server Notice", "`%C29*%O$t$1`", 0: "Text", 1: "Server Name", 2: "Raw Numeric or Identifier");
print_event!(ServerText, "Server Text", "`%C29*%O$t$1`", 0: "Text", 1: "Server Name", 2: "Raw Numeric or Identifier");
print_event!(SslMessage, "SSL Message", "`%C29*%O$t$1`", 0: "Text", 1: "Server Name");
print_event!(StopConnection, "Stop Connection", "`%C23*%O$tStopped previous connection attempt (%C24$1%O)`", 0: "PID");
print_event!(Topic, "Topic", "`%C22*%O$tTopic for %C22$1%C is: $2%O`", 0: "Channel", 1: "Topic");
print_event!(TopicChange, "Topic Change", "`%C22*%O$t%C26$1%C has changed the topic to: $2%O`", 0: "Nick of person who changed the topic", 1: "Topic", 2: "Channel");
print_event!(TopicCreation, "Topic Creation", "`%C22*%O$tTopic for %C22$1%C set by %C26$2%C (%C24$3%O)`", 0: "The channel", 1: "The creator", 2: "The time");
print_event!(
    UnknownHost,
    "Unknown Host",
    "`%C20*%O$tUnknown host. Maybe you misspelled it?`",
);
print_event!(UserLimit, "User Limit", "`%C20*%O$tCannot join %C22$1%C (%C20User limit reached%O)`", 0: "Channel Name");
print_event!(UsersOnChannel, "Users On Channel", "`%C22*%O$tUsers on %C22$1%C: %C24$2%O`", 0: "Channel Name", 1: "Users");
print_event!(WhoisAuthenticated, "WhoIs Authenticated", "`%C23*%O$t%C28[%C18$1%C28]%O $2 %C18$3%O`", 0: "Nickname", 1: "Message", 2: "Account");
print_event!(WhoisAwayLine, "WhoIs Away Line", "`%C23*%O$t%C28[%C18$1%C28]%C is away %C30(%C23$2%O%C30)%O`", 0: "Nickname", 1: "Away reason");
print_event!(WhoisChannelOperLine, "WhoIs Channel/Oper Line", "`%C23*%O$t%C28[%C18$1%C28]%O $2`", 0: "Nickname", 1: "Channel Membership/\"is an IRC operator\"");
print_event!(WhoisEnd, "WhoIs End", "`%C23*%O$t%C28[%C18$1%C28] %OEnd of WHOIS list.`", 0: "Nickname");
print_event!(WhoisIdentified, "WhoIs Identified", "`%C23*%O$t%C28[%C18$1%C28]%O $2`", 0: "Nickname", 1: "Message", 2: "Numeric");
print_event!(WhoisIdleLine, "WhoIs Idle Line", "`%C23*%O$t%C28[%C18$1%C28]%O idle %C23$2%O`", 0: "Nickname", 1: "Idle time");
print_event!(WhoisIdleLineWithSignon, "WhoIs Idle Line with Signon", "`%C23*%O$t%C28[%C18$1%C28]%O idle %C23$2%O, signon: %C23$3%O`", 0: "Nickname", 1: "Idle time", 2: "Signon time");
print_event!(WhoisNameLine, "WhoIs Name Line", "`%C23*%O$t%C28[%C18$1%C28] %C30(%C24$2@$3%C30)%O: %C18$4%O`", 0: "Nickname", 1: "Username", 2: "Host", 3: "Full name");
print_event!(WhoisRealHost, "WhoIs Real Host", "`%C23*%O$t%C28[%C18$1%C28]%O Real Host: %C23$2%O, Real IP: %C30[%C23$3%C30]%O`", 0: "Nickname", 1: "Real user@host", 2: "Real IP", 3: "Message");
print_event!(WhoisServerLine, "WhoIs Server Line", "`%C23*%O$t%C28[%C18$1%C28]%O %C29$2%O`", 0: "Nickname", 1: "Server Information");
print_event!(WhoisSpecial, "WhoIs Special", "`%C23*%O$t%C28[%C18$1%C28]%O $2`", 0: "Nickname", 1: "Message", 2: "Numeric");
print_event!(YouJoin, "You Join", "`%C19*%O$tNow talking on %C22$2%O`", 0: "The nick of the joining person", 1: "The channel being joined", 2: "The host of the person", 3: "The account of the person");
print_event!(YouKicked, "You Kicked", "`%C19*%O$tYou have been kicked from %C22$2%C by %C26$3%O (%C20$4%O)`", 0: "The person being kicked", 1: "The channel", 2: "The nickname of the kicker", 3: "The reason");
print_event!(YouPart, "You Part", "`%C19*%O$tYou have left channel %C22$3%O`", 0: "The nick of the person leaving", 1: "The host of the person", 2: "The channel");
print_event!(YouPartWithReason, "You Part with Reason", "`%C19*%O$tYou have left channel %C22$3%C (%C24$4%O)`", 0: "The nick of the person leaving", 1: "The host of the person", 2: "The channel", 3: "The reason");
print_event!(YourAction, "Your Action", "`%C20*$t%B$1%B %C30$2%O`", 0: "Nickname", 1: "The action", 2: "Mode char", 3: "Identified text");
print_event!(YourInvitation, "Your Invitation", "`%C20*%O$tYou've invited %C18$1%O to %C22$2%O (%C24$3%O)`", 0: "Nick of person who have been invited", 1: "Channel Name", 2: "Server Name");
print_event!(YourMessage, "Your Message", "`%C20%H<%H$4$1%H>%H%O%C30$t$2%O`", 0: "Nickname", 1: "The text", 2: "Mode char", 3: "Identified text");
print_event!(YourNickChanging, "Your Nick Changing", "`%C20*%O$tYou are now known as %C18$2%O`", 0: "Old nickname", 1: "New nickname");

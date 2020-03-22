/* X-Chat
 * Copyright (C) 1998 Peter Zelezny.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301, USA
 */

static char * const pevt_genmsg_help[] = {
	N_("Left message"),
	N_("Right message"),
};

static char * const pevt_join_help[] = {
	N_("The nick of the joining person"),
	N_("The channel being joined"),
	N_("The host of the person"),
	N_("The account of the person"),
};

static char * const pevt_chanaction_help[] = {
	N_("Nickname"),
	N_("The action"),
	N_("Mode char"),
	N_("Identified text"),
};

static char * const pevt_chanmsg_help[] = {
	N_("Nickname"),
	N_("The text"),
	N_("Mode char"),
	N_("Identified text"),
};

static char * const pevt_privmsg_help[] = {
	N_("Nickname"),
	N_("The message"),
	N_("Identified text")
};

static char * const pevt_capack_help[] = {
	N_("Server Name"),
	N_("Acknowledged Capabilities")
};

static char * const pevt_capdel_help[] = {
	N_("Server Name"),
	N_("Removed Capabilities")
};

static char * const pevt_caplist_help[] = {
	N_("Server Name"),
	N_("Server Capabilities")
};

static char * const pevt_capreq_help[] = {
	N_("Requested Capabilities")
};

static char * const pevt_changenick_help[] = {
	N_("Old nickname"),
	N_("New nickname"),
};

static char * const pevt_newtopic_help[] = {
	N_("Nick of person who changed the topic"),
	N_("Topic"),
	N_("Channel"),
};

static char * const pevt_topic_help[] = {
	N_("Channel"),
	N_("Topic"),
};

static char * const pevt_kick_help[] = {
	N_("The nickname of the kicker"),
	N_("The person being kicked"),
	N_("The channel"),
	N_("The reason"),
};

static char * const pevt_part_help[] = {
	N_("The nick of the person leaving"),
	N_("The host of the person"),
	N_("The channel"),
};

static char * const pevt_chandate_help[] = {
	N_("The channel"),
	N_("The time"),
};

static char * const pevt_topicdate_help[] = {
	N_("The channel"),
	N_("The creator"),
	N_("The time"),
};

static char * const pevt_quit_help[] = {
	N_("Nick"),
	N_("Reason"),
	N_("Host"),
};

static char * const pevt_pingrep_help[] = {
	N_("Who it's from"),
	N_("The time in x.x format (see below)"),
};

static char * const pevt_notice_help[] = {
	N_("Who it's from"),
	N_("The message"),
};

static char * const pevt_channotice_help[] = {
	N_("Who it's from"),
	N_("The Channel it's going to"),
	N_("The message"),
};

static char * const pevt_uchangenick_help[] = {
	N_("Old nickname"),
	N_("New nickname"),
};

static char * const pevt_ukick_help[] = {
	N_("The person being kicked"),
	N_("The channel"),
	N_("The nickname of the kicker"),
	N_("The reason"),
};

static char * const pevt_partreason_help[] = {
	N_("The nick of the person leaving"),
	N_("The host of the person"),
	N_("The channel"),
	N_("The reason"),
};

static char * const pevt_ctcpsnd_help[] = {
	N_("The sound"),
	N_("The nick of the person"),
	N_("The channel"),
};

static char * const pevt_ctcpgen_help[] = {
	N_("The CTCP event"),
	N_("The nick of the person"),
};

static char * const pevt_ctcpgenc_help[] = {
	N_("The CTCP event"),
	N_("The nick of the person"),
	N_("The Channel it's going to"),
};

static char * const pevt_chansetkey_help[] = {
	N_("The nick of the person who set the key"),
	N_("The key"),
};

static char * const pevt_chansetlimit_help[] = {
	N_("The nick of the person who set the limit"),
	N_("The limit"),
};

static char * const pevt_chanop_help[] = {
	N_("The nick of the person who did the op'ing"),
	N_("The nick of the person who has been op'ed"),
};

static char * const pevt_chanhop_help[] = {
	N_("The nick of the person who has been halfop'ed"),
	N_("The nick of the person who did the halfop'ing"),
};

static char * const pevt_chanvoice_help[] = {
	N_("The nick of the person who did the voice'ing"),
	N_("The nick of the person who has been voice'ed"),
};

static char * const pevt_chanban_help[] = {
	N_("The nick of the person who did the banning"),
	N_("The ban mask"),
};

static char * const pevt_chanquiet_help[] = {
	N_("The nick of the person who did the quieting"),
	N_("The quiet mask"),
};

static char * const pevt_chanrmkey_help[] = {
	N_("The nick who removed the key"),
};

static char * const pevt_chanrmlimit_help[] = {
	N_("The nick who removed the limit"),
};

static char * const pevt_chandeop_help[] = {
	N_("The nick of the person who did the deop'ing"),
	N_("The nick of the person who has been deop'ed"),
};
static char * const pevt_chandehop_help[] = {
	N_("The nick of the person who did the dehalfop'ing"),
	N_("The nick of the person who has been dehalfop'ed"),
};

static char * const pevt_chandevoice_help[] = {
	N_("The nick of the person who did the devoice'ing"),
	N_("The nick of the person who has been devoice'ed"),
};

static char * const pevt_chanunban_help[] = {
	N_("The nick of the person who did the unban'ing"),
	N_("The ban mask"),
};

static char * const pevt_chanunquiet_help[] = {
	N_("The nick of the person who did the unquiet'ing"),
	N_("The quiet mask"),
};

static char * const pevt_chanexempt_help[] = {
	N_("The nick of the person who did the exempt"),
	N_("The exempt mask"),
};

static char * const pevt_chanrmexempt_help[] = {
	N_("The nick of the person removed the exempt"),
	N_("The exempt mask"),
};

static char * const pevt_chaninvite_help[] = {
	N_("The nick of the person who did the invite"),
	N_("The invite mask"),
};

static char * const pevt_chanrminvite_help[] = {
	N_("The nick of the person removed the invite"),
	N_("The invite mask"),
};

static char * const pevt_chanmodegen_help[] = {
	N_("The nick of the person setting the mode"),
	N_("The mode's sign (+/-)"),
	N_("The mode letter"),
	N_("The channel it's being set on"),
};

static char * const pevt_whois1_help[] = {
	N_("Nickname"),
	N_("Username"),
	N_("Host"),
	N_("Full name"),
};

static char * const pevt_whois2_help[] = {
	N_("Nickname"),
	N_("Channel Membership/\"is an IRC operator\""),
};

static char * const pevt_whois3_help[] = {
	N_("Nickname"),
	N_("Server Information"),
};

static char * const pevt_whois4_help[] = {
	N_("Nickname"),
	N_("Idle time"),
};

static char * const pevt_whois4t_help[] = {
	N_("Nickname"),
	N_("Idle time"),
	N_("Signon time"),
};

static char * const pevt_whois5_help[] = {
	N_("Nickname"),
	N_("Away reason"),
};

static char * const pevt_whois6_help[] = {
	N_("Nickname"),
};

static char * const pevt_whoisid_help[] = {
	N_("Nickname"),
	N_("Message"),
	"Numeric"
};

static char * const pevt_whoisauth_help[] = {
	N_("Nickname"),
	N_("Message"),
	N_("Account"),
};

static char * const pevt_whoisrealhost_help[] = {
	N_("Nickname"),
	N_("Real user@host"),
	N_("Real IP"),
	N_("Message"),
};

static char * const pevt_generic_channel_help[] = {
	N_("Channel Name"),
};

static char * const pevt_saslauth_help[] = {
	N_("Username"),
	N_("Mechanism")
};

static char * const pevt_saslresponse_help[] = {
	N_("Server Name"),
	N_("Raw Numeric or Identifier"),
	N_("Username"),
	N_("Message")
};

static char * const pevt_servertext_help[] = {
	N_("Text"),
	N_("Server Name"),
	N_("Raw Numeric or Identifier")
};

static char * const pevt_sslmessage_help[] = {
	N_("Text"),
	N_("Server Name")
};

static char * const pevt_invited_help[] = {
	N_("Channel Name"),
	N_("Nick of person who invited you"),
	N_("Server Name"),
};

static char * const pevt_usersonchan_help[] = {
	N_("Channel Name"),
	N_("Users"),
};

static char * const pevt_nickclash_help[] = {
	N_("Nickname in use"),
	N_("Nick being tried"),
};

static char * const pevt_connfail_help[] = {
	N_("Error"),
};

static char * const pevt_connect_help[] = {
	N_("Host"),
	N_("IP"),
	N_("Port"),
};

static char * const pevt_sconnect_help[] = {
	"PID"
};

static char * const pevt_generic_nick_help[] = {
	N_("Nickname"),
	N_("Server Name"),
	N_("Network")
};

static char * const pevt_chanmodes_help[] = {
	N_("Channel Name"),
	N_("Modes string"),
};

static char * const pevt_chanurl_help[] = {
	N_("Channel Name"),
	N_("URL"),
};

static char * const pevt_rawmodes_help[] = {
	N_("Nickname"),
	N_("Modes string"),
};

static char * const pevt_kill_help[] = {
	N_("Nickname"),
	N_("Reason"),
};

static char * const pevt_dccchaterr_help[] = {
	N_("Nickname"),
	N_("IP address"),
	N_("Port"),
	N_("Error"),
};

static char * const pevt_dccstall_help[] = {
	N_("DCC Type"),
	N_("Filename"),
	N_("Nickname"),
};

static char * const pevt_generic_file_help[] = {
	N_("Filename"),
	N_("Error"),
};

static char * const pevt_dccrecverr_help[] = {
	N_("Filename"),
	N_("Destination filename"),
	N_("Nickname"),
	N_("Error"),
};

static char * const pevt_dccrecvcomp_help[] = {
	N_("Filename"),
	N_("Destination filename"),
	N_("Nickname"),
	N_("CPS"),
};

static char * const pevt_dccconfail_help[] = {
	N_("DCC Type"),
	N_("Nickname"),
	N_("Error"),
};

static char * const pevt_dccchatcon_help[] = {
	N_("Nickname"),
	N_("IP address"),
};

static char * const pevt_dcccon_help[] = {
	N_("Nickname"),
	N_("IP address"),
	N_("Filename"),
};

static char * const pevt_dccsendfail_help[] = {
	N_("Filename"),
	N_("Nickname"),
	N_("Error"),
};

static char * const pevt_dccsendcomp_help[] = {
	N_("Filename"),
	N_("Nickname"),
	N_("CPS"),
};

static char * const pevt_dccoffer_help[] = {
	N_("Filename"),
	N_("Nickname"),
	N_("Pathname"),
};

static char * const pevt_dccfileabort_help[] = {
	N_("Nickname"),
	N_("Filename")
};

static char * const pevt_dccchatabort_help[] = {
	N_("Nickname"),
};

static char * const pevt_dccresumeoffer_help[] = {
	N_("Nickname"),
	N_("Filename"),
	N_("Position"),
};

static char * const pevt_dccsendoffer_help[] = {
	N_("Nickname"),
	N_("Filename"),
	N_("Size"),
	N_("IP address"),
};

static char * const pevt_dccgenericoffer_help[] = {
	N_("DCC String"),
	N_("Nickname"),
};

static char * const pevt_notifyaway_help[] = {
	N_("Nickname"),
	N_("Away Reason"),
};

static char * const pevt_notifynumber_help[] = {
	N_("Number of notify items"),
};

static char * const pevt_serverlookup_help[] = {
	N_("Server Name"),
};

static char * const pevt_servererror_help[] = {
	N_("Text"),
};

static char * const pevt_foundip_help[] = {
	N_("IP"),
};

static char * const pevt_dccrename_help[] = {
	N_("Old Filename"),
	N_("New Filename"),
};

static char * const pevt_ctcpsend_help[] = {
	N_("Receiver"),
	N_("Message"),
};

static char * const pevt_ignoreaddremove_help[] = {
	N_("Hostmask"),
};

static char * const pevt_resolvinguser_help[] = {
	N_("Nickname"),
	N_("Hostname"),
};

static char * const pevt_malformed_help[] = {
	N_("Nickname"),
	N_("The Packet"),
};

static char * const pevt_pingtimeout_help[] = {
	N_("Seconds"),
};

static char * const pevt_uinvite_help[] = {
	N_("Nick of person who have been invited"),
	N_("Channel Name"),
	N_("Server Name"),
};

static char * const pevt_banlist_help[] = {
	N_("Channel"),
	N_("Banmask"),
	N_("Who set the ban"),
	N_("Ban time"),
};

static char * const pevt_discon_help[] = {
	N_("Error"),
};

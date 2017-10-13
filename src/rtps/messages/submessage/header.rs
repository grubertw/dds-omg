// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 Contains all data structures used for RTPS submessage headers. 
 */

/// Preceeds every submessage.
///
/// Flags define the endianess of the data within the submessage, the presance
/// of optional elements within the submessage, and possibly modifies the 
/// interpretation of the submessage. Flags (other than flag 0), may vary for 
/// each submessage.
///
/// submessage_length represents the number of bytes untill the next submessage
/// header.
pub struct SubmessageHeader {
	pub submessage_id: u8,
	pub flags: u8,
	pub submessage_length: u16,
}

impl SubmessageHeader {
	/// All submessages use the first LSB to mark the endianess of the 
	/// submessage contents.  Test whether this is big or little endian.
	pub fn is_big_endian(&self) -> bool {
		self.flags & 0x01 == 1
	}
}

/// Encoding for the submessage_id within SubmessageHeader.
///
/// These values are defined by the RTPS v2.2 spec (9.4.5.1.1)
pub enum SubmessageKind {
	Pad 			= 0x01,
	AckNack 		= 0x06,
	Heartbeat		= 0x07,
	Gap				= 0x08,
	InfoTimestamp	= 0x09,
	InfoSource		= 0x0c,
	InfoReplyIp4	= 0x0d,
	InfoDestination = 0x0e,
	InfoReply 		= 0x0f,
	NackFrag		= 0x12,
	HeartbeatFrag	= 0x13,
	Data 			= 0x15,
	DataFrag 		= 0x16,
} 

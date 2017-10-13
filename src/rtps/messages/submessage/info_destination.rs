// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message is sent from the RTPS writer to modify the GUID prefix
 used to interpret the reader EntityId(s) appearing in submessages that follow.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct InfoDestination {
	header: SubmessageHeader,
	guid_prefix: GUIDPrefix,
}

impl InfoDestination {
	
	/// guid_prefix specifies the new prefix to be used for subsequent 
	/// EntityId(s) found within the message.
	pub fn new (guid_prefix: GUIDPrefix) -> InfoDestination {
		let flags: u8 = 0x01;
		let msg_len: u16 = guid_prefix.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::InfoDestination as u8,
			flags: flags,
			submessage_length: msg_len};

		InfoDestination {
			header: header,
			guid_prefix: guid_prefix
		}
	}
}
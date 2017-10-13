// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message is used to send a timestamp which applies to the submessages
 that follow within the same message.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct InfoTimestamp {
	header: SubmessageHeader,
	timestamp: Timestamp,
}

impl InfoTimestamp {
	///
	/// invalidates flag is used to indicate subsequent submessages have a timestamp.
	///
	pub fn new (invalidates: bool,
				timestamp: Timestamp) -> InfoTimestamp {
		let flags: u8 = 0x01
			| ((invalidates as u8) << 1);
		let msg_len: u16 = timestamp.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::InfoTimestamp as u8,
			flags: flags,
			submessage_length: msg_len};

		InfoTimestamp {
			header: header,
			timestamp: timestamp
		}
	}

	/// invalidates flag is used to indicate subsequent submessages have a timestamp.
	pub fn invalidates(&self) -> bool {
		self.header.flags & 0x02 == 1
	}
}
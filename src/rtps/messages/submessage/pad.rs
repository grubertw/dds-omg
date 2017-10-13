// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 The purpose of this (sub) message is to allow the introduction of any padding
 necessary to meet any desired memory allignment requirements.
 */
use rtps::messages::submessage::header::*;

pub struct Pad {
	header: SubmessageHeader,
}

impl Pad {
	/// pad_length length of padding bytes (zeros) within this submessage.
	pub fn new (pad_length: u16) -> Pad {
		let flags: u8 = 0x01;

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::Pad as u8,
			flags: flags,
			submessage_length: pad_length};

		Pad {header: header}
	}
}
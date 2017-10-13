// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message modifies the local source of the submessage that follows.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct InfoSource {
	header: SubmessageHeader,
	protocol_version: ProtocolVersion,
	vendor_id: VendorId,
	guid_prefix: GUIDPrefix,
}

impl InfoSource {
	/// protocol_version version of the RTPS protocol and messages that follow.
	///
	/// vender_id unique ID of the RTSP implementation.
	///
	/// guid_prefix to all RTPS readers/writers within the participant.
	pub fn new (protocol_version: ProtocolVersion,
		        vendor_id: VendorId,
		        guid_prefix: GUIDPrefix) -> InfoSource {
		let flags: u8 = 0x01;
		let msg_len: u16 = protocol_version.size() 
			+ vendor_id.size() + guid_prefix.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::InfoSource as u8,
			flags: flags,
			submessage_length: msg_len};

		InfoSource {
			header: header,
			protocol_version: protocol_version,
			vendor_id: vendor_id,
			guid_prefix: guid_prefix
		}
	}
}
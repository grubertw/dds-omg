// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message is sent from the RTPS reader to the RTPS writer. It 
 contains explicit information on where to send a reply to the submessages
 that follow it within the same message.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct InfoReply {
	header: SubmessageHeader,
	unicast_locator_list: LocatorList,
	multicast_locator_list: LocatorList,
}

impl InfoReply {

	/// has_multicast indicates there are multicast addresses in the locator lists.
	///
	/// unicast_locator_list contains all unicast UDP addresses.
	///
	/// multicast_locator_list contains all multicast UDP addresses.
	pub fn new (has_multicast: bool,
			    unicast_locator_list: LocatorList,
			    multicast_locator_list: LocatorList) -> InfoReply {
		let flags: u8 = 0x01
			| ((has_multicast as u8) << 1);
		let msg_len: u16 = unicast_locator_list.size() 
			+ multicast_locator_list.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::InfoReply as u8,
			flags: flags,
			submessage_length: msg_len};

		InfoReply {
			header: header,
			unicast_locator_list: unicast_locator_list,
			multicast_locator_list: multicast_locator_list
		}
	}

	/// True if this message has locator lists with multicast addresses.
	pub fn has_multicast(&self) -> bool {
		self.header.flags & 0x02 == 1
	}
}
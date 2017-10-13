// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message is sent from the RTPS writer to communicate
 data fragments that are available, to the reader.  Once all data
 fragments are available, a normal heartbeat message is sent.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct HeartbeatFrag {
	header: SubmessageHeader,
	reader_id: EntityId,
	writer_id: EntityId,
	writer_sn: SequenceNumber,
	last_fragment_num: FragmentNumber,
	count: Count,
}

impl HeartbeatFrag {

	/// rid identified the RTPS reader being informed of the update.
	///
	/// wid identifies the RTPS writer sending the update.
	///
	/// writer_sn identifies which sequence number of the data change 
	/// has been fragmented.
	///
	/// last_fragment_num all fragments up to and including this fragment 
	/// number are available.
	///
	/// count assists with duplicates that may be recieved via mutiple network
	/// pathways.
	pub fn new (rid: EntityId,
				wid: EntityId,
				writer_sn: SequenceNumber,
				last_frag_num: FragmentNumber,
				count: Count) -> HeartbeatFrag {
		let flags: u8 = 0x01;
		let msg_len: u16 = rid.size() + wid.size() + writer_sn.size() 
			+ last_frag_num.size() + count.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::HeartbeatFrag as u8,
			flags: flags,
			submessage_length: msg_len};

		HeartbeatFrag {
			header: header,
			reader_id: rid,
			writer_id: wid,
			writer_sn: writer_sn,
			last_fragment_num: last_frag_num,
			count: count
		}
	}
}
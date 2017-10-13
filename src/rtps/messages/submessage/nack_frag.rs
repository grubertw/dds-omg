// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message is used to communicate the state of the RTPS reader to the
 RTPS writer. When a data change is sent as a series of fragments, the NackFrag
 submessage allows the reader to inform the writer about specific fragment 
 numbers that are still missing.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct NackFrag {
	header: SubmessageHeader,
	reader_id: EntityId,
	writer_id: EntityId,
	writer_sn: SequenceNumber,
	fragment_number_state: FragmentNumberSet,
	count: Count,
}

impl NackFrag {
	/// rid identifies the reader requesting certian fragments.
	///
	/// wid identifies the writer being asked to resend some fragments.
	///
	/// writer_sn is the sequence number for which some fragments are missing.
	///
	/// fn_state communicates the state of the reader to the writer. Indicates
	/// missing fragments on the reader side.
	///
	/// count allows detection of duplicate messages sent along multiple channels.
	pub fn new (rid: EntityId,
				wid: EntityId,
				writer_sn: SequenceNumber,
				fn_state: FragmentNumberSet,
				count: Count) -> NackFrag {
		let flags: u8 = 0x01;
		let msg_len: u16 = rid.size() + wid.size() + writer_sn.size() 
			+ fn_state.size() + count.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::NackFrag as u8,
			flags: flags,
			submessage_length: msg_len};

		NackFrag {
			header: header,
			reader_id: rid,
			writer_id: wid,
			writer_sn: writer_sn,
			fragment_number_state: fn_state,
			count: count
		}
	}
}
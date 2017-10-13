// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 The purpose of this (sub) message is to communicate the state of the reader to
 a writer. It allows the reader to inform the writer about the sequence numbers
 it has received and which ones are still missing.  This submessage can be used 
 to do both positive and negative ackowledgments.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct AckNack {
	header: SubmessageHeader,
	reader_id: EntityId,
	writer_id: EntityId,
	reader_sn_state: SequenceNumberSet,
	count: Count,
}

impl AckNack {
	/// Adds a Final flag to flags within the submessage header.  This indicates to
	/// the writer whether a response is necessary.
	///
	/// rid is the reader's EntityId.
	///
	/// wid is the writer's EntityId.
	///
	/// r_sn_state is the set of sequence numbers the reader has not yet 
	/// received from the writer. All sequence numbers below the base are confirmed
	/// as received by the reader.
	///
	/// count is a way to de-dup the message in case there were multiple 
	/// transmission paths.
	pub fn new(is_final: bool, 
			   rid: EntityId, 
			   wid: EntityId, 
			   r_sn_state: SequenceNumberSet,
			   c: Count) -> AckNack {
		let flags: u8 = 0x01 | ((is_final as u8) << 1);
		let msg_len: u16 = rid.size() + wid.size() + r_sn_state.size() + c.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::AckNack as u8,
			flags: flags,
			submessage_length: msg_len};

		AckNack {header: header, 
				 reader_id: rid, 
				 writer_id: wid, 
				 reader_sn_state: r_sn_state, 
				 count: c}
	}

	/// indicates if the reader expects a response from the writer.
	pub fn is_final(&self) -> bool {
		self.header.flags & 0x02 == 1
	}
}
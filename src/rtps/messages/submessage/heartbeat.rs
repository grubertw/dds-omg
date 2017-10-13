// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 This (sub) message is sent from the RTPS writer to communicate
 the changes in sequence numbers that it has available, to the reader.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct Heartbeat {
	header: SubmessageHeader,
	reader_id: EntityId,
	writer_id: EntityId,
	first_sn: SequenceNumber,
	last_sn: SequenceNumber,
	count: Count,
}

impl Heartbeat {

	/// is_final signifies the writer does not need to respond to the reader.
	///
	/// has_liveliness signifies the DDS writer has manually set the lifelyness
	/// flag.
	///
	/// rid identified the RTPS reader being informed of the update.
	///
	/// wid identifies the RTPS writer sending the update.
	/// 
	/// first_sn signifies the first sequence number available from the writer.
	///
	/// last_sn signifies the last sequence number available from the writer.
	///
	/// count assists with duplicates that may be recieved via mutiple network
	/// pathways.
	pub fn new (is_final: bool,
				has_liveliness: bool,
				rid: EntityId,
				wid: EntityId,
				first_sn: SequenceNumber,
				last_sn: SequenceNumber,
				count: Count) -> Heartbeat {
		let flags: u8 = 0x01 
			| ((is_final as u8) << 1)
			| ((has_liveliness as u8) << 2);
		let msg_len: u16 = rid.size() + wid.size() + first_sn.size() 
			+ last_sn.size() + count.size();

		let header = SubmessageHeader{
			submessage_id: SubmessageKind::Heartbeat as u8,
			flags: flags,
			submessage_length: msg_len};

		Heartbeat {
			header: header,
			reader_id: rid,
			writer_id: wid,
			first_sn: first_sn,
			last_sn: last_sn,
			count: count
		}
	}

	/// indicates if the reader expects a response from the writer.
	pub fn is_final(&self) -> bool {
		self.header.flags & 0x02 == 1
	}

	/// signifies the DDS writer has manually set the lifelyness flag.
	pub fn has_liveliness(&self) -> bool {
		self.header.flags & 0x04 == 1
	}
}

// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 The purpose of this (sub) message is to notify the RTPS reader that a range of
 sequence numbers are no longer relevant. The set may be a contigious range of 
 sequence numbers or a specific set of sequence numbers.
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct Gap {
	header: SubmessageHeader,
	reader_id: EntityId,
	writer_id: EntityId,
	gap_start: SequenceNumber,
	gap_list: SequenceNumberSet,
}

impl Gap {
	/// rid identifies the reader entity that is being informed of irrelevance
	/// of a set of sequence numbers.
	///
	/// wid identifies the writer entity to which the range of sequence numbers
	/// apply.
	///
	/// gap_start identifies the first sequence number in the interval of 
	/// irrelevant sequence numbers.
	///
	/// gap_list can identify the last sequence in the interval and can also 
	/// include additional sequence numbers beyond the last that are not 
	/// contigious.
	pub fn new(rid: EntityId,
			   wid: EntityId,
			   gap_start: SequenceNumber,
			   gap_list: SequenceNumberSet) -> Gap {
		let flags: u8 = 0x01;
		let msg_len: u16 = rid.size() + wid.size() 
			+ gap_start.size() + gap_list.size();

		let header = SubmessageHeader {
			submessage_id: SubmessageKind::Gap as u8,
			flags: flags,
			submessage_length: msg_len
		};

		Gap {
			header: header,
			reader_id: rid,
			writer_id: wid,
			gap_start: gap_start,
			gap_list: gap_list
		}
	}
}

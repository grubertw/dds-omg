// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 The purpose of this (sub) message is to notify the RTPS reader of a change to
 a data object belonging to an RTPS writer.  Possible changes include changes 
 to value, or changes to lifecycle. 
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;

pub struct Data {
	pub header: SubmessageHeader,
	pub extra_flags: u16,
	pub octets_to_inline_qos: u16,
	pub reader_id: EntityId,
	pub writer_id: EntityId,
	pub writer_sn: SequenceNumber,
	pub inline_qos: Option<ParameterList>,
	pub serialized_payload: Option<SerializedPayload>,
}

impl Data {
	/// Adds three extra flags to SubmessageHeader: InlineQosFlag, DataFlag
	/// and KeyFLag.
	///
	/// InlineQosFlag at the 2nd LSB signifies the submessage has QoS settings
	/// within a ParameterList that override any previous QoS used by the writer.
	///
	/// If the DataFlag is set and KeyFlag is not set, then serialized_payload
	/// is interpreted as the value of the data object.
	///
	/// If the KeyFlag is set and the DataFlag is not set, then the data is 
	/// interpreted as the value of the key that identifies the registered 
	/// instance of the data object.
	///
	/// The writer GUID is obtained using the state of the Receiver:
	///  Receiver.source_guid_prefix + Data.writer_id
	///
	/// The reader GUID is obtained using the state of the Receiver:
	///  Receiver.dest_guid_prefix + Data.reader_id
	///
	/// The Data.reader_id can be ENTITYID_UNKNOWN, in which case the Data 
	/// applies to all readers of that writer GUID within the participant 
	/// identified by the GuidPrefix_t Receiver.dest_guid_prefix.
	pub fn new (has_inline_qos: bool,
			    has_data: bool,
			    has_key: bool,
			    rid: EntityId,
			    wid: EntityId,
			    w_sn: SequenceNumber,
			    i_qos: Option<ParameterList>,
			    data: Option<SerializedPayload>) -> Data {
		let flags: u8 = 0x01 
					  | ((has_inline_qos as u8) << 1)
					  | ((has_data as u8) << 2)
					  | ((has_key as u8) << 3);

		let mut msg_len: u16 = rid.size() + wid.size() + w_sn.size();
		if let Some(ref qos) = i_qos {
			msg_len += qos.size()
		}

		if let Some(ref data) = data {
			msg_len += data.len() as u16
		}

		let header = SubmessageHeader {
			submessage_id: SubmessageKind::Data as u8,
			flags: flags,
			submessage_length: msg_len
		};

		Data {
			header: header,
			extra_flags: 0,
			octets_to_inline_qos: (rid.size() + wid.size() + w_sn.size()),
			reader_id: rid,
			writer_id: wid,
			writer_sn: w_sn,
			inline_qos: i_qos,
			serialized_payload: data
		}
	}

	/// True if there is a ParameterList present in this submessage.
	pub fn has_inline_qos(&self) -> bool {
		self.header.flags & 0x02 == 1
	}

	/// True if the serialized_payload represents the value of the data object.
	pub fn has_data(&self) -> bool {
		self.header.flags & 0x04 == 1
	}

	/// True if the serialized_payload represents a key to a registered data object.
	pub fn has_key(&self) -> bool {
		self.header.flags & 0x08 == 1
	}
}
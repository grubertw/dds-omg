// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 The purpose of this (sub) message is to split the contents of data message 
 into fragments. It largely shares the same structure as the Data submessage,
 allowing data contents to be broken down when it exceeds the size of a UDP
 datagram (or whatever underlying transport is being used). 
 */
use rtps::messages::submessage::header::*;
use rtps::messages::submessage::element::*;
use rtps::messages::submessage::data::*;

pub struct DataFragment {
	data_header: Data,
	fragment_starting_num: FragmentNumber,
	fragments_in_submessage: u16,
	data_size: u32,
	fragment_size: u16,
}

impl DataFragment {
	/// Accepts same arguments as the Data submessage with the same semantics
	/// in addition to the following arguments:
	///
	/// fragment_starting_num represents the first fragment number in this 
	/// submessage. As all fragments within a submessage must be contigious,
	/// this is the only fragment number listed within the submessage.
	///
	/// fragments_in_submessage represents the number of fragments within this 
	/// submessage.
	///
	/// data_size represents the total size (in bytes) of the data accross all 
	/// DataFragment submessages.
	///
	/// fragment_size represents the size of each fragment (in bytes).
	pub fn new (has_inline_qos: bool,
			    has_data: bool,
			    has_key: bool,
			    rid: EntityId,
			    wid: EntityId,
			    w_sn: SequenceNumber,
			    frag_starting_num: FragmentNumber,
			    frags_in_msg: u16,
			    data_size: u32,
			    frag_size: u16,
			    i_qos: Option<ParameterList>,
			    data: Option<SerializedPayload>) -> DataFragment {
		let flags: u8 = 0x01 
					  | ((has_inline_qos as u8) << 1)
					  | ((has_data as u8) << 2)
					  | ((has_key as u8) << 3);

		let mut msg_len: u16 = rid.size() + wid.size() + w_sn.size()
			+ frag_starting_num.size() + 8;

		if let Some(ref qos) = i_qos {
			msg_len += qos.size()
		}

		if let Some(ref data) = data {
			msg_len += data.len() as u16
		}

		let header = SubmessageHeader {
			submessage_id: SubmessageKind::DataFrag as u8,
			flags: flags,
			submessage_length: msg_len
		};

		let data_header = Data {
			header: header,
			extra_flags: 0,
			octets_to_inline_qos: (rid.size() + wid.size() + w_sn.size()),
			reader_id: rid,
			writer_id: wid,
			writer_sn: w_sn,
			inline_qos: i_qos,
			serialized_payload: data
		};

		DataFragment {
			data_header: data_header,
			fragment_starting_num: frag_starting_num,
			fragments_in_submessage: frags_in_msg,
			data_size: data_size,
			fragment_size: frag_size
		}		
	}

	/// True if there is a ParameterList present in this submessage.
	pub fn has_inline_qos(&self) -> bool {
		self.data_header.header.flags & 0x02 == 1
	}

	/// True if the serialized_payload represents the value of the data object.
	pub fn has_data(&self) -> bool {
		self.data_header.header.flags & 0x04 == 1
	}

	/// True if the serialized_payload represents a key to a registered data object.
	pub fn has_key(&self) -> bool {
		self.data_header.header.flags & 0x08 == 1
	}
}
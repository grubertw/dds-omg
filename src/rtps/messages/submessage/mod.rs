// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 Contains all definitions and data types used for RTPS submessages. 
 */

pub mod header;
pub mod element;
pub mod ack_nack;
pub mod data;
pub mod data_frag;
pub mod gap;
pub mod heartbeat;
pub mod heartbeat_frag;
pub mod info_destination;
pub mod info_reply;
pub mod info_source;
pub mod info_timestamp;
pub mod nack_frag;
pub mod pad;

use rtps::*;
use rtps::guid::*;

/// The representation of time, accoring to the Network Time Protocol (NTP)
/// standard IETF RFC 1305. Time is expressed using this formula:
///
/// time = seconds + (fraction / 2^(32)).
///
/// TIME_ZERO corresponds to the Unix prime epoch 0h, 1 January 1970
pub struct Time_t {
	seconds: i32,
	fraction: u32,
}
pub const TIME_ZERO: Time_t = 		Time_t {seconds: 0, fraction: 0};
pub const TIME_INVALID: Time_t = 	Time_t {seconds: -1, fraction: 0xffffffff};
pub const TIME_INFINITE: Time_t = 	Time_t {seconds: 0x7fffffff, fraction: 0xffffffff};

/// Used when data segments are broken into fragments.
pub struct FragmentNumber_t(u32);

pub struct Count_t(i32);

pub struct KeyHash_t([u8; 16]);

pub struct StatusInfo_t([u8; 4]);

pub struct ParameterId_t(i16);

/// Strings for each field must always be allocated with 256 bytes,
/// excepting filter expression, which is variable length. 
pub struct ContentFilterProperty_t {
	content_filtered_topic_name: String,
	related_topic_name: String,
	filter_class_name: String,
	filter_expression: String,
	expression_parameters: Vec<String>,
}

pub struct FilterResult_t(Vec<i32>);
pub struct FilterSignature_t([i32; 4]);
pub struct ContentFilterInfo_t {
	filter_result: FilterResult_t,
	filter_signatures: Vec<FilterSignature_t>,
}

pub struct Property_t {
	name: String,
	value: String,
}

pub struct EntityName_t(String);

pub struct OriginalWriterInfo_t {
	origional_writer_guid: GUID_t,
	origional_writer_sn:SequenceNumber_t,
}

pub struct BuiltinEndpointSet_t(u32);
pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_ANNOUNCER: BuiltinEndpointSet_t = 			BuiltinEndpointSet_t(0x00000001);
pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_DETECTOR: BuiltinEndpointSet_t = 			BuiltinEndpointSet_t(0x00000002);
pub const DISC_BUILTIN_ENDPOINT_PUBLICATION_ANNOUNCER: BuiltinEndpointSet_t = 			BuiltinEndpointSet_t(0x00000004);
pub const DISC_BUILTIN_ENDPOINT_PUBLICATION_DETECTOR: BuiltinEndpointSet_t = 			BuiltinEndpointSet_t(0x00000008);
pub const DISC_BUILTIN_ENDPOINT_SUBSCRIPTION_ANNOUNCER: BuiltinEndpointSet_t = 			BuiltinEndpointSet_t(0x00000010);
pub const DISC_BUILTIN_ENDPOINT_SUBSCRIPTION_DETECTOR: BuiltinEndpointSet_t = 			BuiltinEndpointSet_t(0x00000020);
pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_PROXY_ANNOUNCER: BuiltinEndpointSet_t = 	BuiltinEndpointSet_t(0x00000040);
pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_PROXY_DETECTOR: BuiltinEndpointSet_t = 		BuiltinEndpointSet_t(0x00000080);
pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_STATE_ANNOUNCER: BuiltinEndpointSet_t = 	BuiltinEndpointSet_t(0x00000100);
pub const DISC_BUILTIN_ENDPOINT_PARTICIPANT_STATE_DETECTOR: BuiltinEndpointSet_t = 		BuiltinEndpointSet_t(0x00000200);
pub const BUILTIN_ENDPOINT_PARTICIPANT_MESSAGE_DATA_WRITER: BuiltinEndpointSet_t = 		BuiltinEndpointSet_t(0x00000400);
pub const BUILTIN_ENDPOINT_PARTICIPANT_MESSAGE_DATA_READER: BuiltinEndpointSet_t = 		BuiltinEndpointSet_t(0x00000800);
















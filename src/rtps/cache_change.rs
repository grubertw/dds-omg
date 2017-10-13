// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 Represents an individual change made to a data-object. Includes the creation, modification, and deletion of data-objects.
 */
use rtps::*;
use rtps::guid::*;
use std::cmp::Ordering;

pub struct CacheChange {
	pub kind: ChangeKind_t,
	pub writer_guid: GUID_t,
	pub instance_handle: InstanceHandle_t,
	pub sequence_number: SequenceNumber_t,
	pub data_value: Data,
}

impl PartialEq for CacheChange {
	fn eq(&self, other: &CacheChange) -> bool {
 		   self.kind == other.kind
 		&& self.writer_guid == other.writer_guid
 		&& self.instance_handle == other.instance_handle
 		&& self.sequence_number == other.sequence_number
	}
}
impl Eq for CacheChange {}

impl Ord for CacheChange {
	fn cmp(&self, other: &Self) -> Ordering {
 		self.sequence_number.cmp(&other.sequence_number)
	}
}

impl PartialOrd for CacheChange {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
 		Some(self.cmp(other))
 	}
}

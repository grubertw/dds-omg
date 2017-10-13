// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
The HistoryCache is part of the interface between DDS and RTPS and plays different roles on the reader and the writer side.
On the writer side, the HistoryCache contains the partial history of changes to data-objects made by the corresponding DDS Writer that are needed to service existing and future matched RTPS Reader endpoints. The partial history needed depends on the DDS Qos and the state of the communications with the matched RTPS Reader endpoints.
On the reader side, it contains the partial superposition of changes to data-objects made by all the matched RTPS Writer endpoints.
The word “partial” is used to indicate that it is not necessary that the full history of all changes ever made is maintained. Rather what is needed is the subset of the history needed to meet the behavioral needs of the RTPS protocol and the QoS needs of the related DDS entities. The rules that define this subset are defined by the RTPS protocol and depend both on the state of the communications protocol and on the QoS of the related DDS entities.
The HistoryCache is part of the interface between DDS and RTPS. In other words, both the RTPS entities and their related DDS entities are able to invoke the operations on their associated HistoryCache.
 */
use std::sync::Arc;
use rtps::cache_change::CacheChange;
use rtps::SequenceNumber_t;

pub struct HistoryCache {
	changes: Vec<Arc<CacheChange>>,
}

impl HistoryCache {
	/// Generates a new HistoryCache with an empty vector of CacheChange(s).
	pub fn new() -> HistoryCache {
		HistoryCache{changes: Vec::new()}
	}

	/// Appends a_change to the end of changes vector.
	pub fn add_change(&mut self, a_change: Arc<CacheChange>) {
		self.changes.push(a_change)
	}

	/// Removes first found a_change from the vector.
	/// (a_change(s) within the vector should be unique)
	pub fn remove_change(&mut self, a_change: Arc<CacheChange>) {
		let r = self.changes.binary_search(&a_change);
		if let Ok(i) = r {
			self.changes.remove(i);
		}
	}

	/// Get the minimum sequence number in the history cache.
	pub fn get_seq_num_min(&self) -> Option<SequenceNumber_t> {
		match self.changes.first() {
			Some(c) => Some(c.sequence_number),
			None => None,
		}
	}

	/// Get the maximum sequence number in the history cache.
	pub fn get_seq_num_max(&self) -> Option<SequenceNumber_t> {
		match self.changes.last() {
			Some(c) => Some(c.sequence_number),
			None => None,
		}
	}

	/// Get a references to the changes.
	pub fn get_changes(&self) -> Vec<Arc<CacheChange>> {
		self.changes.to_vec()
	}
}
// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
RTPS Writer specializes RTPS Endpoint and represents the actor that sends 
CacheChange messages to the matched RTPS Reader endpoints. Its role is to 
transfer all CacheChange changes in its HistoryCache to the HistoryCache 
of the matching remote RTPS Readers.
*/

use std::sync::Arc;
use std::iter::FromIterator;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use tokio_core::reactor::Handle;
use tokio_core::net::UdpSocket;

use rtps::*;
use rtps::guid::*;
use rtps::entity::Entity;
use rtps::entity::participant::Participant;
use rtps::entity::endpoint::Endpoint;
use rtps::history_cache::HistoryCache;
use rtps::cache_change::CacheChange;

pub struct Writer {
	// Support for Entity.
	guid: GUID_t,

	// Support for Endpoint.
	unicast_locator_list: Vec<Locator_t>,
	multicast_locator_list: Vec<Locator_t>,
	reliability_level: ReliabilityKind_t,
	topic_kind: TopicKind_t,

	writer_cache: HistoryCache,
	push_mode: bool,
	heartbeat_period: Duration_t,
	nack_response_delay: Duration_t,
	nack_suppression_duration: Duration_t,
	last_change_sequence_number: SequenceNumber_t,
	matched_readers: Vec<ReaderProxy>,

	participant: Arc<Participant>,

	socket: UdpSocket,
}

/// The RTPS ReaderProxy class represents the information an RTPS StatefulWriter 
/// maintains on each matched RTPS Reader. The matching of an RTPS StatefulWriter 
/// with an RTPS Reader means that the RTPS StatefulWriter will send the 
/// CacheChange changes in the writer’s HistoryCache to the matched RTPS Reader 
/// represented by the ReaderProxy. The matching is a consequence of the match 
/// of the corresponding DDS entities. That is, the DDS DataWriter matches a 
/// DDS DataReader by Topic, has compatible QoS, and is not being explicitly 
/// ignored by the application that uses DDS.
pub struct ReaderProxy {
	remote_reader_guid: GUID_t,
	expects_inline_qos: bool,
	unicast_locator_list: Vec<Locator_t>,
	multicast_locator_list: Vec<Locator_t>,
	changes_for_reader: Vec<ChangeForReader>,
	is_active: bool,
}

pub struct ChangeForReader {
	status: ChangeForReaderStatusKind,
	is_relivant: bool,
	change: Arc<CacheChange>,
}

impl Entity for Writer {
	fn guid(&self) -> &GUID_t {
		&self.guid
	}
}

impl Endpoint for Writer {
	fn unicast_locator_list(&self) -> &Vec<Locator_t> {
		&self.unicast_locator_list
	}

	fn multicast_locator_list(&self) -> &Vec<Locator_t> {
		&self.multicast_locator_list
	}

	fn reliability_level(&self) -> &ReliabilityKind_t {
		&self.reliability_level
	}

	fn topic_kind(&self) -> &TopicKind_t {
		&self.topic_kind
	}

	fn participant(&self) -> Arc<Participant> {
		self.participant.clone()
	}
}

impl Writer {
	/// Creates a new RTPS Writer.
	pub fn new(guid: GUID_t,
		       unicast_locator_list: Vec<Locator_t>,
		       multicast_locator_list: Vec<Locator_t>,
		       reliability_level: ReliabilityKind_t,
		       topic_kind: TopicKind_t,
		       push_mode: bool,
		       heartbeat_period: Duration_t,
		       nack_response_delay: Duration_t,
		       nack_suppression_duration: Duration_t,
		       participant: Arc<Participant>,
		       use_ip_v6: bool,
		       handle: &Handle) -> Writer {
		let soc = if use_ip_v6 == true {
			UdpSocket::bind(&SocketAddr::new(
				IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0)), 0), 
						   handle).unwrap()
		} else {
			// binding to 0.0.0.0:0 instructs OS to open the socket on
			// a port of it's coosing, accross all available network
			// interfaces.
			UdpSocket::bind(&SocketAddr::new(
				IpAddr::V4(Ipv4Addr::new(0,0,0,0)), 0), 
						   handle).unwrap()
		};

		Writer {
			guid: guid,
			unicast_locator_list: unicast_locator_list,
			multicast_locator_list: multicast_locator_list,
			reliability_level: reliability_level,
			topic_kind: topic_kind,
			push_mode: push_mode,
			heartbeat_period: heartbeat_period,
			nack_response_delay: nack_response_delay,
			nack_suppression_duration: nack_suppression_duration,
			last_change_sequence_number: SequenceNumber_t(0),
			writer_cache: HistoryCache::new(),
			matched_readers: Vec::new(),
			participant: participant,
			socket: soc
		}
	}

	/// This operation creates a new CacheChange to be appended to the RTPS 
	/// Writer’s HistoryCache. The sequence number of the CacheChange is 
	/// automatically set to be the sequenceNumber of the previous change 
	/// plus one.
	pub fn new_change(&mut self, 
					  kind: ChangeKind_t,
					  data: Data,
					  handle: InstanceHandle_t) -> Arc<CacheChange> {
		self.last_change_sequence_number += SequenceNumber_t(1);

		Arc::new(CacheChange {
			kind: kind,
			writer_guid: self.guid,
			instance_handle: handle,
			sequence_number: self.last_change_sequence_number,
			data_value: data
		})
	}

	/// This operation creates a new CacheChange and automatically adds it 
	/// to the change cache within this writer.  Furthermore, it will push
	/// a write operation within all reader proxies affected by this change.
	pub fn push_change(&mut self,
					   kind: ChangeKind_t,
					   data: Data,
					   handle: InstanceHandle_t) {
		let change = self.new_change(kind, data, handle);
		self.writer_cache.add_change(change);
	}

	/// Adds a_reader_proxy to matched_readers.
	pub fn matched_reader_add(&mut self, a_reader_proxy: ReaderProxy) {
		self.matched_readers.push(a_reader_proxy)
	}

	/// Removes a_reader_proxy from matched_readers.
	pub fn matched_reader_remove(&mut self, a_reader_proxy: &ReaderProxy) {
		let r = self.matched_readers.iter().position(|ref proxy|
			proxy.remote_reader_guid == a_reader_proxy.remote_reader_guid);

		if let Some(i) = r {
			self.matched_readers.remove(i);
		}
	}

	/// Get ReaderProxy by it's reader GUID.
	pub fn matched_reader_lookup(&self, a_reader_guid: GUID_t) -> Option<&ReaderProxy> {
		self.matched_readers.iter().find(|ref proxy| 
			proxy.remote_reader_guid == a_reader_guid)
	}

	/// This operation takes a CacheChange a_change as a parameter and 
	/// determines whether all the ReaderProxy have acknowledged the 
	/// CacheChange. The operation will return true if all ReaderProxy 
	/// have acknowledged the corresponding CacheChange and false otherwise.
	pub fn is_acked_by_all(&self, a_change: Arc<CacheChange>) -> bool {
		self.matched_readers.iter().all(|ref proxy| {
			let c = proxy.changes_for_reader.iter().find(|ref cfr|
			            cfr.status == ChangeForReaderStatusKind::ACKNOWLEDGED
			         && cfr.is_relivant == true
				     && cfr.change == a_change); 

			if c.is_some() {true} else {false}
		})
	}
}

impl ReaderProxy {
	/// Creates a new ReaderProxy.
	/// Pass a tokio reactor handle for handling asynch IO.
	pub fn new<F>(remote_reader_guid: GUID_t,
		          expects_inline_qos: bool,
		          unicast_locator_list: Vec<Locator_t>,
		          multicast_locator_list: Vec<Locator_t>,
		          is_active: bool,
		          changes: &Vec<Arc<CacheChange>>,
		          time_based_filter: F,
		          content_based_filter: F,
		          push_mode: bool) -> ReaderProxy 
					where  F: Fn(&CacheChange) -> bool {
		let cfr_it = changes.iter()
			.map(|change| ChangeForReader {
				status: if push_mode == true {
						ChangeForReaderStatusKind::UNSENT
					} else {
						ChangeForReaderStatusKind::UNACKNOWLEDGED
					},
				is_relivant: false,
				change: change.clone()
			});
		let mut changes_for_reader = Vec::from_iter(cfr_it);

		// changes for reader must be mutated in place because all changes must
		// be kept whether relivant or not.
		for cfr in &mut changes_for_reader {
			if time_based_filter(cfr.change.as_ref()) == true {
				cfr.is_relivant = true;
			}
		}

		for cfr in &mut changes_for_reader {
			if content_based_filter(cfr.change.as_ref()) == true {
				cfr.is_relivant = true;
			}
		}

		ReaderProxy {
			remote_reader_guid: remote_reader_guid,
			expects_inline_qos: expects_inline_qos,
			unicast_locator_list: unicast_locator_list,
			multicast_locator_list: multicast_locator_list,
			changes_for_reader: changes_for_reader,
			is_active: is_active
		}
	}

	/// This operation returns the subset of changes for the ReaderProxy the 
	/// have status ‘UNSENT.’ This represents the set of changes that have 
	/// not been sent to the RTPS Reader represented by the ReaderProxy.
	pub fn unsent_changes(&self) -> Vec<&ChangeForReader> {
		self.changes_for_reader.iter()
			.filter(|ref cfr| cfr.status == ChangeForReaderStatusKind::UNSENT)
			.collect()
	}

	/// This operation returns the subset of changes for the ReaderProxy that
	/// have status ‘UNACKNOWLEDGED.’ This represents the set of changes that 
	/// have not been acknowledged yet by the RTPS Reader represented by the 
	/// ReaderProxy.
	pub fn unacked_changes(&self) -> Vec<&ChangeForReader> {
		self.changes_for_reader.iter()
			.filter(|ref cfr| cfr.status == ChangeForReaderStatusKind::UNACKNOWLEDGED)
			.collect()
	}

	/// This operation changes the ChangeForReader status of a set of changes 
	/// for the reader represented by ReaderProxy ‘the_reader_proxy.’ The set 
	/// of changes with sequence number smaller than or equal to the value 
	/// ‘committed_seq_num’ have their status changed to ACKNOWLEDGED.
	pub fn acked_changes_set(&mut self, committed_seq_num: SequenceNumber_t) {
		for cfr in &mut self.changes_for_reader {
			if cfr.change.sequence_number <= committed_seq_num {
				cfr.status = ChangeForReaderStatusKind::ACKNOWLEDGED;
			}
		}
	}

	/// This operation modifies the ChangeForReader status of a set of changes 
	/// for the RTPS Reader represented by ReaderProxy ‘this.’ The set of 
	/// changes with sequence numbers ‘req_seq_num_set’ have their status 
	/// changed to REQUESTED.
	pub fn requested_changes_set(&mut self, req_seq_num_set: &[SequenceNumber_t]) {
		for cfr in &mut self.changes_for_reader {
			let r = req_seq_num_set.iter()
						.find(|sn| cfr.change.sequence_number == **sn);

			if r.is_some() == true {
				cfr.status = ChangeForReaderStatusKind::REQUESTED;
			}
		}
	}

	/// This operation returns the subset of changes for the ReaderProxy that 
	/// have status ‘REQUESTED.’ This represents the set of changes that were 
	/// requested by the RTPS Reader represented by the ReaderProxy using an 
	/// ACKNACK Message.
	pub fn requested_changes(&self) -> Vec<&ChangeForReader> {
		self.changes_for_reader.iter()
			.filter(|ref cfr| cfr.status == ChangeForReaderStatusKind::REQUESTED)
			.collect()
	}

	/// This operation returns the CacheChange for the ReaderProxy that has the 
	/// lowest sequence number among the changes with status ‘UNSENT.’ This 
	/// represents the next change that should be sent to the RTPS Reader 
	/// represented by the ReaderProxy.
	pub fn next_unsent_change(&self) -> Option<&ChangeForReader> {
		self.changes_for_reader.iter()
			.find(|ref cfr| cfr.status == ChangeForReaderStatusKind::UNSENT)
	}

	/// This operation returns the ChangeForReader for the ReaderProxy that has
	/// the lowest sequence number among the changes with status ‘REQUESTED.’ 
	/// This represents the next repair packet that should be sent to the RTPS 
	/// Reader represented by the ReaderProxy in response to a previous AckNack
	/// message (see 8.3.7.1) from the Reader.
	pub fn next_requested_change(&self) -> Option<&ChangeForReader> {
		self.changes_for_reader.iter()
			.find(|ref cfr| cfr.status == ChangeForReaderStatusKind::REQUESTED)
	}
}

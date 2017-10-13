// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
  Contains all definitions and types necessary to implement the RTPS wire 
  protocol, used by the DDS DCPS module. RTPS is mainly subdivided into the
  entity and messages modules.  Please see those module docs more more details
  (and, as always, you can refer to the RTPS 2.2 specifiction, found on OMGs
  website).
 */
use std::ops::AddAssign;
use bytes::Bytes;
use rtps::messages::submessage::Time_t;

pub mod entity;
pub mod messages;
pub mod cache_change;
pub mod history_cache;

/// Two-byte value identifying the DDS vender and/or implementation,
/// (i.e. OCI, RTI, PrismTech, Thales)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VendorId_t(u16);
pub const VENDORID_UNKNOWN: VendorId_t = VendorId_t(0);

/// 64bit (signed) sequence number
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SequenceNumber_t(i64);
pub const SEQUENCE_NUMBER_UNKNOWN: SequenceNumber_t = SequenceNumber_t(-1);

impl From<i64> for SequenceNumber_t {
	fn from(sn: i64) -> Self {SequenceNumber_t(sn)}
}

impl AddAssign for SequenceNumber_t {
	fn add_assign(&mut self, rhs: SequenceNumber_t) {
		self.0 += rhs.0
	}
}


/// Used to express the IP address and port of a remote endpoint.
pub enum Locator_t {
	Invalid(i32),
	UDP_V4 {port: u16, address: [u8; 4]},
	UDP_V6 {port: u16, address: [u8; 16]},
}
pub const LOCATOR_INVALID: Locator_t = 		Locator_t::Invalid(-1);

impl Locator_t {
	pub fn size(&self) -> u16 {
		match *self {
			Locator_t::UDP_V4{..} => 10 as u16,
			Locator_t::UDP_V6{..} => 22 as u16,
			Locator_t::Invalid(_) => 4 as u16,
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TopicKind_t {
	NO_KEY = 1,
	WITH_KEY = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChangeKind_t {
	ALIVE,
	NOT_ALIVE_DISPOSED,
	NOT_ALIVE_UNREGISTERED,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReliabilityKind_t {
	BEST_EFFORT = 1,
	RELIABLE = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InstanceHandle_t(i64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProtocolVersion_t {
	major: u8,
	minor: u8,
}
pub const PROTOCOL_VERSION_1_0: ProtocolVersion_t = ProtocolVersion_t {major: 1, minor: 0};
pub const PROTOCOL_VERSION_1_1: ProtocolVersion_t = ProtocolVersion_t {major: 1, minor: 1};
pub const PROTOCOL_VERSION_2_0: ProtocolVersion_t = ProtocolVersion_t {major: 2, minor: 0};
pub const PROTOCOL_VERSION_2_1: ProtocolVersion_t = ProtocolVersion_t {major: 2, minor: 1};
pub const PROTOCOL_VERSION_2_2: ProtocolVersion_t = ProtocolVersion_t {major: 2, minor: 2};
pub const PROTOCOL_VERSION: ProtocolVersion_t = PROTOCOL_VERSION_2_2;

pub type Data = Bytes;
pub type ParticipantMessageData = Bytes;

pub type Duration_t = Time_t;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChangeForReaderStatusKind {
	UNSENT,
	UNACKNOWLEDGED,
	REQUESTED,
	ACKNOWLEDGED,
	UNDERWAY,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChangeFromWriterStatusKind {
	LOST,
	MISSING,
	RECEIVED,
	UNKNOWN,
}

pub mod guid {
/*!
 Used to uniquely identify all RTPS entities.  All entities on the same 
 physical node share the same upper 12 byes of the 16 byte GUID.  The next 3 
 bytes, uniquely identify the entity within the node, and the last byte 
 represents the 'kind' of entitiy.
*/
use rtps::*;
use rand::{Rng, thread_rng};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GUID_t {
	prefix: GUIDPrefix_t,
	entity_id: EntityId_t,
}
pub const GUID_UNKNOWN: GUID_t = GUID_t {prefix: GUIDPREFIX_UNKNOWN, entity_id: ENTITYID_UNKNOWN};

impl GUID_t {
	/// Create a GUID from a prefix and entity_id.
	///
	/// GUID's are created in a very specific way for DDS and RTPS
	/// (see RTPS v2.2 section 9.3.1). The prefix is ranomly generated once
	/// for the participant and all entities within the participant share the
	/// same prefix. EntityId_t(s) are partialy randomly generated, where it's 
	/// `EntityKind_t` is encoded in the last byte of it's four-byte value.
	pub fn new (prefix_val: GUIDPrefix_t, entity_id_val: EntityId_t) -> GUID_t {
		GUID_t {prefix:prefix_val, entity_id:entity_id_val}
	}

	pub fn size (&self) -> u16 {
		16 as u16
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GUIDPrefix_t {
	vendor_id: VendorId_t,
	bytes: [u8; 10],
}
pub const GUIDPREFIX_UNKNOWN: GUIDPrefix_t = GUIDPrefix_t {vendor_id: VENDORID_UNKNOWN, bytes: [0; 10]};

impl GUIDPrefix_t {
	/// Genreate a unique GUID prefix.
	///
	/// According to RTPS v2.2 (section 8.2.4.1), all RTPS/DDS entities running
	/// on the same node should share the same GUID prefix (i.e. the first 12
	/// bytes of the GUID). Entities within the node/participant have a unique
	/// `EntityId_t`.
	/// 
	/// When generating multiple GUIDs for the same participant, be sure to 
	/// reuse the generated `GUIDPrefix_t` and not call this again.
	pub fn new () -> GUIDPrefix_t {
		// Generate a unique prefix.
		let mut b = [0u8; 10];
		thread_rng().fill_bytes(&mut b);

		// VenderId isn't set yet, because it has to be officially allocated by
		// the OMG.  Just use 0's for now.
		GUIDPrefix_t {vendor_id: VENDORID_UNKNOWN, bytes: b}
	}

	pub fn size (&self) -> u16 {
		12 as u16
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityId_t {
	key: [u8; 3],
	kind: EntityKind_t,
}

impl EntityId_t {
	/// Generate a new `EntityId_t`, given an `EntityKind_t`
	///
	/// `EntityId_t`'s must be generated whenever a new reader or writer is created
	/// within the participant. Usually, these are created whenever new DDS data
	/// writers or data readers are being created.  On initializaion of the 
	/// framework, builtin entities are created, using predefined `EntityId_t`(s).
	pub fn new (kind_val: EntityKind_t) -> EntityId_t {
		// Genretate key in such a way as it does not conflict with predefined
		// EntityId_t(s)
		let k1: u8 = thread_rng().gen_range(0, 255);
		let k2: u8 = thread_rng().gen_range(3, 255);
		let k3: u8 = thread_rng().gen_range(5, 255);
		let key_val = [k1, k2, k3];

		EntityId_t {key: key_val, kind: kind_val}
	}

	pub fn size (&self) -> u16 {
		4 as u16
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityKind_t(u8);

pub const ENTITY_KIND_WRITER_WITH_KEY: EntityKind_t =			EntityKind_t(0x02);
pub const ENTITY_KIND_WRITER_NO_KEY: EntityKind_t =				EntityKind_t(0x03);
pub const ENTITY_KIND_READER_NO_KEY: EntityKind_t = 			EntityKind_t(0x04);
pub const ENTITY_KIND_READER_WITH_KEY: EntityKind_t =			EntityKind_t(0x07);
pub const ENTITY_KIND_PARTICIPANT_BUILT_IN: EntityKind_t =		EntityKind_t(0xc1);
pub const ENTITY_KIND_WRITER_WITH_KEY_BUILT_IN: EntityKind_t = 	EntityKind_t(0xc2);
pub const ENTITY_KIND_WRITER_NO_KEY_BUILT_IN: EntityKind_t =	EntityKind_t(0xc3);
pub const ENTITY_KIND_READER_NO_KEY_BUILT_IN: EntityKind_t =	EntityKind_t(0xc4);
pub const ENTITY_KIND_READER_WITH_KEY_BUILT_IN: EntityKind_t = 	EntityKind_t(0xc7);

impl EntityKind_t {
	/// Creates a new user-defined `EntityKind_t` which ensures the value follows
	/// the RTPS v2.2 specification (see section 9.3.1.2).
	pub fn new (kind: u8) -> EntityKind_t {
		EntityKind_t(0x3F & kind)
	}
}

// EntityId_t(s) reserved by the framework.
pub const ENTITYID_UNKNOWN:	EntityId_t =									EntityId_t {key: [0, 0, 0], kind: EntityKind_t(0x00)};
pub const ENTITYID_PARTICIPANT: EntityId_t =								EntityId_t {key: [0, 0, 1], kind: EntityKind_t(0xc1)};
pub const ENTITYID_SEDP_BUILTIN_TOPIC_WRITER: EntityId_t = 					EntityId_t {key: [0, 0, 2], kind: EntityKind_t(0xc2)};
pub const ENTITYID_SEDP_BUILTIN_TOPIC_READER: EntityId_t =					EntityId_t {key: [0, 0, 2], kind: EntityKind_t(0xc7)};
pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_WRITER: EntityId_t =			EntityId_t {key: [0, 0, 3], kind: EntityKind_t(0xc2)};
pub const ENTITYID_SEDP_BUILTIN_PUBLICATIONS_READER: EntityId_t =			EntityId_t {key: [0, 0, 3], kind: EntityKind_t(0xc7)};
pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_WRITER: EntityId_t =			EntityId_t {key: [0, 0, 4], kind: EntityKind_t(0xc2)};
pub const ENTITYID_SEDP_BUILTIN_SUBSCRIPTIONS_READER: EntityId_t =			EntityId_t {key: [0, 0, 4], kind: EntityKind_t(0xc7)};
pub const ENTITYID_SEDP_BUILTIN_PARTICIPANT_WRITER: EntityId_t = 			EntityId_t {key: [0, 1, 0], kind: EntityKind_t(0xc2)};
pub const ENTITYID_SEDP_BUILTIN_PARTICIPANT_READER: EntityId_t = 			EntityId_t {key: [0, 1, 0], kind: EntityKind_t(0xc7)};
pub const ENTITYID_SEDP_BUILTIN_PARTICIPANT_MESSAGE_WRITER: EntityId_t = 	EntityId_t {key: [0, 2, 0], kind: EntityKind_t(0xc2)};
pub const ENTITYID_SEDP_BUILTIN_PARTICIPANT_MESSAGE_READER: EntityId_t = 	EntityId_t {key: [0, 2, 0], kind: EntityKind_t(0xc7)};
}
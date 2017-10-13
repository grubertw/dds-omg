// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 Contains all data structures used for RTPS submessage elements. 
 */
use rtps::*;
use rtps::guid::*;
use rtps::messages::submessage::*;
use bytes::Bytes;

/// Encoded as a 4-byte value.  
/// Key = bytes 0..2 
/// Kind = byte 3. 
pub type EntityId = EntityId_t;

/// Encoded as a 12 byte value.
/// venderId = bytes 0..1
/// key = bytes 2..11
pub type GUIDPrefix = GUIDPrefix_t;

/// Encoded as a 2 byte value.
pub type VendorId = VendorId_t;

impl VendorId {
	pub fn size (&self) -> u16 {
		2 as u16
	}
}

/// States the RTPS protocol revision.
///
/// Encoded as 2 bytes.
/// major = byte 0
/// minor = byte 1
pub type ProtocolVersion = ProtocolVersion_t;

impl ProtocolVersion {
	pub fn size (&self) -> u16 {
		2 as u16
	}
}

/// Every submessage gets a sequence number, incremented by the the RTPS writer.
///
/// Encoded as 8 bytes (signed 64bit long)
pub type SequenceNumber = SequenceNumber_t;

impl SequenceNumber {
	pub fn size (&self) -> u16 {
		8 as u16
	}
}

/// Efficient wire representation of a range of sequence numbers, usually used
/// to request a batch of missing sequence numbers (i.e. submessages sent by 
/// the writer and never received by the reader).  
///
/// Uses a compact bitmap representation for sequence numbers. As such, the 
/// range must be from 0...255 of the base sequnce number (i.e. bitmap_base).
/// The sequnce number is included in the set if the bit shows up within the 
/// bitmap vector.  The value of the sequnce number depends on where the bit 
/// appears in the vector (of u32's).  
///
/// For example, lets say the bitmap base = 424242. If there is a bit set in 
/// bitmap[2] = 0000 0000 0000 0000 0000 0001 0000 0000, then sequence number
/// 424346 is in the set (424242 + ((32*3) + 8)).
/// 
/// During serial encoding, there is a num_bits field, represented as a u32. 
/// This signifies how many bit positions are used within the bitmap vector.
/// Note: this value is not necessarilty divisable by 32, as the most 
/// significant bit may be somewhere in the middle of a u32.  
/// Note: this value also represents the size of the bitmap vector:
/// bitmap.len() = num_bits/32.
pub struct SequenceNumberSet {
	bitmap_base: SequenceNumber_t,
	bitmap: Vec<u32>,
}

impl SequenceNumberSet {
	pub fn size (&self) -> u16 {
		8 + (self.bitmap.len() * 4) as u16
	}
}

/// Use to identify data broken into fragments.
///
/// Encoded as 4 bytes (u32).
pub type FragmentNumber = FragmentNumber_t;

impl FragmentNumber {
	pub fn size (&self) -> u16 {
		4 as u16
	}
}

/// Efficient wire representation of fragment numbers, usually used to request
/// a batch of missing data fragments.
///
/// Uses the same compact bitmap representation scheme employed by 
/// SequenceNumberSet (also see RTPS v2.2 spec 9.4.2.6 and 9.4.2.8)
pub struct FragmentNumberSet {
	bitmap_base: FragmentNumber_t,
	bitmap: Vec<u32>,
}

impl FragmentNumberSet {
	pub fn size (&self) -> u16 {
		4 + (self.bitmap.len() * 4) as u16
	}
}

/// Used to timestamp the message.
///
/// Encoded as 8 bytes.
/// seconds = 4 bytes (i32)
/// fraction = 4 bytes (u32)
pub type Timestamp = Time_t;

impl Timestamp {
	pub fn size (&self) -> u16 {
		8 as u16
	}
}

/// A list of locators over the wire.
///
/// preceded by the list is num_locators, represented as an unsiged long (u32).
/// within Locator_t, the kind is encoded as i32, the port as u32, and the address,
/// which is either a u32 (IPv4), or 16 bytes (IPv6).
pub struct LocatorList(Vec<Locator_t>);

impl LocatorList {
	pub fn size (&self) -> u16 {
		self.0.iter().fold(0, |len, ll| len + ll.size())
	}
}

/// Used to encapsulate QoS parameters.
///
/// Becuase every parameter_id must start on a 4 byte boundry, the length field
/// must be a multiple of 4.
pub struct Parameter {
	parameter_id: ParameterId_t,
	length: i16,
	value: Vec<u8>,
}
/// Used to enforce alignment of parameter list data on a 4 byte boundry.
pub const PID_PAD: ParameterId_t = 		ParameterId_t(0);
/// Used to terminate the parameter list.
pub const PID_SENTINEL: ParameterId_t = ParameterId_t(1);

impl Parameter {
	pub fn size (&self) -> u16 {
		4 + self.value.len() as u16
	}
}

pub struct ParameterList(Vec<Parameter>);

impl ParameterList {
	pub fn size (&self) -> u16 {
		self.0.iter().fold(0, |len, p| len + p.size())
	}
}

/// Representation of serialized payload data within a message segment.
///
/// According to CDR, primitive types must be alligned to to their length
/// (i.e. an i32 must start on a 4 byte boundry).  RTPS does not interpret this
/// data, leaving it to code genereated by the IDE compiler.
///
/// Uses a byte buffer implementation from the (external) bytes crate. 
pub type SerializedPayload = Bytes;

/// Allows the receiver to detect multiple of the same submessage.
///
/// Encoded as 4 bytes (i32).
pub type Count = Count_t;

impl Count {
	pub fn size (&self) -> u16 {
		4 as u16
	}
}
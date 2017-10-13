// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 Contains all data structures used for RTPS message headers. 
 */
use rtps::*;
use rtps::guid::*;

/// Inserted as the first bytes of any UDP datagram. 
///
/// Since this implementation is only for UDP, there is no length field, which
/// specified in the UDP header itself. The protocol_id is a 4 byte value, 
/// which shall always contain the contents 'R' 'T' 'P' 'S' in ascii. The 
/// protocol version used depends on the PROTOCOL_VERSION definition.  The 
/// vender ID represents a unique ID for this spific library (assigned by the
/// OMG). The GUID prefix is a 12 byte value which allows all other GUIDs 
/// within the messsage to be expressed as 4 byte entity IDs.
pub struct Header {
	protocol_id: [u8; 4],
	protocol_version: ProtocolVersion_t,
	vendor_id: VendorId_t,
	guid_prefix: GUIDPrefix_t,
}

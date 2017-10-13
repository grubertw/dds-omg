// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 RTPS Participant is the container of RTPS Endpoint entities and maps to a DDS DomainParticipant. In addition, the RTPS Participant facilitates the fact that the RTPS Endpoint entities within a single RTPS Participant are likely to share common properties.
 */
use std::sync::Arc;
use rtps::*;
use rtps::entity::Entity;
use rtps::entity::endpoint::Endpoint;

pub trait Participant: Entity {
	/// Default list of unicast locators (transport, address, port combinations) 
	/// that can be used to to send messages to the endpoint. These are the locators
	/// used if there are none specified by the endpoint.
	fn default_unicast_locator_list(&self) -> Vec<&Locator_t>;

	/// Default list of multicast locators (transport, address, port combinations) 
	/// that can be used to to send messages to the endpoint. These are the locators
	/// used if there are none specified by the endpoint.
	fn default_multicast_locator_list(&self) -> Vec<&Locator_t>;

	/// Identifies the RTPS protocol version the participant uses to communicate.
	fn protocol_version(&self) -> &ProtocolVersion_t;

	/// Identifies the vendor of the middleware supporting the RTPS participant. 
	fn vendor_id(&self) -> &VendorId_t;

	/// Lists all endpoints contained within this participant.
	fn endpoints(&self) -> Vec<Arc<Endpoint>>;
}
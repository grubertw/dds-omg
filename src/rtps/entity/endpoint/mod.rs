// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
RTPS Endpoint represents the possible communication endpoints from the point of view of the RTPS protocol. There are two kinds of RTPS Endpoint entities: Writer endpoints and Reader endpoints.
RTPS Writer endpoints send CacheChange messages to RTPS Reader endpoints and potentially receive acknowledgments for the changes they send. RTPS Reader endpoints receive CacheChange and change-availability announcements from Writer endpoints and potentially acknowledge the changes and/or request missed changes.
 */
mod reader;
mod writer;

use std::sync::Arc;
use rtps::*;
use rtps::entity::Entity;
use rtps::entity::participant::Participant;

pub trait Endpoint: Entity {
	/// List of unicast locators (transport, address, port combinations) 
	/// that can be used to to send messages to the endpoint. The list may 
	/// be empty.
	fn unicast_locator_list(&self) -> &Vec<Locator_t>;

	/// List of multicast locators (transport, address, port combinations) 
	/// that can be used to to send messages to the endpoint. The list may 
	/// be empty.
	fn multicast_locator_list(&self) -> &Vec<Locator_t>;

	/// The levels of reliability supported by the endpoint.
	fn reliability_level(&self) -> &ReliabilityKind_t;

	/// Used to indicate whether the endpoint is associated with a datatype
	/// that has defined some fields as containing the DDS key.
	fn topic_kind(&self) -> &TopicKind_t;

	/// Get the RTPS participant containing this Endpoint.
	fn participant(&self) -> Arc<Participant>;
}
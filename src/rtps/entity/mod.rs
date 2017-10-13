// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
 Contains all definitions and data types used for RTPS entities. 

 Entity is the top level structure representing any 'actor' within RTPS. 
 */
mod endpoint;
mod participant;

use rtps::guid::*;

pub trait Entity {
	/// Globally and uniquely identifies the RTPS Entity within the DDS domain.
	///
	/// Please see the docs in the guid module which describes the structure of a GUID within DDS.
	fn guid(&self) -> &GUID_t;
}
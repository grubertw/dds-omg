// Copyright © Travis Gruber 2017, All Rights Reserved.
/*! 
  This library seeks to implement the Data Distribution Serivce middleware
  Standardized by the Open Management Group (OMG). DDS has a long history 
  in the industrial automation industry, aviation and flight control, and
  other technologies that are now being classified as "Internet of Things".
  Being an open standard, anyone (including myself) can implement the wire
  protocol and seek to support a minimal feature set, along with whatever 
  the application may need to do it's job. A question that may immeadiatly
  come to mind is: "Why use a big framework when I don't need it?". The answer
  is always: "it depends". If the application involves networking, with any
  kind of communications that involve frequent changes, such as a sensor that
  takes routine measurments that more than one viewier wishes to see, then DDS
  may be a very good option.
 
  This library is split into three major parts: RTPS (real-time publish/
  subscribe), DCPS (Data Centric Publish/Subscribe), and the IDL compiler. 
  Please see module documentation for more in-depth descriptions.
 
  For general usage of the library, please see the README, as well as the 
  samples module. The README also lists other good resources for using/
  understanding DDS. 
 
 
  Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
  http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
  <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
  option. This file may not be copied, modified, or distributed
  except according to those terms.
 */
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate rand;
extern crate bytes;
extern crate futures;
extern crate tokio_core;

// See docs within module for more detail.
pub mod dcps;

// See docs within module for more detail.
pub mod rtps;


#[cfg(test)]
mod tests;
//#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_html_tags)]
#![allow(dead_code)]

//! Little CMS, Remade in Rust
//!
//! Taking from the original README file:
//! # About Little CMS
//!
//! Little CMS intends to be an OPEN SOURCE small-footprint color management engine, with special focus on accuracy and
//! performance. It uses the International Color Consortium standard (ICC), which is the modern standard when regarding
//! to color management. The ICC specification is widely used and is referred to in many International and other
//! de-facto standards. It was approved as an International Standard, ISO 15076-1, in 2005.
//!
//! # Conformance
//!
//! Little CMS is a FULL IMPLEMENTATION of ICC specification 4.3, it fully supports all kind of V2 and V4 profiles,
//! including abstract, devicelink and named color profiles. Check the tutorial for a exhaustive list of features.
//!
//! # A bit of story
//!
//! Since the initial release, back in 1998, Little CMS has grown to become one of the most popular open-source color
//! management libraries, and has been used in a large number of production projects, in areas as printer firmware,
//! monitors, digital cameras, RIPs, publishing, scientific, and many others. You can find Little CMS in most Linux
//! distributions, and it's released under an open source license.
//!

#[allow(unused_macros)]
macro_rules! use_big_endian {
    () => {
        !cfg!(feature = "use_little_endian")
    };
}

// Fixed Point Types

/// Fixed point [UQ8.8](https://en.wikipedia.org/wiki/Q_(number_format)) Number
pub type U8F8 = u16;
/// Fixed point [Q15.16](https://en.wikipedia.org/wiki/Q_(number_format)) Number
pub type S15F16 = i32;
/// Fixed point [UQ16.16](https://en.wikipedia.org/wiki/Q_(number_format)) Number
pub type U16F16 = u32;

pub mod io;
pub mod plugins;
pub mod state;
pub mod types;

/// The version/release of lcms2 implemented. (2.13.1)
pub const LCMS_VERSION: u32 = 2131;

#[cfg(test)]
mod testing;

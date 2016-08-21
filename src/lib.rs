#![feature(unicode)]

//! # XmlWriter
//! This crate is to write xml in the probably most efficient way, by writing directly to the stream,
//! without any DOM or other intermediate structures. It strives to be zero allocation.

#![deny(missing_docs)]

mod xml_writer;

pub use xml_writer::XmlWriter;

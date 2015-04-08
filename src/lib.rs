//#![feature(std_misc)]
//#![feature(core)]

pub use geolocation::GeoLocation;
pub use boundingbox::BoundingBox;
pub use geohash::{BinaryHash, encode, decode, neighbor, neighbors};

mod geolocation;
mod boundingbox;
mod geohash;
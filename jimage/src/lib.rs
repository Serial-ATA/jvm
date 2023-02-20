#![feature(let_chains)]

mod decompressor;
mod header;
mod image;
mod index;
mod location;
mod strings;

pub use header::{
	JImageHeader, JIMAGE_MAGIC, JIMAGE_MAGIC_INVERTED, JIMAGE_MAJOR_VERSION, JIMAGE_MINOR_VERSION,
};
pub use image::{JImage, JImageBuilder};
pub use index::JImageIndex;
pub use location::JImageLocation;
pub use strings::ImageStrings;

#![feature(let_chains)]

mod decompressor;
pub mod error;
mod header;
mod image;
mod index;
mod location;
mod parse;
mod strings;

pub use header::{
	JIMAGE_MAGIC, JIMAGE_MAGIC_INVERTED, JIMAGE_MAJOR_VERSION, JIMAGE_MINOR_VERSION, JImageHeader,
};
pub use image::JImage;
pub use index::JImageIndex;
pub use location::JImageLocation;
pub use strings::ImageStrings;

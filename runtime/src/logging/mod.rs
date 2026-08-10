mod macros;
pub(crate) use macros::*;
mod write;
pub(crate) use write::__write; // For the `log!` macro

use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::sync::atomic::Atomic;

macro_rules! log_levels {
    (pub enum LogLevel {
        $(
        $(#[$meta:meta])*
        $variant:ident = $s:literal
        ),* $(,)?
    }) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
        pub enum LogLevel {
            $(
                $(#[$meta])*
                $variant,
            )*
        }

        impl FromStr for LogLevel {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $s => Ok(LogLevel::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }

        impl AsRef<str> for LogLevel {
            fn as_ref(&self) -> &str {
                match self {
                    $(LogLevel::$variant => $s,)*
                }
            }
        }
    }
}

log_levels! {
	pub enum LogLevel {
		Off = "off",
		Error = "error",
		Warning = "warning",
		#[default]
		Info = "info",
		Debug = "debug",
		Trace = "trace",
	}
}

impl Display for LogLevel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.pad(self.as_ref())
	}
}

macro_rules! tags {
    (
    pub enum $tag_enum:ident {
        $(
        $(#[$meta:meta])*
        $variant:ident = $s:literal
        ),* $(,)?
    }
    ) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub enum $tag_enum {
            $(
            $(#[$meta])*
            $variant,
            )*
            All,
        }

        impl AsRef<str> for $tag_enum {
            fn as_ref(&self) -> &str {
                match self {
                    $($tag_enum::$variant => $s,)*
                    $tag_enum::All => "all",
                }
            }
        }

        impl FromStr for $tag_enum {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($s => Ok($tag_enum::$variant),)*
                    _ => Err(()),
                }
            }
        }
    }
}

tags! {
	pub enum Tag {
		Class = "class",
		Init = "init",
		Exceptions = "exceptions",
	}
}

impl Tag {
	/// The number of tags available, excluding `all`.
	pub const VARIANTS: u8 = Tag::All as u8;
}

impl Display for Tag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

/// A collection of [`Tag`]s.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct TagSet(u16);

impl TagSet {
	/// A set containing all log tags.
	pub const ALL: Self = Self({
		let mut bits = 0;
		let mut i = 0;
		while i < Tag::All as u8 {
			bits |= 1 << i;
			i += 1;
		}
		bits
	});

	pub const fn new(tags: &[Tag]) -> Self {
		let mut set = 0;
		let mut idx = 0;
		while idx < tags.len() {
			set |= 1 << (tags[idx] as u8);
			idx += 1;
		}
		Self(set)
	}

	pub const fn insert(&mut self, tag: Tag) {
		self.0 |= 1 << (tag as u8);
	}

	/// Checks if the tag is contained in the set.
	pub fn contains(&self, tag: Tag) -> bool {
		self.0 & (1 << (tag as u8)) != 0
	}

	/// The number of [`Tag`]s in the set.
	pub fn len(&self) -> usize {
		self.iter().count()
	}

	pub fn iter(&self) -> impl Iterator<Item = Tag> {
		unsafe { iter_tag_set::<Tag>(self.0) }
	}
}

impl Display for TagSet {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut tags = String::new();
		let len = self.iter().count();

		for (idx, tag) in self.iter().enumerate() {
			tags.push_str(tag.as_ref());
			if idx != len - 1 {
				tags.push(',');
			}
		}

		f.pad(&tags)
	}
}

impl FromIterator<Tag> for TagSet {
	fn from_iter<T: IntoIterator<Item = Tag>>(iter: T) -> Self {
		let mut set = TagSet::default();
		for tag in iter {
			set.insert(tag);
		}
		set
	}
}

unsafe fn iter_tag_set<T>(mut bits: u16) -> impl Iterator<Item = T> {
	std::iter::from_fn(move || {
		if bits == 0 {
			return None;
		}

		let tag = unsafe {
			std::mem::transmute_prefix::<u8, T>(
				u8::try_from(bits.trailing_zeros())
					.expect("the maximum tag value should be less than 16"),
			)
		};

		bits ^= bits & bits.overflowing_neg().0;
		Some(tag)
	})
}

impl Debug for TagSet {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

/// Decorators to prefix log events with.
///
/// NOTE: **THE VARIANT ORDER IS SIGNIFICANT**. The order they appear here is the order they will
///       appear in the log output.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LogDecorator {
	Uptime,
	Level,
	Tags,
	// TODO: time, utctime, timemillis, uptimemillis, timenanos, uptimenanoes, hostname, pid, tid
	All,
}

impl LogDecorator {
	/// The number of tags available, excluding `all`.
	pub const VARIANTS: u8 = LogDecorator::All as u8;
}

impl FromStr for LogDecorator {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"uptime" => Ok(LogDecorator::Uptime),
			"level" => Ok(LogDecorator::Level),
			"tags" => Ok(LogDecorator::Tags),
			_ => Err(()),
		}
	}
}

/// A collection of [`LogDecorator`]s.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LogDecoratorSet(u16);

impl LogDecoratorSet {
	/// An empty set of decorators.
	pub const EMPTY: LogDecoratorSet = LogDecoratorSet(0);
	/// The implied decorators when none are provided on the CLI.
	pub const DEFAULT: LogDecoratorSet = LogDecoratorSet::new(&[
		LogDecorator::Uptime,
		LogDecorator::Level,
		LogDecorator::Tags,
	]);

	pub const fn new(decorators: &[LogDecorator]) -> Self {
		let mut set = 0;
		let mut idx = 0;
		while idx < decorators.len() {
			set |= 1 << (decorators[idx] as u8);
			idx += 1;
		}
		Self(set)
	}

	pub fn is_empty(&self) -> bool {
		*self == Self::EMPTY
	}

	pub fn insert(&mut self, decorator: LogDecorator) {
		self.0 |= 1 << (decorator as u8);
	}

	pub fn union(&mut self, other: LogDecoratorSet) -> LogDecoratorSet {
		LogDecoratorSet(self.0 | other.0)
	}

	pub fn iter(&self) -> impl Iterator<Item = LogDecorator> {
		unsafe { iter_tag_set::<LogDecorator>(self.0) }
	}
}

impl Debug for LogDecoratorSet {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

#[derive(Debug)]
pub struct LogDecoratorContext {
	pub decorators: LogDecoratorSet,
	widths: [Atomic<u8>; LogDecorator::VARIANTS as usize],
}

impl PartialEq for LogDecoratorContext {
	fn eq(&self, other: &Self) -> bool {
		self.decorators == other.decorators
	}
}

impl Eq for LogDecoratorContext {}

impl LogDecoratorContext {
	pub fn new(decorators: LogDecoratorSet) -> Self {
		Self {
			decorators,
			widths: core::array::from_fn(|_| Atomic::default()),
		}
	}
}

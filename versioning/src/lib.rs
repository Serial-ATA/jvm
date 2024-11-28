pub mod error;

use crate::error::VersionError;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

mod patterns {
	use std::sync::LazyLock;

	use const_format::concatcp;
	use regex::Regex;

	const VNUM_PATTERN_STRING: &str = "(?<VNUM>[1-9][0-9]*(?:(?:\\.0)*\\.[1-9][0-9]*)*)";
	const PRE_PATTERN_STRING: &str = "(?:-(?<PRE>[a-zA-Z0-9]+))?";
	const BUILD_PATTERN_STRING: &str = "(?:(?<PLUS>\\+)(?<BUILD>0|[1-9][0-9]*)?)?";
	const OPT_PATTERN_STRING: &str = "(?:-(?<OPT>[-a-zA-Z0-9.]+))?";
	const VSTR_FORMAT: &str = concatcp!(
		VNUM_PATTERN_STRING,
		PRE_PATTERN_STRING,
		BUILD_PATTERN_STRING,
		OPT_PATTERN_STRING
	);

	pub static VSTR_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(VSTR_FORMAT).unwrap());
}

/// A representation of a version string for an implementation of the Java SE Platform.
///
/// The format of a version string is as follows:
/// ```regex
/// [1-9][0-9]*((\.0)*\.[1-9][0-9]*)*
/// ```
/// The length of this section is unbounded, but the first four elements are assigned special meanings:
/// ```text
/// <FEATURE>.<INTERIM>.<UPDATE>.<PATCH>
/// ```
///
/// * Feature (**REQUIRED**):<br />
///     The feature-release counter, incremented for every feature release regardless of content.
///
/// * Interim (**OPTIONAL**):<br />
///     The interim-release counter, incremented for non-feature releases contain compatible bug-fixes
///     and enhancements but no incompatible changes, no feature removals, and no changes to standard APIs.
///
/// * Update (**OPTIONAL**):<br />
///     The update-release counter, incremented for compatible update releases that fix security issues,
///     regressions and bugs in newer features.
///
/// * Patch (**OPTIONAL**):<br />
///     The emergency path-release counter, incremented only it's necessary to produce an emergency release
///     to fix a critical issue.
///
/// Additional elements:
///
/// * Pre-release identifier (`[a-zA-Z0-9]+`):<br />
///     Typically `ea`, for a potentially unstable early-access release under active development, or `internal`
///     for an internal developer build.
///
/// * Build (`0|[1-9][0-9]*`):<br />
///     The build number, incremented for each promoted build.
///
/// * Opt (`[-a-zA-Z0-9.]+`):<br />
///     Additional build information.
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
/// use versioning::Version;
///
/// # fn main() -> versioning::error::Result<()> {
/// // Simple version number, only contains a feature element
/// let simple_version = Version::from_str("10")?;
///
/// // A longer version number, more likely to be encountered in the wild.
/// // This one contains all 4 elements.
/// let normal_version = Version::from_str("10.0.2.1")?;
///
/// // This version features a pre-release identifier
/// let pre_release_version = Version::from_str("10-ea")?;
/// # Ok(()) }
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Version {
	version: Vec<u32>,
	pre: Option<String>,
	build: Option<u32>,
	optional: Option<String>,
}

impl Version {
	/// Returns the value of the feature element of the version number
	pub fn feature(&self) -> u32 {
		self.version[0]
	}

	/// Returns the values of the interim element of the version number, or zero if it is absent
	pub fn interim(&self) -> u32 {
		self.version.get(1).copied().unwrap_or(0)
	}

	/// Returns the values of the interim element of the version number, or zero if it is absent
	pub fn update(&self) -> u32 {
		self.version.get(2).copied().unwrap_or(0)
	}

	/// Returns the values of the interim element of the version number, or zero if it is absent
	pub fn patch(&self) -> u32 {
		self.version.get(3).copied().unwrap_or(0)
	}

	/// Returns the list of integers represented in the version number.
	///
	/// The list always contains at least one element corresponding to the feature version number.
	pub fn version(&self) -> &[u32] {
		&self.version
	}

	/// Returns the optional pre-release information
	pub fn pre(&self) -> Option<&str> {
		self.pre.as_deref()
	}

	/// Returns the build number
	pub fn build(&self) -> Option<u32> {
		self.build
	}

	/// Returns optional additional identifying build information
	pub fn optional(&self) -> Option<&str> {
		self.optional.as_deref()
	}
}

impl FromStr for Version {
	type Err = VersionError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut simple_number = true;
		for (i, c) in s.chars().enumerate() {
			let lower_bound = if i == 0 { '1' } else { '0' };
			if !(lower_bound..='9').contains(&c) {
				simple_number = false;
				break;
			}
		}

		if simple_number {
			return Ok(Self {
				version: vec![s.parse()?],
				pre: None,
				build: None,
				optional: None,
			});
		}

		let captures = patterns::VSTR_PATTERN
			.captures(s)
			.ok_or(VersionError::NoMatch)?;

		let vnum = captures.name("VNUM").unwrap();
		let mut version = Vec::new();
		for i in vnum.as_str().split('.') {
			version.push(i.parse()?);
		}

		let pre = captures.name("PRE").map(|pre| pre.as_str().to_string());

		let build: Option<u32> = match captures.name("BUILD") {
			Some(build_match) => Some(build_match.as_str().parse()?),
			None => None,
		};

		let optional = captures.name("OPT").map(|pre| pre.as_str().to_string());

		if build.is_none() {
			match captures.name("PLUS") {
				Some(_) => {
					if optional.is_some() && pre.is_some() {
						return Err(VersionError::PlusGroupWithPreAndOptional);
					}

					if optional.is_none() {
						return Err(VersionError::UnMatchedPlusGroup);
					}
				},
				None => {
					if optional.is_some() && pre.is_none() {
						return Err(VersionError::UnMatchedOptional);
					}
				},
			}
		}

		Ok(Self {
			version,
			pre,
			build,
			optional,
		})
	}
}

impl Display for Version {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(pre) = &self.pre {
			f.write_char('-')?;
			f.write_str(pre)?;
		}

		match &self.build {
			Some(build) => {
				f.write_char('+')?;
				f.write_fmt(format_args!("{}", build))?;
				if let Some(optional) = &self.optional {
					f.write_char('-')?;
					f.write_str(optional)?;
				}
			},
			None => {
				if let Some(optional) = &self.optional {
					match self.pre {
						Some(_) => f.write_char('-')?,
						None => f.write_str("+=")?,
					}
					f.write_str(optional)?;
				}
			},
		}

		Ok(())
	}
}

impl PartialOrd for Version {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Version {
	fn cmp(&self, other: &Self) -> Ordering {
		let version_cmp = self.version.len().cmp(&other.version.len());
		if !version_cmp.is_eq() {
			return version_cmp;
		}

		let pre_cmp = self.pre.cmp(&other.pre);
		if !pre_cmp.is_eq() {
			return pre_cmp;
		}

		let build_cmp = self.build.cmp(&other.build);
		if !build_cmp.is_eq() {
			return build_cmp;
		}

		let optional_cmp = self.optional.cmp(&other.optional);
		if !optional_cmp.is_eq() {
			return optional_cmp;
		}

		Ordering::Equal
	}
}

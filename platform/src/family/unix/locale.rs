// Adapted from https://github.com/openjdk/jdk/blob/master/src/java.base/unix/native/libjava/locale_str.h
//
// No conditional compilation here, these iterators are extended in the os-specific dirs.
// For example, see: ./linux/locale.rs

use core::ffi::CStr;

pub(crate) fn base_locale_aliases() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)>
{
	const LOCALE_ALIASES: &[(&CStr, &CStr)] = &[
		(c"ar", c"ar_EG"),
		(c"be", c"be_BY"),
		(c"bg", c"bg_BG"),
		(c"br", c"br_FR"),
		(c"ca", c"ca_ES"),
		(c"cs", c"cs_CZ"),
		(c"cz", c"cs_CZ"),
		(c"da", c"da_DK"),
		(c"de", c"de_DE"),
		(c"el", c"el_GR"),
		(c"en", c"en_US"),
		(c"eo", c"eo"), // no country for Esperanto
		(c"es", c"es_ES"),
		(c"et", c"et_EE"),
		(c"eu", c"eu_ES"),
		(c"fi", c"fi_FI"),
		(c"fr", c"fr_FR"),
		(c"ga", c"ga_IE"),
		(c"gl", c"gl_ES"),
		(c"he", c"iw_IL"),
		(c"hr", c"hr_HR"),
		(c"hu", c"hu_HU"),
		(c"id", c"in_ID"),
		(c"in", c"in_ID"),
		(c"is", c"is_IS"),
		(c"it", c"it_IT"),
		(c"iw", c"iw_IL"),
		(c"ja", c"ja_JP"),
		(c"kl", c"kl_GL"),
		(c"ko", c"ko_KR"),
		(c"lt", c"lt_LT"),
		(c"lv", c"lv_LV"),
		(c"mk", c"mk_MK"),
		(c"nl", c"nl_NL"),
		(c"no", c"no_NO"),
		(c"pl", c"pl_PL"),
		(c"pt", c"pt_PT"),
		(c"ro", c"ro_RO"),
		(c"ru", c"ru_RU"),
		(c"se", c"se_NO"),
		(c"sk", c"sk_SK"),
		(c"sl", c"sl_SI"),
		(c"sq", c"sq_AL"),
		(c"sr", c"sr_CS"),
		(c"su", c"fi_FI"),
		(c"sv", c"sv_SE"),
		(c"th", c"th_TH"),
		(c"tr", c"tr_TR"),
		(c"uk", c"uk_UA"),
		(c"vi", c"vi_VN"),
		(c"wa", c"wa_BE"),
		(c"zh", c"zh_CN"),
	];

	LOCALE_ALIASES.iter()
}

pub(crate) fn base_language_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)>
{
	const LANGUAGE_NAMES: &[(&CStr, &CStr)] = &[
		(c"C", c"en"),
		(c"POSIX", c"en"),
		(c"cz", c"cs"),
		(c"he", c"iw"),
		(c"id", c"in"),
		(c"sh", c"sr"), // sh is deprecated
		(c"su", c"fi"),
	];

	LANGUAGE_NAMES.iter()
}

pub(crate) fn base_script_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)> {
	const SCRIPT_NAMES: &[(&CStr, &CStr)] = &[
		(c"Arab", c"Arab"),
		(c"Cyrl", c"Cyrl"),
		(c"Deva", c"Deva"),
		(c"Ethi", c"Ethi"),
		(c"Hans", c"Hans"),
		(c"Hant", c"Hant"),
		(c"Latn", c"Latn"),
		(c"Sund", c"Sund"),
		(c"Syrc", c"Syrc"),
		(c"Tfng", c"Tfng"),
	];

	SCRIPT_NAMES.iter()
}

pub(crate) fn base_country_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)>
{
	const COUNTRY_NAMES: &[(&CStr, &CStr)] = &[
		(c"YU", c"CS"), // YU has been removed from ISO 3166
	];

	COUNTRY_NAMES.iter()
}

pub(crate) fn base_variant_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)>
{
	const VARIANT_NAMES: &[(&CStr, &CStr)] = &[(c"nynorsk", c"NY")];

	VARIANT_NAMES.iter()
}

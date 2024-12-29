use core::ffi::CStr;

pub(super) fn locale_aliases() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)> {
	const LINUX_LOCALE_ALIASES: &[(&CStr, &CStr)] = &[
		(c"hs", c"en_US"), // used on Linux, not clear what it stands for
		(c"ua", c"en_US"), // used on Linux, not clear what it stands for
		(c"bokmal", c"nb_NO"),
		(c"bokm\xE5cl", c"nb_NO"),
		(c"catalan", c"ca_ES"),
		(c"croatian", c"hr_HR"),
		(c"czech", c"cs_CZ"),
		(c"danish", c"da_DK"),
		(c"dansk", c"da_DK"),
		(c"deutsch", c"de_DE"),
		(c"dutch", c"nl_NL"),
		(c"eesti", c"et_EE"),
		(c"estonian", c"et_EE"),
		(c"finnish", c"fi_FI"),
		(c"fran\xE7c\x61is", c"fr_FR"),
		(c"french", c"fr_FR"),
		(c"galego", c"gl_ES"),
		(c"galician", c"gl_ES"),
		(c"german", c"de_DE"),
		(c"greek", c"el_GR"),
		(c"hebrew", c"iw_IL"),
		(c"hrvatski", c"hr_HR"),
		(c"hungarian", c"hu_HU"),
		(c"icelandic", c"is_IS"),
		(c"italian", c"it_IT"),
		(c"japanese", c"ja_JP"),
		(c"korean", c"ko_KR"),
		(c"lithuanian", c"lt_LT"),
		(c"norwegian", c"no_NO"),
		(c"nynorsk", c"nn_NO"),
		(c"polish", c"pl_PL"),
		(c"portuguese", c"pt_PT"),
		(c"romanian", c"ro_RO"),
		(c"russian", c"ru_RU"),
		(c"slovak", c"sk_SK"),
		(c"slovene", c"sl_SI"),
		(c"slovenian", c"sl_SI"),
		(c"spanish", c"es_ES"),
		(c"swedish", c"sv_SE"),
		(c"thai", c"th_TH"),
		(c"turkish", c"tr_TR"),
	];

	crate::locale::base_locale_aliases().chain(LINUX_LOCALE_ALIASES.into_iter())
}

pub(super) fn language_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)> {
	const LINUX_LANGUAGE_NAMES: &[(&CStr, &CStr)] = &[
		(c"hs", c"en"), // used on Linux, not clear what it stands for
		(c"ua", c"en"), // used on Linux, not clear what it stands for
		(c"catalan", c"ca"),
		(c"croatian", c"hr"),
		(c"czech", c"cs"),
		(c"danish", c"da"),
		(c"dansk", c"da"),
		(c"deutsch", c"de"),
		(c"dutch", c"nl"),
		(c"finnish", c"fi"),
		(c"fran\xE7c\x61is", c"fr"),
		(c"french", c"fr"),
		(c"german", c"de"),
		(c"greek", c"el"),
		(c"hebrew", c"he"),
		(c"hrvatski", c"hr"),
		(c"hungarian", c"hu"),
		(c"icelandic", c"is"),
		(c"italian", c"it"),
		(c"japanese", c"ja"),
		(c"norwegian", c"no"),
		(c"polish", c"pl"),
		(c"portuguese", c"pt"),
		(c"romanian", c"ro"),
		(c"russian", c"ru"),
		(c"slovak", c"sk"),
		(c"slovene", c"sl"),
		(c"slovenian", c"sl"),
		(c"spanish", c"es"),
		(c"swedish", c"sv"),
		(c"turkish", c"tr"),
	];

	crate::locale::base_language_names().chain(LINUX_LANGUAGE_NAMES.into_iter())
}

pub(super) fn script_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)> {
	const LINUX_SCRIPT_NAMES: &[(&CStr, &CStr)] = &[
		(c"cyrillic", c"Cyrl"),
		(c"devanagari", c"Deva"),
		(c"iqtelif", c"Latn"),
		(c"latin", c"Latn"),
	];

	crate::locale::base_script_names().chain(LINUX_SCRIPT_NAMES.into_iter())
}

pub(super) fn country_names() -> impl Iterator<Item = &'static (&'static CStr, &'static CStr)> {
	const LINUX_COUNTRY_NAMES: &[(&CStr, &CStr)] = &[
		(c"RN", c"US"), // used on Linux, not clear what it stands for
	];

	crate::locale::base_country_names().chain(LINUX_COUNTRY_NAMES.into_iter())
}

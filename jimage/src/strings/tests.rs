use crate::ImageStrings;

#[test]
#[ignore]
fn hash_code() {
	assert_eq!(
		16_777_619,
		ImageStrings::hash_code("", ImageStrings::HASH_MULTIPLIER)
	);
	assert_eq!(
		1_213_053_849,
		ImageStrings::hash_code("foo", ImageStrings::HASH_MULTIPLIER)
	);
	assert_eq!(
		977_475_810,
		ImageStrings::hash_code("bar", ImageStrings::HASH_MULTIPLIER)
	);
	assert_eq!(
		468_742_824,
		ImageStrings::hash_code("Hello, World!", ImageStrings::HASH_MULTIPLIER)
	);
	assert_eq!(
		1_641_313_752,
		ImageStrings::hash_code("你好，世界！", ImageStrings::HASH_MULTIPLIER)
	);
	assert_eq!(
		1_798_886_865,
		ImageStrings::hash_code(
			"123456789:一二三四五六七八九",
			ImageStrings::HASH_MULTIPLIER
		)
	);
}

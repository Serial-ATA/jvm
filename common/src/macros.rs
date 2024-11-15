/// Create a boxed slice
///
/// This has the same syntax as the [vec!] macro.
#[macro_export]
macro_rules! box_slice {
	() => {
		Box::new([])
	};
	($default:expr; $size:expr) => {
		(0..$size).map(|_| $default).collect::<Vec<_>>().into_boxed_slice()
	};
    ($($x:expr),+) => {
		Box::new([$($x),+])
	};
}

use crate::logging::{LogLevel, Tag};
use crate::options::logging::{
	LogOption, LogOptions, LogOptionsBuilder, LogOutputName, LogOutputOptions, Selection,
	Selections,
};

use std::str::FromStr;

#[test]
fn jvm_log_options() {
	let expectations: Vec<(&'static str, LogOptions)> = vec![
		("-Xlog", LogOption::default().into()),
		(
			"-Xlog:exceptions=warning",
			LogOption {
				selections: Selections(vec![Selection {
					tag: Tag::Exceptions,
					level: LogLevel::Warning,
				}]),
				..LogOption::default()
			}
			.into(),
		),
		(
			"-Xlog::stderr",
			LogOption {
				output: LogOutputName::Stderr,
				..LogOption::default()
			}
			.into(),
		),
		(
			"-Xlog:::foldmultilines=true",
			LogOption {
				output_options: LogOutputOptions {
					fold_multilines: true,
					..LogOutputOptions::default()
				},
				..LogOption::default()
			}
			.into(),
		),
	];

	for (opt_string, expected) in expectations {
		let mut builder = LogOptionsBuilder::default();
		let option = LogOption::from_str(opt_string).unwrap();
		builder.apply_option(option);
		assert_eq!(builder.build(), expected);
	}
}

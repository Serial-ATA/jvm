#[macro_export]
macro_rules! log {
	(
        TARGETS: ($($tag:ident),+),
        $level:ident,
        $($arg:tt)+
    ) => {{
        let level = $crate::logging::LogLevel::$level;
        let tags = $crate::logging::TagSet::new(&[$($crate::logging::Tag::$tag),*]);
        if $crate::options::logging::LogOptions::get().are_tags_enabled_at(
            tags,
            level,
        ) {
            let message = ::std::format!($($arg)+);
            $crate::logging::__write(
                level,
                tags,
                &message,
            );
        }
    }};
}

/// Check whether the given tags are enabled at the given level.
///
/// # Examples
///
/// ```rust,no_run
/// use jvm::enabled;
///
/// if enabled!(TARGETS: (Exceptions), Info) {
///     // Do expensive exception logging stuff...
/// }
/// ```
#[macro_export]
macro_rules! enabled {
    (
        TARGETS: ($($tag:ident),+),
        $level:ident $(,)?
    ) => {{
        let level = $crate::logging::LogLevel::$level;
        let tags = $crate::logging::TagSet::new(&[$($crate::logging::Tag::$tag),*]);
        $crate::options::logging::LogOptions::get().are_tags_enabled_at(
            tags,
            level,
        )
    }}
}

macro_rules! define_log_level_macros {
	($(($level:ident, $level_camel:ident)),+) => {
		$(
        paste::paste! {
            #[macro_export]
            #[doc = "Print a log message at the `" $level "` level for the given tags"]
            ///
            /// The first argument is a list of [`Tag`]s in the form: `TARGETS: (TAG1 [, TAG2...])`.
            /// The following arguments are identical to a [`println!`] call.
            ///
            /// # Examples
            ///
            /// ```rust
            #[doc = "use jvm::logging::" $level ";"]
            ///
            #[doc = $level "!(TARGETS: (Class, Init), \"Woah, the class {} just initialized!\", class_name)"]
            /// ```
            ///
            /// [`Tag`]: crate::logging::Tag
            macro_rules! [<_ $level>] {
                (TARGETS: ($$($$tag:ident),+), $$($$arg:tt)+) => {
                    $$crate::log!(TARGETS: ($$($$tag),+), $level_camel, $$($$arg)+);
                };
            }

            #[doc(hidden)]
            pub use [<_ $level>] as $level;
        }
        )+
	};
}

define_log_level_macros!(
	(trace, Trace),
	(debug, Debug),
	(info, Info),
	(warn, Warning),
	(error, Error)
);

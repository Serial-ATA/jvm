macro_rules! conditional {
    (
        #[$meta:meta]

        $(
            $item:item
        )+
    ) => {
        $(
            #[$meta]
            $item
        )+
    }
}

macro_rules! match_cfg_meta {
	(
        match cfg($ident:ident) {
            $($pat:literal => { $($val:item)* }),+ $(,)?
            $(_ => { $($fallback:item)* })?
        }
    ) => {
        $(
            $(
                #[cfg($ident = $pat)]
                $val
            )*
        )+

        match_cfg_meta!(@FALLBACK $ident; $($pat)+; $($($fallback)*)?);
    };
    (@FALLBACK $ident:ident; $($pat:literal)+; $($fallback:item)*) => {
        #[cfg(not(any($($ident = $pat),+)))]
        $($fallback)*
    };
    (@FALLBACK $ident:ident; $($pat:literal)+;) => {}
}

#[allow(unused_imports)]
pub(crate) use {conditional, match_cfg_meta};

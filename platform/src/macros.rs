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

pub(crate) use conditional;

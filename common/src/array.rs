use crate::int_types::{s1, u1};

pub trait IntoJByte {
	fn into_jbyte_array(self) -> Box<[s1]>;
}

impl IntoJByte for Box<[u1]> {
	fn into_jbyte_array(self) -> Box<[s1]> {
		let ptr = Box::into_raw(self);
		unsafe { Box::from_raw(ptr as *mut [s1]) }
	}
}

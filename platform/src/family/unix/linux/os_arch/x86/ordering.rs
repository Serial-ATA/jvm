#![cfg_attr(rustfmt, rustfmt_skip)]

#[inline(always)]
unsafe fn compiler_barrier() {
	// Create a compiler barrier to prevent instruction reordering
	core::arch::asm!("", options(nomem))
}

#[inline(always)]
pub fn loadload() { unsafe { compiler_barrier() } }

#[inline(always)]
pub fn storestore() { unsafe { compiler_barrier() } }

#[inline(always)]
pub fn loadstore() { unsafe { compiler_barrier() } }

#[inline(always)]
pub fn storeload() { fence() }

#[inline(always)]
pub fn acquire() { unsafe { compiler_barrier() } }

#[inline(always)]
pub fn release() { unsafe { compiler_barrier() } }

#[cfg(target_arch = "x86")]
#[inline(always)]
pub fn fence() {
	unsafe {
		core::arch::asm!(
			"lock",
			"addl $0,0(%esp)",
			options(nomem, preserves_flags, att_syntax)
		)
	}

	unsafe { compiler_barrier() }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn fence() {
	unsafe {
		core::arch::asm!(
			"lock",
			"addl $0,0(%rsp)",
			options(nomem, preserves_flags, att_syntax)
		)
	}
	
	unsafe { compiler_barrier() }
}

#[inline(always)]
pub fn cross_modify_fence_impl() {
	todo!()
}

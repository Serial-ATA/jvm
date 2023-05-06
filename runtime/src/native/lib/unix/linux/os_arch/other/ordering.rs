use core::sync::atomic::Ordering;

// Use the strongest barriers by default for correctness

#[inline(always)]
pub fn loadload() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn storestore() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn loadstore() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn storeload() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn acquire() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn release() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn fence() {
	core::sync::atomic::fence(Ordering::SeqCst)
}

#[inline(always)]
pub fn cross_modify_fence_impl() {}

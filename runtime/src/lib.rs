#![feature(thread_local)]
#![feature(impl_trait_in_assoc_type)]
#![feature(macro_metavar_expr)]
#![feature(specialization)]
#![feature(sync_unsafe_cell)]
#![feature(try_trait_v2)]
#![feature(iter_next_chunk)]
#![feature(reentrant_lock)]
#![feature(std_internals)]
#![feature(pointer_is_aligned_to)]
#![feature(try_trait_v2_residual)]
#![feature(int_roundings)]
#![feature(transmute_prefix)]
#![feature(generic_atomic)]
#![feature(adt_const_params)]
#![feature(decl_macro)]
#![feature(panic_update_hook)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("The VM only supports 64-bit systems!");

pub mod calls;
pub mod classes;
pub mod classpath;
mod dynamic;
pub mod error;
pub mod globals;
mod initialization;
mod interpreter;
pub mod logging;
pub mod modules;
pub mod native;
pub mod objects;
pub mod options;
pub mod stack;
mod symbols;
pub mod thread;
pub mod verifier;

#[cfg(test)]
pub(crate) mod test_utils;

pub fn init_panic_handler() {
	use thread::JavaThread;

	std::panic::update_hook(|prev, info| {
		if let Some(thread) = JavaThread::current_opt() {
			use std::io::Write as _;
			let stderr = std::io::stderr();
			let mut stderr = stderr.lock();

			let _ = writeln!(stderr, "Panic on Java thread `{}`\n", thread.name());

			let _ = writeln!(stderr, "Operand stack:\n");
			if let Some(current) = thread.frame_stack().current() {
				for op in current.stack() {
					let _ = writeln!(stderr, "{op:?}");
				}
			}

			let _ = writeln!(stderr, "Frame stack:\n");
			for (idx, frame) in thread.frame_stack().iter().enumerate() {
				let _ = write!(stderr, "{idx}: ");
				if frame.method().is_native() {
					let _ = write!(stderr, "(native) ");
				}
				let _ = writeln!(stderr, "{}", frame.method().external_name());
			}
			if thread.frame_stack().is_empty() {
				let _ = writeln!(stderr, "<empty>");
			}

			let _ = writeln!(stderr);
		}

		prev(info)
	});
}

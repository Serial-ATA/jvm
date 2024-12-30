use std::sync::{LazyLock, Mutex};

use common::int_types::u1;
use fxhash::FxBuildHasher;
use indexmap::IndexSet;

static INTERNER: LazyLock<Mutex<SymbolInterner>> =
	LazyLock::new(|| Mutex::new(SymbolInterner::initialize()));

type FxIndexSet<T> = IndexSet<T, FxBuildHasher>;

/// The symbol interner, responsible for mapping symbols to strings
struct SymbolInterner {
	arena: bumpalo::Bump,
	set: FxIndexSet<&'static [u8]>,
}

impl SymbolInterner {
	fn initialize() -> Self {
		let mut this = Self {
			arena: bumpalo::Bump::new(),
			set: FxIndexSet::default(),
		};

		// Method generated by `vm_symbols::define_symbols`
		this.preintern();
		assert!(
			this.set.len() < (1 << Symbol::LOG2_LIMIT),
			"Too many symbols registered for pre-intern (> 2048)"
		);

		this
	}

	/// Intern a string
	#[allow(trivial_casts)]
	fn intern(&mut self, string: &[u8]) -> Symbol {
		if let Some(symbol_idx) = self.set.get_index_of(string) {
			return Symbol::new(symbol_idx as u32);
		}

		// We extend the lifetime of the string to `'static`, which is safe,
		// as we only use the strings while the arena is alive
		let string: &'static [u8] = unsafe { &*(*self.arena.alloc(string) as *const [u8]) };

		let (index, _) = self.set.insert_full(string);
		Symbol::new(index as u32)
	}

	fn intern_static(&mut self, bytes: &'static [u8]) -> Symbol {
		let (index, _) = self.set.insert_full(bytes);
		Symbol::new(index as u32)
	}

	/// Gets the string for a Symbol
	fn get(&self, symbol: Symbol) -> &'static [u8] {
		self.set[symbol.as_u32() as usize]
	}
}

/// An index representation of an interned string
///
/// These are used for quick actions frequently accessed strings
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct Symbol(u32);

impl Symbol {
	/// Access the actual string associated with this symbol
	pub fn as_str(&self) -> &'static str {
		let guard = INTERNER.lock().unwrap();
		unsafe { std::str::from_utf8_unchecked(guard.get(*self)) }
	}

	/// Access the byte string associated with this symbol
	pub fn as_bytes(&self) -> &'static [u8] {
		let guard = INTERNER.lock().unwrap();
		guard.get(*self)
	}

	/// Access the `u32` representation of this symbol
	#[inline]
	pub fn as_u32(&self) -> u32 {
		self.0
	}
}

impl Symbol {
	/// The maximum number of bits that a pre-interned symbol ID can occupy
	///
	/// This currently allows for up to 2048 VM symbols
	pub const LOG2_LIMIT: u8 = 11;

	/// Create a new symbol with the specified index
	pub const fn new(index: u32) -> Self {
		Self(index)
	}

	/// Maps a string to its interned representation
	///
	/// NOTE: This will leak `string`.
	pub fn intern_owned(string: String) -> Self {
		let leaked_bytes: &'static mut [u8] = string.into_bytes().leak();
		let mut guard = INTERNER.lock().unwrap();
		guard.intern_static(leaked_bytes)
	}

	/// Maps a string to its interned representation
	pub fn intern(string: &'static str) -> Self {
		let mut guard = INTERNER.lock().unwrap();

		// SAFETY: &str and &[u8] have the same layout
		guard.intern_static(unsafe { core::mem::transmute(string) })
	}

	/// Same as [`Symbol::intern`], but takes a byte slice, which is used
	/// heavily in the VM
	pub fn intern_bytes(bytes: &[u1]) -> Self {
		assert!(std::str::from_utf8(bytes).is_ok());
		unsafe { Self::intern_bytes_unchecked(bytes) }
	}

	pub unsafe fn intern_bytes_unchecked(bytes: &[u1]) -> Self {
		let mut guard = INTERNER.lock().unwrap();
		guard.intern(bytes)
	}
}

/// Gets a generated symbol using the names defined in `vm_symbols::define_symbols!`
#[macro_export]
macro_rules! sym {
	($symbol:ident) => {
		$crate::generated_symbols::$symbol
	};
}

// Defined in $ROOT/generators/vm_symbols
//
// NOTE: **ONLY ADD MANUAL ENTRIES ABOVE THE MARKER COMMENTS**
//       Other generators may inject into this macro, take note of the marker comments below
vm_symbols::define_symbols! {
	// Classes

	// Primitive
	java_lang_Boolean: "java/lang/Boolean",
	java_lang_Byte: "java/lang/Byte",
	java_lang_Character: "java/lang/Character",
	java_lang_Double: "java/lang/Double",
	java_lang_Float: "java/lang/Float",
	java_lang_Integer: "java/lang/Integer",
	java_lang_Long: "java/lang/Long",
	java_lang_Short: "java/lang/Short",
	java_lang_Void: "java/lang/Void",

	java_lang_Class: "java/lang/Class",
	java_lang_Object: "java/lang/Object",
	java_lang_String: "java/lang/String",
	java_lang_Module: "java/lang/Module",
	java_lang_System: "java/lang/System",
	java_lang_Cloneable: "java/lang/Cloneable",
	java_io_Serializable: "java/io/Serializable",
	java_io_File: "java/io/File",
	jdk_internal_misc_UnsafeConstants: "jdk/internal/misc/UnsafeConstants",
	jdk_internal_reflect_MethodAccessorImpl: "jdk/internal/reflect/MethodAccessorImpl",
	java_lang_StackTraceElement: "java/lang/StackTraceElement",
	java_lang_invoke_MethodHandle: "java/lang/invoke/MethodHandle",
	java_lang_invoke_VarHandle: "java/lang/invoke/VarHandle",
	java_lang_Thread: "java/lang/Thread",
	java_lang_ThreadGroup: "java/lang/ThreadGroup",
	java_lang_Thread_FieldHolder: "java/lang/Thread$FieldHolder",
	java_lang_ref_Finalizer: "java/lang/ref/Finalizer",
	// -- GENERATED CLASS NAME MARKER, DO NOT DELETE --

	// Signatures
	main_signature: "([Ljava/lang/String;)V",
	void_method_signature: "()V",
	bool_bool_int_signature: "(ZZ)I",
	ClassLoader_string_long_signature: "(Ljava/lang/ClassLoader;Ljava/lang/String;)J",
	ThreadGroup_String_void_signature: "(Ljava/lang/ThreadGroup;Ljava/lang/String;)V",
	ThreadGroup_Runnable_void_signature: "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;)V",
	// -- GENERATED METHOD SIGNATURE MARKER, DO NOT DELETE --

	// Types
	bool: "Z",
	byte: "B",
	char: "C",
	double: "D",
	float: "F",
	int: "I",
	long: "J",
	short: "S",
	bool_array: "[Z",
	byte_array: "[B",
	char_array: "[C",
	double_array: "[D",
	float_array:  "[F",
	int_array:  "[I",
	long_array: "[J",
	short_array: "[S",

	object_array: "[Ljava/lang/Object;",
	string_array: "[Ljava/lang/String;",
	StackTraceElement_array: "[Ljava/lang/StackTraceElement;",

	// Methods
	main_name: "main",
	object_initializer_name: "<init>",
	class_initializer_name: "<clinit>",
	athrow_name: "<athrow>",

	initPhase1_name: "initPhase1",
	initPhase2_name: "initPhase2",
	initPhase3_name: "initPhase3",

	printStackTrace_name: "printStackTrace",
	findNative_name: "findNative",
	run_name: "run",
	// -- GENERATED METHOD NAME MARKER, DO NOT DELETE --

	// Fields
	ADDRESS_SIZE0: "ADDRESS_SIZE0",
	PAGE_SIZE: "PAGE_SIZE",
	BIG_ENDIAN: "BIG_ENDIAN",
	UNALIGNED_ACCESS: "UNALIGNED_ACCESS",
	DATA_CACHE_LINE_FLUSH_SIZE: "DATA_CACHE_LINE_FLUSH_SIZE",
	name: "name",
	referent: "referent",
	loader: "loader",
	holder: "holder",
	eetop: "eetop",
	stackSize: "stackSize",
	priority: "priority",
	daemon: "daemon",
	threadStatus: "threadStatus",
	value: "value",
	coder: "coder",
	fd: "fd",
	path: "path",
	r#in: "in",
	out: "out",
	err: "err",
	stackTrace: "stackTrace",
	backtrace: "backtrace",
	depth: "depth",
	classLoaderName: "classLoaderName",
	moduleName: "moduleName",
	moduleVersion: "moduleVersion",
	declaringClass: "declaringClass",
	methodName: "methodName",
	fileName: "fileName",
	lineNumber: "lineNumber",
}

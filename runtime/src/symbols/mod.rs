use std::borrow::Cow;
use std::fmt::Display;
use std::sync::{LazyLock, Mutex};

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
			arena: bumpalo::Bump::with_capacity(1024),
			set: FxIndexSet::default(),
		};

		// Method generated by `vm_symbols::define_symbols`
		this.preintern();
		assert!(
			this.set.len() <= Symbol::PRE_INTERNED_LIMIT,
			"Too many symbols registered for pre-intern (> {})",
			Symbol::PRE_INTERNED_LIMIT
		);

		this
	}

	/// Intern a string
	#[allow(trivial_casts)]
	fn intern<T: Internable>(&mut self, string: T) -> Symbol {
		let bytes = string.as_bytes();

		if let Some(symbol_idx) = self.set.get_index_of(bytes) {
			assert!(symbol_idx < self.set.len());
			return Symbol::new(symbol_idx as u32);
		}

		// We extend the lifetime of the string to `'static`, which is safe,
		// as we only use the strings while the arena is alive
		let string: &'static [u8] =
			unsafe { &*(self.arena.alloc_slice_copy(bytes) as *const [u8]) };
		self.intern_static(string)
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
	/// The maximum number of pre-interned symbols allowed
	const PRE_INTERNED_LIMIT: usize = 2048;
	pub const PRE_INTERNED_LIMIT_LOG2: u8 = (Self::PRE_INTERNED_LIMIT.ilog2()) as u8;

	/// Create a new symbol with the specified index
	pub const fn new(index: u32) -> Self {
		Self(index)
	}

	/// Maps a string to its interned representation
	pub fn intern<T: Internable>(string: T) -> Self {
		let mut guard = INTERNER.lock().unwrap();
		guard.intern(string)
	}
}

pub trait Internable {
	fn as_bytes(&self) -> &[u8];
}

impl Internable for String {
	fn as_bytes(&self) -> &[u8] {
		String::as_bytes(self)
	}
}

impl Internable for Cow<'_, str> {
	fn as_bytes(&self) -> &[u8] {
		str::as_bytes(&*self)
	}
}

impl Internable for Box<[u8]> {
	fn as_bytes(&self) -> &[u8] {
		&*self
	}
}

impl Internable for Vec<u8> {
	fn as_bytes(&self) -> &[u8] {
		&*self
	}
}

impl Internable for Cow<'_, [u8]> {
	fn as_bytes(&self) -> &[u8] {
		&*self
	}
}

impl Internable for &[u8] {
	fn as_bytes(&self) -> &[u8] {
		self
	}
}

impl Internable for &str {
	fn as_bytes(&self) -> &[u8] {
		str::as_bytes(self)
	}
}

impl<T> Internable for &T
where
	T: Internable,
{
	fn as_bytes(&self) -> &[u8] {
		<T as Internable>::as_bytes(*self)
	}
}

impl<T> From<T> for Symbol
where
	T: Internable,
{
	fn from(value: T) -> Self {
		Symbol::intern(value)
	}
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

/// Gets a generated symbol using the names defined in `vm_symbols::define_symbols!`
macro_rules! sym {
	($symbol:ident) => {
		$crate::symbols::generated_symbols::$symbol
	};
}

pub(crate) use sym;

// Defined in $ROOT/generators/vm_symbols
//
// NOTE: **ONLY ADD MANUAL ENTRIES ABOVE THE MARKER COMMENTS**
//       Other generators may inject into this macro, take note of the marker comments below
vm_symbols::define_symbols! {
	EMPTY: "",

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
	java_lang_invoke_MethodHandle: "java/lang/invoke/MethodHandle",
	java_lang_invoke_MethodHandleNatives: "java/lang/invoke/MethodHandleNatives",
	java_lang_invoke_VarHandle: "java/lang/invoke/VarHandle",
	java_lang_invoke_MemberName: "java/lang/invoke/MemberName",
	java_lang_invoke_ResolvedMethodName: "java/lang/invoke/ResolvedMethodName",
	java_lang_invoke_MethodType: "java/lang/invoke/MethodType",
	java_lang_invoke_LambdaForm: "java/lang/invoke/LambdaForm",
	java_lang_reflect_Constructor: "java/lang/reflect/Constructor",
	java_lang_reflect_Method: "java/lang/reflect/Method",
	java_lang_Thread: "java/lang/Thread",
	java_lang_ThreadGroup: "java/lang/ThreadGroup",
	java_lang_Thread_FieldHolder: "java/lang/Thread$FieldHolder",
	java_lang_ref_Finalizer: "java/lang/ref/Finalizer",
	jdk_internal_loader_ClassLoaders_PlatformClassLoader: "jdk/internal/loader/ClassLoaders$PlatformClassLoader",

	// Throwables
	java_lang_Throwable: "java/lang/Throwable",
	java_lang_StackTraceElement: "java/lang/StackTraceElement",

	java_lang_VirtualMachineError: "java/lang/VirtualMachineError",

	java_lang_ClassFormatError: "java/lang/ClassFormatError",
	java_lang_UnsupportedClassVersionError: "java/lang/UnsupportedClassVersionError",
	java_lang_NoClassDefFoundError: "java/lang/NoClassDefFoundError",
	java_lang_ClassCastException: "java/lang/ClassCastException",

	java_lang_LinkageError: "java/lang/LinkageError",
	java_lang_UnsatisfiedLinkError: "java/lang/UnsatisfiedLinkError",
	java_lang_IncompatibleClassChangeError: "java/lang/IncompatibleClassChangeError",
	java_lang_NoSuchFieldError: "java/lang/NoSuchFieldError",
	java_lang_NoSuchMethodError: "java/lang/NoSuchMethodError",
	java_lang_AbstractMethodError: "java/lang/AbstractMethodError",

	java_lang_NegativeArraySizeException: "java/lang/NegativeArraySizeException",
	java_lang_ArrayIndexOutOfBoundsException: "java/lang/ArrayIndexOutOfBoundsException",

	java_lang_CloneNotSupportedException: "java/lang/CloneNotSupportedException",

	java_lang_InvalidClassException: "java/lang/InvalidClassException",

	java_lang_NullPointerException: "java/lang/NullPointerException",
	java_lang_IllegalAccessError: "java/lang/IllegalAccessError",
	java_lang_IllegalArgumentException: "java/lang/IllegalArgumentException",
	java_lang_IllegalStateException: "java/lang/IllegalStateException",
	java_lang_IndexOutOfBoundsException: "java/lang/IndexOutOfBoundsException",
	java_lang_IllegalThreadStateException: "java/lang/IllegalThreadStateException",
	java_lang_InternalError: "java/lang/InternalError",
	// -- GENERATED CLASS NAME MARKER, DO NOT DELETE --

	// Signatures
	main_signature: "([Ljava/lang/String;)V",
	void_method_signature: "()V",
	bool_bool_int_signature: "(ZZ)I",
	ClassLoader_string_long_signature: "(Ljava/lang/ClassLoader;Ljava/lang/String;)J",
	ThreadGroup_String_void_signature: "(Ljava/lang/ThreadGroup;Ljava/lang/String;)V",
	ThreadGroup_Runnable_void_signature: "(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;)V",
	linkCallSite_signature: "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/invoke/MemberName;",
	findMethodHandleType_signature: "(Ljava/lang/Class;[Ljava/lang/Class;)Ljava/lang/invoke/MethodType;",
	linkMethodHandleConstant_signature: "(Ljava/lang/Class;ILjava/lang/Class;Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/invoke/MethodHandle;",
	ClassLoader_class_string_string_long_signature: "(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;)J",

	Boolean_valueOf_signature: "(Z)Ljava/lang/Boolean;",
	Integer_valueOf_signature: "(I)Ljava/lang/Integer;",
	Long_valueOf_signature: "(J)Ljava/lang/Long;",
	Double_valueOf_signature: "(Z)Ljava/lang/Double;",
	Float_valueOf_signature: "(Z)Ljava/lang/Float;",
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

	linkCallSite,
	findMethodHandleType,
	linkMethodHandleConstant,

	dispatchUncaughtException,
	exit_name: "exit",

	initPhase1_name: "initPhase1",
	initPhase2_name: "initPhase2",
	initPhase3_name: "initPhase3",

	printStackTrace_name: "printStackTrace",
	findNative_name: "findNative",
	run_name: "run",

	valueOf_name: "valueOf",
	// -- GENERATED METHOD NAME MARKER, DO NOT DELETE --

	// Modules
	java_base: "java.base",

	// Fields
	ADDRESS_SIZE0,
	PAGE_SIZE,
	BIG_ENDIAN,
	UNALIGNED_ACCESS,
	DATA_CACHE_LINE_FLUSH_SIZE,
	name,
	module,
	classLoader,
	componentType,
	referent,
	loader,
	holder,
	eetop,
	stackSize,
	priority,
	daemon,
	threadStatus,
	value,
	coder,
	hash,
	hashIsZero,
	fd,
	path,
	r#in: "in",
	out,
	err,
	stackTrace,
	backtrace,
	depth,
	declaringClassObject,
	classLoaderName,
	moduleName,
	moduleVersion,
	declaringClass,
	methodName,
	fileName,
	lineNumber,
	unnamedModule,
	nameAndId,
	parallelLockMap,
	r#type: "type",
	flags,
	clazz,
	method,
	vmholder,
	vmentry,
	ptypes,
	rtype,
	slot,
	parameterTypes,
	exceptionTypes,
	modifiers,
	signature,
	annotations,
	parameterAnnotations,
	form,

	// Injected fields
	loader_ptr,
	module_ptr,
	vmindex,
}

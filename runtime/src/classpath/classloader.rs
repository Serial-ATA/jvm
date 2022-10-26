use crate::heap::class::{Class, ClassRef};

use std::ops::RangeInclusive;

use common::traits::ReferenceType;
use common::types::u1;

const SUPPORTED_MAJOR_LOWER_BOUND: u1 = 45;
const SUPPORTED_MAJOR_UPPER_BOUND: u1 = 63;
const SUPPORTED_MAJOR_VERSION_RANGE: RangeInclusive<u1> = SUPPORTED_MAJOR_LOWER_BOUND..=SUPPORTED_MAJOR_UPPER_BOUND;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClassLoader {
    Bootstrap,
    UserDefined,
}

impl ClassLoader {
    pub fn load(&self, name: &[u1]) -> ClassRef {
        match self {
            ClassLoader::Bootstrap => Self::load_bootstrap(name),
            ClassLoader::UserDefined => unimplemented!("User defined loader")
        }
    }

    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.3.1
    fn load_bootstrap(name: &[u1]) -> ClassRef {
        // TODO:
        // First, the Java Virtual Machine determines whether the bootstrap class loader has
        // already been recorded as an initiating loader of a class or interface denoted by N.
        // If so, this class or interface is C, and no class loading or creation is necessary.

        // Otherwise, the Java Virtual Machine passes the argument N to an invocation of a method on
        // the bootstrap class loader [...] and then [...] create C, via the algorithm of §5.3.5.
        let classref = ClassLoader::Bootstrap.load_class_by_name(name);

        // TODO:
        // If no purported representation of C is found, the bootstrap class loader throws a ClassNotFoundException.
        // The process of loading and creating C then fails with a NoClassDefFoundError whose cause is the ClassNotFoundException.

        // If a purported representation of C is found, but deriving C from the purported representation fails,
        // then the process of loading and creating C fails for the same reason.

        // Otherwise, the process of loading and creating C succeeds.
        classref
    }

    // Deriving a Class from a class File Representation
    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.3.5
    fn load_class_by_name(self, name: &[u1]) -> ClassRef {
        // TODO:
        // 1. First, the Java Virtual Machine determines whether L has already been recorded
        //    as an initiating loader of a class or interface denoted by N. If so, this derivation
        //    attempt is invalid and derivation throws a LinkageError.

        // 2. Otherwise, the Java Virtual Machine attempts to parse the purported representation.
        let classfile_bytes = super::find_classpath_entry(name);
        let classfile = class_parser::parse_class(&mut &classfile_bytes[..]);

        //    The purported representation may not in fact be a valid representation of C, so
        //    derivation must detect the following problems:

        // TODO:
        //  2.1. If the purported representation is not a ClassFile structure (§4.1, §4.8), derivation
        //       throws a ClassFormatError.

        //  2.2. Otherwise, if the purported representation is not of a supported major or
        //       minor version (§4.1), derivation throws an UnsupportedClassVersionError.
        assert!(SUPPORTED_MAJOR_VERSION_RANGE.contains(&(classfile.major_version as u1)), "UnsupportedClassVersionError");

        // TODO:
        //  2.3. Otherwise, if the purported representation does not actually represent a class or
        //       interface named N, derivation throws a NoClassDefFoundError. This occurs when the
        //       purported representation has either a this_class item which specifies a name other
        //       than N, or an access_flags item which has the ACC_MODULE flag set.

        //  3. If C has a direct superclass, the symbolic reference from C to its direct
        //     superclass is resolved using the algorithm of §5.4.3.1. Note that if C is an interface
        //     it must have Object as its direct superclass, which must already have been loaded.
        //     Only Object has no direct superclass.
        let mut super_class = None;

        if let Some(super_class_name) = classfile.get_super_class() {
            super_class = Self::resolve_super_class(super_class_name);
        }

        // TODO:
        // 4. If C has any direct superinterfaces, the symbolic references from C to its direct
        //    superinterfaces are resolved using the algorithm of §5.4.3.1.

        // If no exception is thrown in steps 1-4, then derivation of the class or interface C succeeds.
        // The Java Virtual Machine marks C to have L as its defining loader, records that L is an initiating
        // loader of C (§5.3.4), and creates C in the method area (§2.5.4).

        let class = Class::new(classfile, super_class, self);
        ClassRef::new(class)
    }

    fn resolve_super_class(super_class_name: &[u1]) -> Option<ClassRef> {
        // Any exception that can be thrown as a result of failure of class or interface resolution
        // can be thrown as a result of derivation. In addition, derivation must detect the following problems:

        // TODO:
        //     If any of the superclasses of C is C itself, derivation throws a ClassCircularityError.

        // TODO:
        //     Otherwise, if the class or interface named as the direct superclass of C is in fact an interface
        //     or a final class, derivation throws an IncompatibleClassChangeError.

        // TODO:
        //     Otherwise, if the class named as the direct superclass of C has a PermittedSubclasses attribute (§4.7.31)
        //     and any of the following is true, derivation throws an IncompatibleClassChangeError:

        // TODO:
        //         The superclass is in a different run-time module than C (§5.3.6).

        // TODO:
        //         C does not have its ACC_PUBLIC flag set (§4.1) and the superclass is in a different run-time package than C (§5.3).

        // TODO:
        //         No entry in the classes array of the superclass's PermittedSubclasses attribute refers to a class or interface with the name N.

        // TODO:
        //     Otherwise, if C is a class and some instance method declared in C can override (§5.4.5)
        //     a final instance method declared in a superclass of C, derivation throws an IncompatibleClassChangeError.

        panic!("Attempted to load super class: {}", unsafe { std::str::from_utf8_unchecked(super_class_name) });
    }
}
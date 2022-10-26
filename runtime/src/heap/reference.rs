use crate::heap::class::ClassRef;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.4
pub enum Reference {
    Class(ClassRef),
    Array,
    Interface,
    Null,
}
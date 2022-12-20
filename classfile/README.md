# JVM/classfile

An implementation of the [Java SE 19 class file format](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html).

The current structure is as follows:

```
ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
```

This crate only defines the structure of `ClassFile`. The parsing logic is housed in [the class-parser crate](../class-parser).
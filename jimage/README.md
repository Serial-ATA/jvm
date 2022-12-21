# JVM/jimage

An implementation of the JImage file format, as created by [jlink](https://docs.oracle.com/en/java/javase/11/tools/jlink.html).

This format is **not specified**, the structure is taken from [OpenJDK's libjimage](https://github.com/openjdk/jdk/tree/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage).

See [libjimage/imageFile.hpp](https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L40-L148) for a structure breakdown.
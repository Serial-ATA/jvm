# JVM/jmod

An implementation of the JMOD file format, as created by [jmod](https://docs.oracle.com/en/java/javase/11/tools/jmod.html).

This format is **not specified**, the structure is taken from [OpenJDK's JmodFile](https://github.com/openjdk/jdk/blob/master/src/java.base/share/classes/jdk/internal/jmod/JmodFile.java).

The current structure is as follows:

```
foo.jmod (ZIP archive)
├─ classes/
│  ├─ bar.class
│  ├─ ... (Class files)
├─ conf/
│  ├─ ... (Configuration files)
├─ include/
│  ├─ bar.h
│  ├─ ... (Header files)
├─ legal/
│  ├─ ... (Licenses)
├─ man/
│  ├─ ... (Man pages)
├─ lib/
│  ├─ bar.so
│  ├─ ... (Native libraries)
├─ bin/
│  ├─ ... (Native commands)
```

This crate contains both the structure and parsing logic.
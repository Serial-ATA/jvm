# JVM/platform

Platform specific items

This crate has the following layout:

```
platform
├── arch (`target_arch` specific definitions)
└── family (`target_family` specific definitions)
    └── <family>
        └── <os> (`target_os` specific definitions)
```
# (Binary name from Cargo, Packaged binary name)
BINARIES = [
    ("sj", "java"),
    ("jmod", "jmod"),
    ("jimage", "jimage"),
]

VM_LIBRARIES = ["jvm_runtime"]

LIBRARIES = [
    "nio",
]

LIBRARIES.extend(VM_LIBRARIES)

# Getting Started

Plug In C is a comprehensive C binding for writing plugins for [ewwii](https//ewwii-sh.github.io) in the C 
and C++ programming languages. Most modern languages support C through FFI (Foreign Function Interface),
so this project can be extended to make bindings for other languages like [Go](https://go.dev).

## Installation

There are two critical files that are required to make the C bindings work:

1. The `ewwii.h` file (C API).
2. The `libpluginc-ewwii.a` file (Rust bridge).

You can set it up manually, but we highly advice you to use our tool which will handle
initialization, and managing of the C bindings a breeze.

You can run this command to install `plugc-ewwii` locally:

```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/Byson94/plug-in-c.ewwii/refs/heads/main/plugc-ewwii/install.sh)"
```


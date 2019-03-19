# libtcc-sys
FFI bindings to libtcc (TinyCC compiler library)

This is just a first attempt at bindings for libtcc, made by a total Rust newbie.
Unfortunately I had very little time to put into this, but perhaps it can still be inspiring or useful to somebody.

Please note that libtcc is NOT thread safe, some parts of compilation routines use shared global state.
To run your tests, use the command ```cargo test -- --test-threads=1```

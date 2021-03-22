# Windows API documentation for Rust

This is an experimental documentation generator for the [Rust for Windows](https://github.com/microsoft/windows-rs) project. The documentation is published here:

https://microsoft.github.io/windows-docs-rs/

The `modules` crate generates the list of module paths to use in the bindings crate.

The `bindings` crate generates the documentation for the Windows API.

The build cannot be automated with GitHub Actions as it takes around 30 minutes (mostly single-threaded) and will exhaust the CI build's resources (`rustdoc` allocates around 20GB of memory) and ultimately fails the build.

It can be manually generated as follows:

`C:\git\windows-docs-rs\crates\bindings>cargo doc --no-deps --target-dir ..\..\docs`

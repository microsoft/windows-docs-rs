# Windows API documentation for Rust

This is an experimental documentation generator for Rust.

The `modules` crate generates the list of module paths to use in the bindings crate.

The `bindings` crate generates the documentation for the Windows API.

The documentation is published using GitHub Pages:

https://microsoft.github.io/windows-docs-rs/

It can be manually generated as follows:

C:\git\windows-docs-rs\crates\bindings>cargo doc --target-dir ..\..\docs

It needs to be generated in this folder so that GitHub Pages will publish it.

This cannot currently be done as a GitHub Action because it takes around 30 minutes (mostly single-threaded) and will exhaust the CI build's resources (`rustdoc` allocates around 20GB) and ultimately fails the build. A major reason for this is that `cargo doc` generates a multi-gigabyte `windows.rs.html` file containing the pretty-formatted version of the generated `windows.rs` file and there does not appear to be a way to exclude source code browsing using the Rust document generator.

So if you work on rustc or rustdoc, please use this for some performance testing and consider an option to exclude source code browsing. :)

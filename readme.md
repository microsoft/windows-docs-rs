# Windows API documentation for Rust

This is an experimental documentation generator for the [Rust for Windows](https://github.com/microsoft/windows-rs) project. The documentation is published here:

https://microsoft.github.io/windows-docs-rs/

The `modules` crate generates the list of module paths to use in the bindings crate.

The `bindings` crate generates the documentation for the Windows API.

Unfortunately, the documentation cannot easily be published using GitHub Pages because GitHub has a 100MB limit on files and `rustdoc` generates a few files that are well over that limit. The build also cannot be automated with GitHub Actions as it takes around 30 minutes (mostly single-threaded) and will exhaust the CI build's resources (`rustdoc` allocates around 20GB of memory) and ultimately fails the build. A major reason for this is that `cargo doc` generates a multi-gigabyte `windows.rs.html` file containing the pretty-formatted version of the generated `windows.rs` file and there does not appear to be a way to exclude source code browsing using the Rust document generator.

It can be manually generated as follows:

`C:\git\windows-docs-rs\crates\bindings>cargo doc --no-deps --target-dir ..\..\docs`

You then need to be careful to delete any files that are over 100MB before pushing to GitHub.

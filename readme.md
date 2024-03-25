# Windows API documentation for Rust

This is an experimental documentation generator for the [Rust for Windows](https://github.com/microsoft/windows-rs) project. The documentation is published here:

https://microsoft.github.io/windows-docs-rs/

It can be manually generated (using the nightly toolchain) as follows:

1. Delete everything in `windows-docs-rs/docs` except `index.html` and `.nojekyll`

2. Run the following command from the windows-rs directory:

```console
C:\git\windows-rs> cargo doc -p windows --all-features --no-deps --target-dir d:\git\windows-docs-rs\docs
```

3. Delete everything in `windows-docs-rs/docs` except `index.html`, `.nojekyll`, and `doc`

4. Run `git add .` followed by `git commit -am "<next version>"`

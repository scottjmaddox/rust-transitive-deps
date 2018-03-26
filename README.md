# rust-transitive-deps

An example rust project that contains conflicting transitive dependencies on the
`log` crate. Package `a` requires `log = "0.4"`, while package `b` requires
`log = "0.3"`. Running `cargo build` results in the following build log, which
shows log v0.4.1 being built for inclusion in `a` and log v0.3.9 being built
for inclusion in `b`.

```
Blocking waiting for file lock on the registry index
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling cfg-if v0.1.2
   Compiling log v0.4.1
   Compiling a v0.1.0 (file:///Users/smaddox/code/rust/test-transitive-deps/a)
   Compiling log v0.3.9
   Compiling b v0.1.0 (file:///Users/smaddox/code/rust/test-transitive-deps/b)
    Finished dev [unoptimized + debuginfo] target(s) in 1.49 secs
```

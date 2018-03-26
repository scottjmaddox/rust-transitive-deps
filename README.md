# rust-transitive-deps

An example rust project that contains conflicting transitive dependencies on the
`log` crate. Package `c` depends on packages `a` and `b`. Package `a` requires
`log = "0.4"`, while package `b` requires `log = "0.3"`. Running `cargo build`
results in the following build log, which shows `log v0.4.1` being built for
inclusion in `a` and `log v0.3.9` being built for inclusion in `b`. Package `c`
builds without issue, and contains code from both `log v0.4.1` and `log v0.3.9`.

```
   Compiling cfg-if v0.1.2
   Compiling log v0.4.1
   Compiling a v0.1.0 (file:///Users/smaddox/code/rust/test-transitive-deps/a)
   Compiling log v0.3.9
   Compiling b v0.1.0 (file:///Users/smaddox/code/rust/test-transitive-deps/b)
   Compiling c v0.1.0 (file:///Users/smaddox/code/rust/test-transitive-deps/c)
    Finished dev [unoptimized + debuginfo] target(s) in 1.36 secs
```

Quick example for https://users.rust-lang.org/t/clippy-fix-rlib-complaints/88473

This was occurring with both 1.66 and 1.67, *but* I've been seeing this for ages, first time I've tried to track this down.

This example code has a nice simple thing for clippy to detect and potentially fix.

```
cargo clippy
   Compiling autocfg v1.1.0
   Compiling crossbeam-utils v0.8.14
    Checking cfg-if v1.0.0
   Compiling libc v0.2.139
    Checking scopeguard v1.1.0
   Compiling rayon-core v1.10.2
    Checking either v1.8.1
   Compiling memoffset v0.7.1
   Compiling crossbeam-epoch v0.9.13
    Checking crossbeam-channel v0.5.6
    Checking num_cpus v1.15.0
    Checking crossbeam-deque v0.8.2
    Checking rayon v1.6.1
    Checking clippyhuh v0.1.0 (/home/twirrim/rust/clippyhuh)
warning: variables can be used directly in the `format!` string
 --> src/main.rs:5:9
  |
5 |         println!("Number {}", number);
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args
  = note: `#[warn(clippy::uninlined_format_args)]` on by default
help: change this to
  |
5 -         println!("Number {}", number);
5 +         println!("Number {number}");
  |

error: crate `rayon` required to be available in rlib format, but was not found in this form

warning: `clippyhuh` (bin "clippyhuh") generated 1 warning
error: could not compile `clippyhuh` due to previous error; 1 warning emitted
```


Note that error on line 35.
`error: crate `rayon` required to be available in rlib format, but was not found in this form`

Same error comes through via `cargo fix`:
```
$ cargo fix
    Checking clippyhuh v0.1.0 (/home/twirrim/rust/clippyhuh)
error: crate `rayon` required to be available in rlib format, but was not found in this form

error: could not compile `clippyhuh` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `clippyhuh` due to previous error
```
and the underlying issue remains unfixed.

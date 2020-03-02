This repository is to illustrate one of my issue when trying to use the [preferred answer](https://stackoverflow.com/a/44545091).

```
git clone https://github.com/czotti/rust_test_inner_crate.git
```

I want to use a type defined in the `main` crate but the rust compiler give me the error when I do `cargo test`

```
   Compiling main v0.1.0 (file:///mnt/data/sandbox/rust/main)
   Compiling utilities v0.1.0 (file:///mnt/data/sandbox/rust/main/utilities)
error[E0308]: mismatched types
  --> src/lib.rs:33:35
   |
33 |         assert_eq!(compute_sphere(get_default_sphere()), res);
   |                                   ^^^^^^^^^^^^^^^^^^^^ expected struct `Sphere`, found struct `main::Sphere`
   |
   = note: expected type `Sphere`
              found type `main::Sphere`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: Could not compile `main`.

To learn more, run the command again with --verbose.

```


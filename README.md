optimized-away
==============

A simple macro that prevents compilation unless all calls to it are optimized away.

Examples
========

Compiles:
```rs
use optimized_away::optimized_away;

fn main() {
    match 1 + 1 {
        2 => println!("math!"),
        _ => optimized_away!(),
    }

    assert!(false)
    optimized_away!("assert!(false) should panic");
}
```
```
Finished release [optimized] target(s) in 0.17s
```

Does not compile:
```rs
use optimized_away::optimized_away;

fn main() {
    match 1 + 1 {
        5 => println!("oh no"),
        _ => optimized_away!("1 + 1 is 2, not 5"),
    }
}
```

```
   Compiling playground_bin v0.1.0 (C:\...\playground_bin)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "link.exe" <lots of linker arguments>

error: not optimized away: `1 + 1 is 2, not 5`
  --> src\main.rs:6:14
   |
   | a reference to `optimized_away!` was found in the final executable
   |
   = note: referenced in function _ZN14playground_bin4main17h5c387cc751c86bb3E
          C:\...\playground_bin.exe : fatal error LNK1120: 1 unresolved externals


error: could not compile `optimized_away` due to previous error
```

How?
====

A non-existent external symbol is defined and then accessed.

If the access is optimized away, the symbol is never linked to, and the linker will not complain about it not existing.

Since we can be *very* creative when naming the symbol we're importing, we're inserting context like file and line number into the symbol's name. We're also including console control characters so that the output error is prettier and closer to `rustc`'s normal errors.

---

For anyone familiar with [`no_panic`](https://github.com/dtolnay/no-panic) / [`dont_panic`](https://github.com/Kixunil/dont_panic), this crate works through similar means.

I wasn't aware that either of them used the same trick until it was pointed out to me, but it's nice to see that I'm not the only one (ab)using tooling for unusual reasons.

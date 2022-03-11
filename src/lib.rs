#![no_std]

/// A simple macro that prevents compilation unless the call to it is optimized away.
///
/// This compiles, since the compiler is able to tell that 1 + 1 is 2.
/// ```
/// # use optimized_away::optimized_away;
/// match 1 + 1 {
///     2 => println!("math!"),
///     _ => optimized_away!(),
/// }
/// ```
///
/// This also compiles as the assert ensures that `optimized_away!` is never hit.
/// ```should_panic
/// # use optimized_away::optimized_away;
/// assert!(false);
/// optimized_away!("assert!(false) should panic");
/// ```
///
/// And lastly, this does not compile as the `optimized_away!` macro is reachable.
/// ```compile_fail
/// # use optimized_away::optimized_away;
/// match 1 + 1 {
///     5 => println!("oh no"),
///     _ => optimized_away!(),
/// }
/// ```
#[macro_export]
macro_rules! optimized_away {
    () => { $crate::optimized_away!("no description") };
    ($x:literal) => {{
        mod optimized_away {
            extern "C" {
                #[link_name = concat!(
                    "\x01",    // Avoid mangling
                    "\x1B[2K", // Clear the current line

                    /*       clear     bold  */
                    "\n\r", "\x1B[K", "\x1B[1m", "\x1B[91merror\x1B[39m\x1B[1m: not optimized away: `", $x, "`",
                    "\n\r", "\x1B[K", "\x1B[1m", "  --> \x1B[m", file!(), ":", line!(), ":", column!(),
                    "\n\r", "\x1B[K", "\x1B[1m", "   |",
                    "\n\r", "\x1B[K", "\x1B[1m", "   | \x1B[ma reference to `optimized_away!` was found in the final executable",
                    "\n\r", "\x1B[K", "\x1B[1m", "   |",
                    "\n\r", "\x1B[K", "\x1B[1m", "   = note:\x1B[m",
                )] pub static OPT: extern "C" fn() -> !;
            }
        }

        #[allow(unused_unsafe)]
        unsafe { optimized_away::OPT() };
    }};
}

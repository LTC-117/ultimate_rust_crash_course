// 1. Organize code into a library without changing the output of the program.
//
// For each step of this exercise, you should be able to run the program before and after your
// change without affecting the output of the program.
//
// Move the constants below (FIRST, SECOND, and THIRD) into the library:
// - Create a `src/lib.rs` file
// - Move all of the constants into lib.rs
// - Make the constants public by adding the `pub` keyword in front of them
// - Add `use` statement(s) to main.rs to bring the constants into scope.
//
// Hint: the name of the library is defined in Cargo.toml

pub mod sound;

pub const FIRST: i32 = 1;
pub const SECOND: i32 = 2;
pub const THIRD: i32 = 3;

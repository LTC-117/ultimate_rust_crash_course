// 2. Create a library module named `sound` and move the animal functions into it.
//
// - In your lib.rs file add the line `pub mod sound;`
// - Create a `src/sound.rs` file for your module
// - Move the `dog`, `cat`, and `fox` functions into sound.rs
// - Make the functions public by adding the `pub` keyword in front of them
// - Add a `use` statement to bring the `sound` module into scope.
// - Change the function calls to access the functions through the `sound` module.
//   For example: sound::dog()

pub mod tame;
pub mod wild;

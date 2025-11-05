// Challenge 1
//
// - Move the `dog` and `cat` functions into a submodule `animal::sound::tame`
// - Move the `fox` function into a submodule `animal::sound::wild`
//
// Hint: You will need to create a subdirectory for the top-level `sound` modules' submodules to
// be placed in.

pub fn dog() {
    println!("Dog goes WOOF!");
}

pub fn cat() {
    println!("Cat goes MEOW!");
}


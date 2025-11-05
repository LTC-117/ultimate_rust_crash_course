use animal::prelude::*;


fn main() {
    print!("Listening to animal {}: ", FIRST);
    sound::tame::dog();

    print!("Listening to animal {}: ", SECOND);
    sound::tame::cat();

    print!("Listening to animal {}: ", THIRD);
    sound::wild::fox();
}

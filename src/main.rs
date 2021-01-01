#![allow(incomplete_features)]
#![feature(const_generics)]

mod traits;
pub use traits::*;

mod types;
pub use types::*;

mod util;
pub use util::*;

mod game_types;
pub use game_types::*;

fn main() {
    Game::start();
}

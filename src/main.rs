// real magic happens in lib.rs and model.rs

use tutorial9_models::run;

fn main() {
    pollster::block_on(run());
}

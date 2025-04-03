// real magic happens in all the other files

use tutorial9_models::run;

fn main() {
    pollster::block_on(run());
}

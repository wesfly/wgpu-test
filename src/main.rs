// real magic happens in lib.rs and model.rs

use tutorial2_surface::run;

fn main() {
    pollster::block_on(run());
}

// real magic happens in lib.rs and model.rs

use wgpu_test::run;

fn main() {
    pollster::block_on(run());
}

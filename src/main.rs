// real magic happens in all the other files

use wgpu_test::run;

fn main() {
    pollster::block_on(run());
}

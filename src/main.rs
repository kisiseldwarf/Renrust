extern crate renrust;

use renrust::*;
use renrust::core::*;
use renrust::renderer::*;

const WIDTH : u32 = 1280;
const HEIGHT : u32 = 720;
const FULLSCREEN : bool = false;

fn main() {
    let cb = core::core_builder().width(WIDTH).height(HEIGHT).fullscreen(FULLSCREEN);
    let core = renderer::renderer_init(cb);
    renrust::start(core);
}

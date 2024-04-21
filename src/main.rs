extern crate kiss3d;

use kiss3d::window::Window;

fn main() {
    let mut window = Window::new("Kiss3d: simple_window");
    while window.render() {

    }
}

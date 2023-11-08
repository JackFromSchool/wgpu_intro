mod window;
mod state;
mod buffers;

fn main() {
    pollster::block_on(crate::window::run());
}

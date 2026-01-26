mod handlers;
mod state;
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};

#[derive(Debug, Clone)]
enum backend {
    winit,
    tty,
}

fn main() {

    // match backend{}
}

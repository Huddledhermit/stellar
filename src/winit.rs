use smithay::{
    backend::renderer::{
        damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
    },
    reexports::calloop::EventLoop,
};
use crate::state::{ClientState,stellar};
fn run_winit() {
    let mut event_loop = EventLoop::try_new().unwrap();
    let display = Display::new().unwrap();
    let mut display_handle = display.handle();

    event_loop.handle().insert_source<stellar>(a)
}


use crate::state::stellar;
use smithay::wayland::{
    buffer::BufferHandler,
    compositor::{CompositorClientState, CompositorState, CompostorHandler},
    shm::{ShmHandler, ShmState},
};

impl BufferHandler for stellar {
    fn buffer_destroyed(&mut self, buffer: &WLBuffer);
}

impl CompositorHandler for stellar {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client.get_data
    }
    fn commit(&mut self, surface: &WLSurface);
}

impl ShmHandler for stellar {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

delegate_compositor!(stellar);
delegate_shm!(stellar);

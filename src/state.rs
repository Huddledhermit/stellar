use std::{ffi::OsString, sync::Arc};

use smithay::{
    desktop::{PopupManager, Space, Window, WindowSurfaceType},
    input::{Seat, SeatHandler, SeatState},
    reexports::{
        calloop::{EventLoop, Interest, LoopSignal, Mode, PostAction, generic::Generic},
        rustix::net::listen,
        wayland_server::{
            Display, DisplayHandle,
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::wl_surface::WlSurface,
        },
    },
    utils::{Logical, Point},
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        output::OutputManagerState,
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
        socket::ListeningSocketSource,
    },
};
pub struct stellar {
    pub start_time: std::time::Instant,
    pub socket: OsString,
    pub space: Space<Window>,
    pub loop_signal: LoopSignal,
    pub display_handle: DisplayHandle,

    pub compositor_state: CompositorState,
    pub xdg_state: XdgShellState,
    pub shm_state: ShmState,
    pub output_state: OutputManagerState,
    pub seat_state: SeatState<stellar>,
    pub data_device_state: DataDeviceState,
    pub popup_manager: PopupManager,

    pub seat: Seat<Self>,
}

impl stellar {
    pub fn new(eventloop: &mut EventLoop<Self>, display: Display<self>) -> Self {
        let start_time = std::time::Instant::now();

        let dh = display.handle();
        let compositor_state = CompositorState::new::<Self>(&dh);
        let xdg_state = XdgShellState::new::<Self>(&dh);
        let output_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);
        let shm_state = ShmState::new::<Self>(&dh, vec![]);
        let popup_manager = PopupManager::default();

        let mut seat_state = SeatState::new();
        let mut seat: Seat<Self> = seat_state.new_wl_seat(&dh, "winit");

        let data_device_state = DataDeviceState::new::<Self>(&dh);

        seat.add_keyboard(Default::default(), 200, 25).unwrap();
        seat.add_pointer();
        let space = Space::default();
        let socket = Self::create_wl_listener(display, eventloop);
        let loop_signal = eventloop.get_signal();

        Self {
            start_time,
            socket,
            space,
            seat,
            seat_state,
            display_handle: dh,
            compositor_state,
            output_state,
            popup_manager,
            shm_state,
            xdg_state,
            output_state,
            loop_signal,
            data_device_state

        }
    }

    fn create_wl_listener(display: Display<stellar>, eventloop: &mut EventLoop<Self>) -> OsString {
        let socket = ListeningSocketSource::new_auto().unwrap();

        let socket_name = socket.socket_name().to_os_string();
        let loop_handle = eventloop.handle();
        loop_handle
            .insert_source(socket, move |client_stream, _, state| {
                state
                    .display_handle
                    .insert_client(client_stream, Arc::new(ClientState::default()))
                    .unwrap();
            })
            .expect("failed to initialize");
        loop_handle
            .insert_source(
                Generic::new(display, Interest::READ, Mode::Level),
                |display, _, state| {
                    unsafe {
                        display.get_mut().dispatch_clients(state).unwrap();
                    }
                    Ok(PostAction::Continue)
                },
            )
            .unwrap();
        socket_name
    }
    pub fn surface_under(&self,pos:Point<f64, Logical>) -> Option<(WlSurface,Poibt<f64, Logical>)>{
        self.space.element_under(pos).and_then(|(window, location)| {
                   window
                       .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                       .map(|(s, p)| (s, (p + location).to_f64()))
               })
           }
    }


}

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}

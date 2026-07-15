use astra_studio::app::application::AstraApplication;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = AstraApplication::new();

    event_loop.run_app(&mut app).unwrap();
}
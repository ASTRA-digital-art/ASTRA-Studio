use crate::app::window::AstraWindow;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::WindowId,
};

pub struct AstraApplication {
    window: Option<AstraWindow>,
}

impl AstraApplication {
    pub fn new() -> Self {
        Self {
            window: None,
        }
    }
}

impl ApplicationHandler for AstraApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            AstraWindow::create(event_loop)
        );
    }

    fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
    ){
    match event {
        WindowEvent::CloseRequested => {
            println!("ASTRA closing");
            event_loop.exit();
        }
        _ => {}
    }
}

}
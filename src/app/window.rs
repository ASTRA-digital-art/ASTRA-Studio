use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

pub struct AstraWindow {
    window: Window,
}

impl AstraWindow {
    pub fn create(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("ASTRA Studio"),
            )
            .unwrap();

        Self { window }
    }
}
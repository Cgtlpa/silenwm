use smithay::{
    backend::{
        renderer::{Color32F, Frame, Renderer, gles::GlesRenderer},
        winit::{self, WinitEvent},
    },
    reexports::winit::platform::pump_events::PumpStatus,
    utils::{Rectangle, Transform},
};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing SilenWM compositor...");

    // Initialize the graphical backend.
    let (mut backend, mut event_loop) = winit::init::<GlesRenderer>()?;

    // Set the window title.
    backend.window().set_title("SilenWM");

    println!("SilenWM graphical backend initialized.");

    // Main rendering loop.
    loop {
        let mut should_close = false;

        let status = event_loop.dispatch_new_events(|event| {
            if matches!(event, WinitEvent::CloseRequested) {
                should_close = true;
            }
        });

        if should_close || matches!(status, PumpStatus::Exit(_)) {
            break;
        }

        // Get current window dimensions.
        let size = backend.window_size();
        let damage = Rectangle::from_size(size);

        // Bind the renderer to the window.
        let (renderer, mut framebuffer) = backend.bind()?;

        // Start a rendering frame.
        let mut frame = renderer.render(&mut framebuffer, size, Transform::Flipped180)?;

        // Clear the screen with Silen's dark background.
        frame.clear(Color32F::new(0.075, 0.08, 0.065, 1.0), &[damage])?;

        // Finish rendering.
        let _ = frame.finish()?;

        // Drop framebuffer before submitting.
        drop(framebuffer);

        // Display the rendered frame.
        backend.submit(Some(&[damage]))?;
    }

    println!("SilenWM compositor stopped.");

    Ok(())
}

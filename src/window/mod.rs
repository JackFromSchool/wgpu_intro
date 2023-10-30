use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{PhysicalKey, KeyCode}
};

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = crate::state::State::new(window).await;
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == state.window.id() => if !state.input(&event) {
                match event {
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                        ..
                    } => {
                        elwt.exit();
                    },
                    
                    WindowEvent::Resized(size) => {
                        state.resize(size);
                    },

                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {},
                            // Reconfigure the window if we lost the surface
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            // Other errors resolve by next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }

                    _ => {},
                }
            },

            Event::AboutToWait => {
                state.window.request_redraw();
            },

            _ => {}
        }
    }).unwrap();
}



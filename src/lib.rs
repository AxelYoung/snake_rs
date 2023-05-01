use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::PhysicalSize,
};

mod render;
mod systems;

use systems::GameState;

pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Snake")
        .with_inner_size(PhysicalSize { width: 800, height: 800})
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut render_state = pollster::block_on(render::new(window));

    let mut game_state = GameState::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == render_state.window().id() => {
            game_state.update();
            render_state.update(&game_state);
            match render_state.render() {
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => render_state.resize(render_state.size()),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e)
            }
        },
        Event::MainEventsCleared => {
            render_state.window().request_redraw();
        },
        Event::WindowEvent {
            window_id,
            ref event,
        } if window_id == render_state.window().id() => if !game_state.input(event) {
            match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    render_state.resize(*physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    render_state.resize(**new_inner_size);
                },
                _ => {}
            }
        },
        _ => {}
    });
}
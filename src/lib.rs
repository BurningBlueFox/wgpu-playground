use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowBuilder};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, control_flow| {
        handle_event(event, control_flow, &window);
    });
}

fn handle_event(event: Event<()>, control_flow: &EventLoopWindowTarget<()>, window: &Window) {
    match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => handle_window_event(event, control_flow),
        _ => {}
    }
}

fn handle_window_event(event: &WindowEvent, control_flow: &EventLoopWindowTarget<()>) {
    match event {
        WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
            event:
            KeyEvent {
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(KeyCode::Escape),
                ..
            },
            ..
        } => control_flow.exit(),
        _ => {}
    }
}
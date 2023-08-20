use n3d::{
    engine::EngineState,
    window::Window,
};

fn main() {
    match Window::new() {
        Ok((window, event_loop)) => {
            let mut engine = EngineState::new(&window);
            engine.setup();
            event_loop.run(move |event, _, control_flow| {
                control_flow.set_poll();
                engine.run(&window, event, control_flow);
            });
        }
        Err(_) => {
            eprintln!("Could not create window");
        }
    }
}

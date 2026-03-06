extern crate glfw;

use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

struct Engine {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<f64, WindowEvent>
}

impl Engine {
    fn new() -> self {
        use glfw::fail_on_errors;

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
        .create_window(640, 480, "engine test", glfw::WindowMode::Windowed)
        .expect("failed to create window");

        window.make_current();
        window.set_key_polling(true);

        self.glfw = glfw;
        self.window = window;
        self.events = events;
    }

    fn run() {
    
    }
}


fn main() {
    use glfw::fail_on_errors;
    // Initialize GLFW
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a window
    let (mut window, events) = glfw
        .create_window(640, 480, "GLFW Test", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    // Make the context current
    window.make_current();
    window.set_key_polling(true);

    // Main loop
    while !window.should_close() {
        // Swap buffers
        window.swap_buffers();
        // Poll events
        glfw.poll_events();

        // Example: close window when Escape is pressed
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
    println!("GLFW window closed successfully!");
}
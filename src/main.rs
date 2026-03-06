extern crate glfw;

use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use glow::{HasContext, MAX_WIDTH};
use core::panic;
use std::f32::consts::PI;


struct Engine {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    gl: glow::Context,

    shader_program: glow::NativeProgram,

    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,

    height: i32,
    width: i32,
}
 // ctrl + alt to enable inlay hints
impl Engine {

    fn new(height:i32, width:i32) -> Self {
        use glfw::fail_on_errors;

        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        let (mut window, events) = glfw
        .create_window(640, 480, "engine test", glfw::WindowMode::Windowed)
        .expect("failed to create window");

        window.make_current();
        window.set_key_polling(true);
        
        let gl = 
        unsafe {
            glow::Context::from_loader_function(|s| 
                window.get_proc_address(s)
                .map(|f| f as *const _)
                .unwrap_or(std::ptr::null()))
        };

        unsafe {
            println!(
                "OpenGL version: {}",
                gl.get_parameter_string(glow::VERSION)
            );
        }

        let shader_program = unsafe {
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex_shader, r#"
                #version 330 core
                layout(location = 0) in vec2 aPos;
                layout(location = 1) in vec3 aColor;

                out vec3 vertexColor;

                void main(){
                    gl_Position = vec4(aPos + uOffset, 0.0, 1.0);
                    vertexColor = aColor;
            "#);

            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                panic!("{}", gl.get_shader_info_log(vertex_shader))
            }

            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment_shader, r#"
                #version 330 core
                in vec3 vertexColor;
                out vec4 FragColor;
                void main() {
                    FragColor = vec4(vertexColor, 1.0);
                }
            "#);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                panic!("{}", gl.get_shader_info_log(fragment_shader))
            }

            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program))
            }

            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);

            program
        };

        let (vao, vbo) = unsafe {
            let quadVerticies [32f:]
        }

        Engine {
            glfw,
            window,
            events,
            gl, 
            shader_program,
            height,
            width
        }
    }

    pub fn circle(&mut self, radius:f32) {
        const VERT_COUNT: usize = 100;
        let mut vert:[f32; VERT_COUNT * 5] = [0.0; VERT_COUNT*5];

        for i in 0..VERT_COUNT {
            let angle = ((i as f32) / VERT_COUNT as f32) * 2.0 * PI;
            let x = f32::cos(angle) * radius;
            let y = f32::sin(angle) * radius;
            
            vert[i*5] = x;
            vert[i*5 + 1] = y;
            
            vert[i*5 + 2] = 1.0; // R
            vert[i*5 + 3] = 1.0; // G
            vert[i*5 + 4] = 1.0; // B
        }

        unsafe {
            let vao = self.gl.create_vertex_array().unwrap();
            let vbo = self.gl.create_buffer().unwrap();

            self.gl.bind_vertex_array(Some(vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(
                    vert.as_ptr() as *const u8,
                    vert.len() * std::mem::size_of::<f32>()
                ),
                glow::STATIC_DRAW
            );

            // Position attribute (x, y)
            self.gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 5 * 4, 0);
            self.gl.enable_vertex_attrib_array(0);

            // Color attribute (r, g, b)
            self.gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 5 * 4, 2 * 4);
            self.gl.enable_vertex_attrib_array(1);

            self.gl.draw_arrays(glow::TRIANGLE_FAN, 0, VERT_COUNT as i32);
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            unsafe {
                self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
                self.gl.clear(glow::COLOR_BUFFER_BIT);

                // Bind shader
                self.gl.use_program(Some(self.shader_program));

                // Bind VAO
                self.gl.bind_vertex_array(Some(self.circle_vao));

                // Draw
                self.gl.draw_arrays(glow::TRIANGLE_FAN, 0, VERT_COUNT as i32);
            }

            // Swap buffers
            self.window.swap_buffers();
            // Poll events
            self.glfw.poll_events();

            self.circle(20.0);

            // Example: close window when Escape is pressed
            for (_, event) in glfw::flush_messages(&self.events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }
}


fn main() {

    let mut engine = Engine::new(400, 400);
    engine.run()
    
}
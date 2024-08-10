use gl::types::{GLsizei, GLsizeiptr, GLuint};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::time::Duration;

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
    [-0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
    [0.5, 0.5, 0.0],
    [-0.5, 0.5, 0.0],
];

const INDICES: [TriIndexes; 2] = [[0, 3, 1], [1, 3, 2]];

const VERTEX_SHADER_SOURCE: &str = r#"
      #version 330 core
      layout (location = 0) in vec3 aPos;

      void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
      }
    "#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
      #version 330 core

      uniform vec4 uni_color;

      out vec4 final_color;

      void main() {
        final_color = uni_color;
      }
    "#;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Rust SDL2 OpenGL", 800, 600)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let _gl_context = window.gl_create_context()?;
    let _gl: () =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let vertex_shader = create_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = create_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program = create_program(vertex_shader, fragment_shader);

    let (vao, vbo, ebo) = unsafe { create_buffers() };

    let uni_color_loc = unsafe {
        gl::UseProgram(shader_program);
        gl::GetUniformLocation(shader_program, b"uni_color\0".as_ptr() as *const _)
    };

    let mut event_pump = sdl_context.event_pump()?;

    unsafe {
        gl::ClearColor(0., 0., 0., 1.0);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let time = (sdl_context.timer().unwrap().ticks() as f32) / 1000.0;
        let green = (time.sin() / 2.0) + 0.5;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::Uniform4f(uni_color_loc, 0.1, green, 0.1, 1.0);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // 프로그램 종료 시 정리
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
        gl::DeleteProgram(shader_program);
    }

    Ok(())
}

unsafe fn create_buffers() -> (GLuint, GLuint, GLuint) {
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);

    gl::BindVertexArray(vao);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (VERTICES.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
        VERTICES.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (INDICES.len() * std::mem::size_of::<TriIndexes>()) as GLsizeiptr,
        INDICES.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vertex>() as GLsizei,
        std::ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    (vao, vbo, ebo)
}

fn create_shader(source: &str, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &(source.as_ptr() as *const i8),
            &(source.len() as gl::types::GLint),
        );
        gl::CompileShader(shader);
    }
    shader
}

fn create_program(
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
) -> gl::types::GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    program
}

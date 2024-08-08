use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::time::Duration;

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

    const VERTEX_SHADER_SOURCE: &str = r#"
      #version 330 core
      layout (location = 0) in vec3 aPos;
      void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
      }
    "#;

    const FRAGMENT_SHADER_SOURCE: &str = r#"
      #version 330 core
      out vec4 FragColor;
      void main() {
        FragColor = vec4(1.0, 0.5, 0.2, 1.0);
      }
    "#;

    let vertex_shader = create_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = create_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program = create_program(vertex_shader, fragment_shader);

    type Vertex = [f32; 3];
    type TriIndexes = [u32; 3];

    let vertices: [Vertex; 4] = [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.5, 0.5, 0.0],
        [-0.5, 0.5, 0.0],
    ];

    const indices: [TriIndexes; 2] = [[0, 3, 1], [1, 3, 2]];

    let (vao, vbo, ebo) = unsafe {
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
            (vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<TriIndexes>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (vao, vbo, ebo)
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

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
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

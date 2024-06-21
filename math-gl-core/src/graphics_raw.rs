use glfw::{Context, GlfwReceiver, WindowEvent};
pub fn file2string(filename: &str) -> String {
    return std::fs::read_to_string(filename).unwrap();
}
pub fn read_both(vertex: &str, fragment: &str) -> (String, String) {
    let vertex_source = std::fs::read_to_string(vertex).unwrap();
    let fragment_source = std::fs::read_to_string(fragment).unwrap();

    return (vertex_source.to_string(), fragment_source.to_string());
}
#[allow(non_snake_case)]
pub fn glfwInit(
    width: u32,
    height: u32,
) -> (glfw::Glfw, glfw::PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    let mut glfw_ = glfw::init_no_callbacks().unwrap(); //glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    unsafe {
        // glfwWindowHint(glfw::WindowHint::, hint)

        glfw::ffi::glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MAJOR, 4);
        glfw::ffi::glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MINOR, 3);
        glfw::ffi::glfwWindowHint(glfw::ffi::OPENGL_PROFILE, glfw::ffi::OPENGL_CORE_PROFILE);
        glfw::ffi::glfwWindowHint(glfw::ffi::SAMPLES, 16);
    }

    let (mut window, events) = glfw_
        .create_window(width, height, "Cycloid", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    glfw_.set_swap_interval(glfw::SwapInterval::Sync(2));
    gl::load_with(|f| glfw_.get_proc_address_raw(f));
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // glBlendFuncSeparate(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA, GL_ONE, GL_ONE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
    }

    println!("{}", gl::ClearColor::is_loaded());
    return (glfw_, window, events);
}
pub mod Shaders {
    // use crate::GetCompiledShader;
    #[allow(non_snake_case)]
    // pub fn GetCompiledShader(shader_type: GLenum, shader_source: &String) -> GLuint {
    pub fn GetCompiledShader(
        shader_type: gl::types::GLenum,
        shader_source: &String,
    ) -> gl::types::GLuint {
        // let source_ptr = shader_source as *const GLchar;
        let sources = vec![shader_source.as_ptr() as *const i8];
        let sources_len = vec![shader_source.len() as i32];
        // let source_len = vec![]
        let mut shader: u32 = 0;
        unsafe {
            shader = gl::CreateShader(shader_type);
            // let source_ptr2 = &source_ptr;
            gl::ShaderSource(shader, 1, sources.as_ptr(), sources_len.as_ptr());
            gl::CompileShader(shader);

            //error checking
            let mut err_msg_len: i32 = 0;
            let mut err_msg: Vec<u8> = Vec::new();
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut err_msg_len);
            if err_msg_len == 0 {
                println!("Error Compiling shader :{:x}", shader_type);
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut err_msg_len);
                err_msg.resize(err_msg_len as usize, 0);
                gl::GetShaderInfoLog(
                    shader,
                    err_msg_len,
                    &mut err_msg_len,
                    (&mut err_msg[0]) as *mut u8 as _,
                );
                let k = String::from_utf8(err_msg).unwrap();
                println!("Error:{}", k);
                return 0;
            }
        }
        println!("Done Compiling...");
        return shader;
    }
    #[allow(non_snake_case)]
    pub fn CompileProgram(
        vertex_filename: &str,
        fragment_filename: &str,
        geometry_filename: Option<&str>,
    ) -> gl::types::GLuint {
        let (v_source, f_source) = super::read_both(vertex_filename, fragment_filename);
        unsafe {
            let vertex = GetCompiledShader(gl::VERTEX_SHADER, &v_source);
            let fragment = GetCompiledShader(gl::FRAGMENT_SHADER, &f_source);

            let Prog = gl::CreateProgram();
            gl::AttachShader(Prog, vertex);
            gl::AttachShader(Prog, fragment);
            if let Some(geometry_shader_filename) = geometry_filename {
                let g_shader = std::fs::read_to_string(geometry_shader_filename)
                    .expect("Couldn't read geometry shader!");
                let geometry = GetCompiledShader(gl::GEOMETRY_SHADER, &g_shader);
                gl::AttachShader(Prog, geometry);
                gl::LinkProgram(Prog);
                gl::DeleteShader(geometry);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            } else {
                gl::LinkProgram(Prog);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            }
            return Prog;
        }
    }

    #[allow(non_snake_case)]
    pub fn CompileProgramFromString(
        vertex_source: String,
        fragment_source: String,
        geometry_source: Option<String>,
    ) -> gl::types::GLuint {
        let (v_source, f_source) = (vertex_source, fragment_source); //super::read_both(vertex_filename, fragment_filename);
        unsafe {
            let vertex = GetCompiledShader(gl::VERTEX_SHADER, &v_source);
            let fragment = GetCompiledShader(gl::FRAGMENT_SHADER, &f_source);

            let Prog = gl::CreateProgram();
            gl::AttachShader(Prog, vertex);
            gl::AttachShader(Prog, fragment);
            if let Some(g_shader) = geometry_source {
                // let g_shader = std::fs::read_to_string(geometry_shader_filename)
                // .expect("Couldn't read geometry shader!");
                let geometry = GetCompiledShader(gl::GEOMETRY_SHADER, &g_shader);
                gl::AttachShader(Prog, geometry);
                gl::LinkProgram(Prog);
                gl::DeleteShader(geometry);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            } else {
                gl::LinkProgram(Prog);
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
            }
            return Prog;
        }
    }

    pub fn ComputeShader(cshader_source: &String) -> u32 {
        unsafe {
            let COMP = gl::CreateProgram();
            let C = GetCompiledShader(gl::COMPUTE_SHADER, cshader_source); //CompileShaderSource(gl::COMPUTE_SHADER, &cshader);
            gl::AttachShader(COMP, C);
            gl::LinkProgram(COMP);
            gl::DeleteShader(C);
            return COMP;
        }
    }
}

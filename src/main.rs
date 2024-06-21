mod projection_box;

use glfw::{ffi::glfwGetTime, Context, Window};
use math_gl_core::{
    basic_components::{MathGrid, OrthogonalCoordinates},
    functions::ParametricLine,
    glbuffers::{ArrayBuffer, GLAttrib},
    graphics_raw::{self, file2string, Shaders::ComputeShader},
    la_projection::{ProjectionCompute, ProjectionPlane},
    scene::GLWindow,
    shaders::{
        self, BasicShader, Camera, CameraProjection, CameraView, ShaderCreator, ShaderProgram,
    },
};
use nalgebra_glm::{Vec3, Vec4};
use std::{ffi::c_void, time::Duration};

fn wavelet(t: f32) -> Vec4 {
    let gaussian = f32::exp(-t * t);
    Vec4::new(
        t,
        gaussian * f32::cos(20.0 * t),
        gaussian * f32::sin(20.0 * t),
        1.0,
    )
}

fn compute_it(vbo: u32) -> ArrayBuffer {
    unsafe {
        let compute_shader = ComputeShader(&file2string("shaders/projection_compute.glsl"));
        let mut ssbo = 0u32;
        let mut vao = 0u32;

        //Bind VBO to the compute shader

        // gl::GenVertexArrays(1, &mut vao);
        // gl::BindVertexArray(vao);
        // // Generate and bind SSBO
        // gl::GenBuffers(1, &mut ssbo);
        // gl::BindBuffer(gl::ARRAY_BUFFER, ssbo);
        // gl::BufferData(gl::ARRAY_BUFFER, 5000 * 8 * 4, 0 as _, gl::DYNAMIC_DRAW);
        // GLAttrib::new(0, 4, 8, 0).set();
        // GLAttrib::new(1, 4, 8, 4).set();

        // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // gl::BindVertexArray(0);

        let mut buffer = ArrayBuffer::new();
        buffer
            .allocateBuffer(0 as _, 5000 * 8 * 4)
            .setAttributes(&vec![GLAttrib::new(0, 4, 8, 0), GLAttrib::new(1, 4, 8, 4)]);
        let mut ssbo = buffer.VBO.0;
        let mut vao = buffer.VAO.0;
        // gl::BindVertexArray(vao);
        // gl::BindBuffer(gl::ARRAY_BUFFER, ssbo);

        // unsafe{forget(buffer)};

        // Generating projection matrix

        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, vbo);
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, ssbo);
        // gl::BindVertexArray(0);
        gl::UseProgram(compute_shader);

        // Get the uniform location

        let matrix = ProjectionPlane::new(Vec3::new(1.0, 1.0, 1.0), 0.0).projection_matrix;

        //nalgebra_glm::Mat3::new(1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 3.0f32);
        let uniform_location = gl::GetUniformLocation(compute_shader, "projection\0".as_ptr() as _);
        gl::UniformMatrix3fv(uniform_location, 1, 0, matrix.as_ptr() as _);

        // Dispatch compute shader
        gl::DispatchCompute(5000, 1, 1);

        gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);

        let mut tmp = vec![(Vec4::new(0.0, 0.0, 0.0, 0.0), Vec4::new(0.0, 0.0, 0.0, 0.0)); 5000];
        gl::GetNamedBufferSubData(ssbo, 0, 5000 * 8 * 4, tmp.as_mut_ptr() as _);
        for i in 0..10 {
            println!("{:?}", tmp[i]);
        }

        // Unbind SSBO
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);

        return buffer;
    }
}

fn main() {
    let mut window = GLWindow::new_configured(1000, 1000);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut grid = MathGrid::new_full(
        OrthogonalCoordinates::default(),
        (5, 5),
        (3, 3),
        (5.0, 5.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Some(Vec4::new(0.2, 0.8, 1.0, 0.70)),
        Some(Vec4::new(1.0, 1.0, 1.0, 0.20)),
        None,
    );

    let mut wavelet = ParametricLine::new_line(
        wavelet,
        -5.0,
        5.0,
        5000,
        Some(|t: f32| {
            Vec4::new(
                0.8 + 0.2 * f32::sin(t),
                0.0, //0.5 + 0.5 * f32::sin(3.5 * t),
                0.8 + 0.2 * f32::sin(4.5 * t),
                1.0,
            )
        }),
    );

    let mut Cam = Camera::new(CameraProjection::default(), CameraView::default());
    Cam.add_program(grid.get_program())
        .add_program(wavelet.get_program()) //.add_program(param_shader.program_id)
        .set_perspective(1.0, 3.14 / 2.0, 0.01, 100.0)
        .update_uniforms();

    // let buff = compute_it(wavelet.shader.buffer.VBO.0);
    // let (ssbo, vao) = (buff.VBO.0, buff.VAO.0);
    // window.m_glfw.set_swap_interval(glfw::SwapInterval::Sync(2));
    // std::thread::sleep(Duration::from_secs(1));

    let proj_compute = ProjectionCompute::new(None);
    let mut buff = proj_compute.generate_projection_for_array_buffer(
        &wavelet.shader.buffer,
        &ProjectionPlane::new(Vec3::new(0.0001, 1.0, 0.0001), 1.0).projection_matrix,
    );

    while !window.m_window.should_close() {
        let t = unsafe { glfwGetTime() } as f32;
        Cam.get_camera_view()
            .set_eye(&(2.0 * Vec3::new(f32::sin(t / 10.0), f32::cos(t / 10.0), 1.0)))
            .update_matrix();

        Cam.update_uniforms();
        window.m_glfw.poll_events();
        grid.clean_draw();
        // wavelet.draw_on(3.0);
        unsafe {
            gl::LineWidth(3.0);
            gl::UseProgram(wavelet.get_program());
            // buff.bindAB();
            gl::BindVertexArray(buff.VAO.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, buff.VBO.0);
            //wavelet.shader.buffer.bindAB();
            gl::DrawArrays(gl::LINE_STRIP, 0, 5000);
        }
        window.m_window.swap_buffers();
    }
}

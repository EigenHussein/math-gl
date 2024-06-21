use function_sweep::{sweep, sweep_vec4};
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
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use std::{f32::consts::PI, ffi::c_void, marker::PhantomData, time::Duration};
use math_gl_core::texture_utility::FrameBuffer;

mod function_sweep;
mod lorenz;
mod projection_box;


pub struct VirtualScreen {
    shader: BasicShader,
    camera: Camera,
    frame_buffer: FrameBuffer,
    simple_fbo: u32,
    simple_tx: u32,
}

impl VirtualScreen {
    pub fn get_quad() -> [(Vec4, Vec4, Vec2); 4] {
        let quad_vertices = [
            (
                Vec4::new(-1.0, -1.0, 0.0, 1.0),
                Vec4::new(1.0, 1.0, 1.0, 1.0),
                Vec2::new(0.0, 0.0),
            ),
            (
                Vec4::new(-1.0, 1.0, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 1.0),
                Vec2::new(0.0, 1.0),
            ),
            (
                Vec4::new(1.0, 1.0, 0.0, 1.0),
                Vec4::new(0.0, 1.0, 0.0, 1.0),
                Vec2::new(1.0, 1.0),
            ),
            (
                Vec4::new(1.0, -1.0, 0.0, 1.0),
                Vec4::new(0.0, 0.0, 1.0, 1.0),
                Vec2::new(1.0, 0.0),
            ),
        ];
        quad_vertices
    }
    pub fn new(width: i32, height: i32) -> Self {
        let quad_vertices = Self::get_quad();
        let indices = [0, 1, 2, 0, 2, 3];

        let mut texture_prog =
            BasicShader::new(ShaderCreator::new(shaders::ColorMode::ColorBloom, false).program_id);
        let mut cam = Camera::new(
            CameraProjection::default_perspective(1.0, 1.5),
            CameraView::default(),
        );

        texture_prog.buffer.allocateBuffer(
            quad_vertices.as_ptr() as _,
            std::mem::size_of_val(&quad_vertices) as _,
        );
        println!(
            "sizes: {}\t{}",
            std::mem::size_of_val(&indices),
            std::mem::size_of_val(&quad_vertices)
        );
        texture_prog
            .buffer
            .allocateElements(indices.as_ptr() as _, std::mem::size_of_val(&indices) as _);
        texture_prog.buffer.setAttributes(&vec![
            GLAttrib::new(0, 4, 10, 0),
            GLAttrib::new(1, 4, 10, 4),
            GLAttrib::new(3, 2, 10, 8),
        ]);

        cam.add_program(texture_prog.get_program());
        cam.set_projection_matrix(Mat4::identity());
        cam.set_view_matrix(Mat4::identity());

        let virtualframe = FrameBuffer::new(width, height);

        cam.update_uniforms();

        let (mut s_fbo, mut s_tex) = (0u32, 0u32);
        unsafe {
            gl::GenFramebuffers(1, &mut s_fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, s_fbo);
            gl::GenTextures(1, &mut s_tex);
            gl::BindTexture(gl::TEXTURE_2D, s_tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA32F as _,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                0 as _,
            );
            FrameBuffer::texture_set_default_parameters();
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                s_tex,
                0,
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Self {
            shader: texture_prog,
            camera: cam,
            frame_buffer: virtualframe,
            simple_fbo: s_fbo,
            simple_tx: s_tex,
        }
    }
    pub fn render_new_to_frame_buffer<T>(&self, mut render_call: T)
    where
        T: FnMut() -> (),
    {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer.fbo);
            // gl::Viewport(0, 0, self.frame_buffer.width, self.frame_buffer.height);
            // gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            // gl::UseProgram(self.shader.program);
            render_call();
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::BlitNamedFramebuffer(
                self.frame_buffer.fbo,
                self.simple_fbo,
                0,
                0,
                1024,
                1024,
                0,
                0,
                1024,
                1024,
                gl::COLOR_BUFFER_BIT,
                gl::LINEAR,
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.simple_tx);
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::ClearColor(0.01, 0.01, 0.01, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(self.shader.program);
            self.shader.buffer.bindAB();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
        }
    }
}
fn from_hex(hex: &str) -> Vec4 {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;
    Vec4::new(r, g, b, 1.0)
}
fn grad_color(t: f32) -> Vec4 {
    let s1 = sweep_vec4(&from_hex("00F260"), &from_hex("0575E6"), (5.0 + t) / 10.0);
    let s2 = sweep_vec4(&from_hex("0575E6"), &from_hex("00F260"), (5.0 + t) / 10.0);
    10.0*sweep_vec4(&s1, &s2, (5.0 + t) / 10.0)
    //s1
}

fn main() {
    let mut window = GLWindow::new_configured(1024, 1024);
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut virt_screen = VirtualScreen::new(1024, 1024);

    // let mut per_vertex =
    //     BasicShader::new(ShaderCreator::new(shaders::ColorMode::ColorPerVertex, false).program_id);
    // let mut cam = Camera::new(
    //     CameraProjection::default_perspective(1.0, 1.5),
    //     CameraView::default(),
    // );
    // cam.add_program(per_vertex.get_program());
    // cam.set_projection_matrix(Mat4::identity());
    // cam.set_view_matrix(Mat4::identity());

    let mut parametric = ParametricLine::new_line(
        |t: f32| Vec4::new(f32::sin(10.0 * t), f32::cos(10.0 * t), t, 1.0),
        -5.0,
        5.0,
        1000,
        Some(|t: f32| grad_color(t)),
    );

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );
    cam.add_program(parametric.get_program());
    cam.get_camera_view().set_eye(&Vec3::new(5.0, 5.0, 5.0)).update_matrix();
    cam.update_uniforms();

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        virt_screen.render_new_to_frame_buffer(|| {
            unsafe {
                // gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
            parametric.draw_on(0,3.0);
        });

        // virt_screen.render_new_to_frame_buffer(|| unsafe {
        //     gl::ClearColor(0.5, 0.0, 0.0, 1.0);
        //     gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        // });
        window.m_window.swap_buffers();
    }
}
fn main2() {
    let mut window = GLWindow::new_configured(1024, 1024);

    // let quad_vertices = [
    //     (
    //         Vec4::new(-1.0, -1.0, 0.0, 1.0),
    //         Vec4::new(1.0, 1.0, 1.0, 1.0),
    //         Vec2::new(0.0, 0.0),
    //     ),
    //     (
    //         Vec4::new(-1.0, 1.0, 0.0, 1.0),
    //         Vec4::new(1.0, 0.0, 0.0, 1.0),
    //         Vec2::new(0.0, 1.0),
    //     ),
    //     (
    //         Vec4::new(1.0, 1.0, 0.0, 1.0),
    //         Vec4::new(0.0, 1.0, 0.0, 1.0),
    //         Vec2::new(1.0, 1.0),
    //     ),
    //     (
    //         Vec4::new(1.0, -1.0, 0.0, 1.0),
    //         Vec4::new(0.0, 0.0, 1.0, 1.0),
    //         Vec2::new(1.0, 0.0),
    //     ),
    // ];

    // println!("{:?}", std::mem::size_of_val(&quad_vertices));
    // let indices = [0, 1, 2, 0, 2, 3];

    // let mut buffer = ArrayBuffer::new();
    // buffer.allocateBuffer(quad_vertices.as_ptr() as _, 40 * 4);
    // buffer.allocateElements(indices.as_ptr() as _, 4 * 6);
    // buffer.setAttributes(&vec![
    //     GLAttrib::new(0, 4, 10, 0),
    //     GLAttrib::new(1, 4, 10, 4),
    //     GLAttrib::new(3, 2, 10, 8),
    // ]);

    // let mut texture_prog =
    //     BasicShader::new(ShaderCreator::new(shaders::ColorMode::ColorBloom, false).program_id);
    // let mut cam = Camera::new(
    //     CameraProjection::default_perspective(1.0, 1.5),
    //     CameraView::default(),
    // );

    let mut vs = VirtualScreen::new(1024, 1024);

    let mut per_vertex =
        BasicShader::new(ShaderCreator::new(shaders::ColorMode::ColorPerVertex, false).program_id);
    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );

    cam.set_projection_matrix(Mat4::identity());
    cam.set_view_matrix(Mat4::identity());

    cam.add_program(per_vertex.get_program())
        .add_program(vs.shader.get_program());
    cam.update_uniforms();

    // println!("HAH");
    // let virtualframe = texture_utility::FrameBuffer::new(1024, 1024);
    // println!("AHAHD");
    // unsafe {
    //     virtualframe.bind();
    //     gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    //     gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //     // gl::Viewport(0, 0, 1024, 1024)
    //     gl::UseProgram(per_vertex.get_program());
    //     vs.shader.buffer.bindAB();
    //     gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
    //     virtualframe.unbind();
    //     gl::ActiveTexture(gl::TEXTURE0);
    //     gl::BindTexture(gl::TEXTURE_2D, virtualframe.fbo_texture);
    //     gl::GenerateMipmap(gl::TEXTURE_2D);
    // }

    // unsafe {
    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    // }
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        unsafe {
            gl::UseProgram(vs.shader.get_program());
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            // gl::BindVertexArray(0);

            vs.shader.buffer.bindAB();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
        }
        window.m_window.swap_buffers();
    }
}

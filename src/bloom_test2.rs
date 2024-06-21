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
use nalgebra_glm::{Vec3, Vec4};
use std::{f32::consts::PI, ffi::c_void, marker::PhantomData, time::Duration};
use texture_utility::VirtualScreen;

mod function_sweep;
mod lorenz;
mod projection_box;

fn from_hex(hex: &str) -> Vec4 {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;
    Vec4::new(r, g, b, 1.0)
}

fn grad_color(t: f32) -> Vec4 {
    let s1 = sweep_vec4(&from_hex("00F260"), &from_hex("0575E6"), (5.0 + t) / 10.0);
    let s2 = sweep_vec4(&from_hex("0575E6"), &from_hex("00F260"), (5.0 + t) / 10.0);
    //Vec4::new(5.0,5.0,0.0,1.0)
    sweep_vec4(&s1, &s2, (5.0 + t) / 10.0)
    //s1
}

fn wavelet_color(t: f32) -> Vec4 {
    Vec4::new(
        0.8 + 0.2 * f32::sin(t),
        0.0, //0.5 + 0.5 * f32::sin(3.5 * t),
        0.8 + 0.2 * f32::sin(4.5 * t),
        1.0,
    )
}

fn wavelet_func(t: f32) -> Vec4 {
    let gaussian = f32::exp(-t * t);
    Vec4::new(
        t,
        gaussian * f32::cos(12.0 * t),
        gaussian * f32::sin(12.0 * t),
        1.0,
    )
}
fn circle1(t: f32) -> Vec4 {
    let gaussian = 1.0; //f32::exp(-t * t);
    Vec4::new(
        0.0,
        gaussian * f32::cos(12.0 * t),
        gaussian * f32::sin(12.0 * t),
        1.0,
    )
}
fn circle(t: f32) -> Vec4 {
    let gaussian = 1.0; //f32::exp(-t * t);
    Vec4::new(
        t,
        gaussian * f32::cos(12.0 * t),
        gaussian * f32::sin(12.0 * t),
        1.0,
    )
}

fn cycloid(t2: f32) -> Vec4 {
    let t = 2.0 * t2;
    0.3 * Vec4::new(
        f32::sin(15.0 * t),
        f32::sin(t) + 2.0 * f32::sin(14.0 * t),
        f32::cos(t) + 2.0 * f32::cos(14.0 * t),
        3.3333,
    )
}

fn cycloid2(t2: f32) -> Vec4 {
    let t = 2.0 * t2;
    // 0.3 * Vec4::new(
    //     f32::sin(16.0 * t),
    //     f32::sin(t) + 1.5 * f32::sin(17.0 * t),
    //     f32::cos(t) + 1.5 * f32::cos(17.0 * t),
    //     3.3333,
    // )

    let t = 2.0 * t2;
    0.3 * Vec4::new(
        f32::sin(26.0 * t),
        f32::sin(t) + 1.5 * f32::sin(27.0 * t),
        f32::cos(t) + 1.5 * f32::cos(27.0 * t),
        3.3333,
    )
}
fn cycloid3(t2: f32) -> Vec4 {
    let t = 2.0 * t2;
    0.3 * Vec4::new(
        f32::sin(9.0 * t),
        f32::sin(t) + 1.5 * f32::sin(27.0 * t),
        f32::cos(t) + 1.5 * f32::cos(27.0 * t),
        3.3333,
    )
}
fn cycloid4(t2: f32) -> Vec4 {
    let t = 2.0 * t2;
    // 0.7 * Vec4::new(
    //     f32::sin(9.0 * t),
    //     f32::cos(t) + f32::cos(52.40 * t) / 3.0 + f32::sin(25.0 * t) / 5.0,
    //     f32::sin(t) + f32::sin(52.40 * t) / 3.0 + f32::cos(25.0 * t) / 5.0,
    //     1.0/0.7,
    // )
    0.5 * Vec4::new(
        f32::sin(100.0 * t),
        f32::cos(t) - f32::cos(50.0 * t) * f32::sin(t),
        2.0 * f32::sin(t) - f32::sin(50.0 * t),
        1.0 / 0.5,
    )
}
fn cycloid5(t2: f32) -> Vec4 {
    let t = 4.0 * t2;
    // 0.7 * Vec4::new(
    //     f32::sin(9.0 * t),
    //     f32::cos(t) + f32::cos(52.40 * t) / 3.0 + f32::sin(25.0 * t) / 5.0,
    //     f32::sin(t) + f32::sin(52.40 * t) / 3.0 + f32::cos(25.0 * t) / 5.0,
    //     1.0/0.7,
    // )
    0.5 * Vec4::new(
        f32::sin(100.0 * t),
        f32::cos(0.50 * t) - f32::cos(50.0 * t) * f32::sin(0.5 * t),
        1.0 * f32::sin(0.50 * t) - f32::sin(50.0 * t),
        1.0 / 0.5,
    )
}
fn sig(t: f32) -> f32 {
    1.0 / (1.0 + f32::exp(10.0 - 20.0 * t))
}

use math_gl_core::texture_utility;//mod texture_utility;

fn main() {
    let mut window = GLWindow::new_configured(1024, 1024);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    let mut virt_screen = VirtualScreen::new(1024, 1024);
    let mut proj_box = projection_box::ProjectionBox::new(2, 2, 2, Some(0.5), Some(0.5), Some(0.5));
    let mut wavelet = ParametricLine::new_line(wavelet_func, -5.0, 5.0, 5000, Some(grad_color));
    let mut my_functions =
        function_sweep::FunctionCollection::new(Box::new(circle1), 2.0, Some(Box::new(sig)));

    my_functions
        .add_function(Box::new(circle), 4.0, Some(Box::new(sig)))
        .add_function(Box::new(wavelet_func), 2.0, Some(Box::new(sig)))
        .add_function(Box::new(cycloid), 2.0, Some(Box::new(sig)))
        .add_function(Box::new(cycloid5), 4.50, Some(Box::new(sig)))
        .add_function(Box::new(cycloid4), 6.0, Some(Box::new(sig)));

    let mut cam = Camera::new(
        //CameraProjection::normalized_ortho(-5.0, 5.0, -5.0, 5.0, 0.001, 100.0),
        CameraProjection::perspective(1.0, 3.14 / 3.0, 0.01, 100.0),
        CameraView::default(),
    );
    proj_box.bind_camera(&mut cam);

    cam.add_program(wavelet.get_program())
        .set_camera(&Vec3::new(5.0, 5.0, 5.0), &Vec3::zeros(), &Vec3::z())
        .update_uniforms();

    proj_box.generate_projection(&wavelet.shader.buffer);

    let mut freq = 0.0f32;
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        let t0 = unsafe { glfwGetTime() } as f32 / 1.5 - 5.0;
        // println!("{t0}");
        cam.get_camera_view()
            .set_eye(
                &(8.0
                    * Vec3::new(
                        -f32::sin(t0 / 10.0 + 0.30),
                        -f32::cos(t0 / 10.0 + 0.30),
                        0.50 - 0.5 * sig(-5.0 + t0 / 2.0),
                    )),
            )
            .update_matrix();

        let proj_mat = function_sweep::sweep(
            &nalgebra_glm::ortho(-5.0, 5.0, -5.0, 5.0, 0.01, 100.0),
            &nalgebra_glm::perspective(1.0, 3.14 / 5.0, 0.01, 100.0),
            ((t0 + 5.0) / 6.0).min(1.0),
        );

        let final_proj = sweep(
            &proj_mat,
            &nalgebra_glm::ortho(-3.0, 3.0, -3.0, 3.0, 0.01, 100.0),
            sig(-10.0 + t0 / 1.0),
        );

        cam.set_projection_matrix(final_proj);
        cam.update_uniforms();

        virt_screen.render_new_to_frame_buffer(|| {
            proj_box.clean_draw_grid();
            {
                wavelet.update_function(0,-5.0, 5.0, |t| my_functions.eval(t0 + 5.0, t / 2.5));
                proj_box.update_projection(&wavelet.shader.buffer);
                freq += 0.002 * PI;
                println!("{:?}",t0);
            }
            wavelet.draw_on(0,2.0);
            proj_box.draw_projection_on(&wavelet.shader, 0.5);
            // println!("t0: {}\t", t0);
        });
        window.m_window.swap_buffers();
    }
}

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
        gaussian * f32::cos(2.0 * t),
        gaussian * f32::sin(2.0 * t),
        1.0,
    )
}

fn wavelet_color(t: f32) -> Vec4 {
    Vec4::new(
        0.8 + 0.2 * f32::sin(t),
        0.0, //0.5 + 0.5 * f32::sin(3.5 * t),
        0.8 + 0.2 * f32::sin(4.5 * t),
        1.0,
    )
}

mod lorenz;

fn main() {
    let mut window = GLWindow::new_configured(1000, 1000);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut grid1 = MathGrid::new_full(
        OrthogonalCoordinates::new(Vec3::new(0.0, 0.0, -2.0), Vec3::x(), Vec3::y()),
        (5, 2),
        (4, 4),
        (5.0, 2.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Some(Vec4::new(0.2, 0.8, 1.0, 0.70)),
        Some(Vec4::new(1.0, 1.0, 1.0, 0.30)),
        None,
    );

    let mut grid2 = MathGrid::new_full(
        OrthogonalCoordinates::new(
            Vec3::new(-5.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (2, 2),
        (4, 4),
        (2.0, 2.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Some(Vec4::new(0.2, 0.8, 1.0, 0.70)),
        Some(Vec4::new(0.5, 0.50, 0.50, 0.30)),
        Some(grid1.get_program()),
    );
    grid2.set_widths(1.0, 0.01, 0.01);

    let mut grid3 = MathGrid::new_full(
        OrthogonalCoordinates::new(
            Vec3::new(0.0, -2.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        (5, 2),
        (4, 4),
        (5.0, 2.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Some(Vec4::new(0.2, 0.8, 1.0, 0.70)),
        Some(Vec4::new(0.5, 0.50, 0.50, 0.30)),
        Some(grid1.get_program()),
    );

    let mut wavelet = ParametricLine::new_line(wavelet, -5.0, 5.0, 5000, Some(wavelet_color));

    let mut projection1 = ProjectionCompute::new(None);
    let mut proj0_AB = projection1.generate_projection_for_array_buffer(
        &wavelet.shader.buffer,
        &ProjectionPlane::new(Vec3::new(0.0, 0.0, 1.0), -2.0).projection_matrix,
    );

    let mut proj1_AB = projection1.generate_projection_for_array_buffer(
        &wavelet.shader.buffer,
        &ProjectionPlane::new(Vec3::x(), -5.0).projection_matrix,
    );
    let mut proj2_AB = projection1.generate_projection_for_array_buffer(
        &wavelet.shader.buffer,
        &ProjectionPlane::new(Vec3::new(-0.0001, 1.0, 0.0), -2.0).projection_matrix,
    );

    let mut cam = Camera::new(
        //CameraProjection::normalized_ortho(-10.0, 10.0, -10.0, 10.0, 0.001, 100.0),
        CameraProjection::perspective(1.0, 3.14 / 5.0, 0.01, 100.0),
        CameraView::default(),
    );
    cam.add_program(grid1.get_program())
        .add_program(grid2.get_program())
        .add_program(grid3.get_program())
        .add_program(wavelet.get_program())
        .set_camera(&Vec3::new(5.0, 5.0, 5.0), &Vec3::zeros(), &Vec3::z())
        .update_uniforms();

    let mut freq = 0.0f64;
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();

        let t = unsafe { glfwGetTime() } as f32 - 5.0;
        cam.get_camera_view()
            .set_eye(&(10.0 * Vec3::new(f32::sin(t / 10.0), f32::cos(t / 10.0), 1.0)))
            .update_matrix();
        cam.update_uniforms();

        grid1.clean_draw();
        grid2.draw_on();
        grid3.draw_on();
        {
            let (mut x, mut y, mut z) = (1.10, 1.10, 1.10);
            wavelet.update_function(0,-5.0, 5.0, move |t| {
                let gaussian = f32::exp(-t * t);
                // Vec4::new(
                //     t,
                //     gaussian * f32::cos(freq * t),
                //     gaussian * f32::sin(freq * t),
                //     1.0,
                // )
                // let a = f32::sin(0.53 * freq * t);
                // let b = f32::cos(0.33 * freq * t);

                lorenz::lorenz(&mut x, &mut y, &mut z, freq as f64);
                // println!("{:?}", (x, y, z));
                0.02 * Vec4::new(
                    x as f32, y as f32, z as f32,
                    // (1.0 + 0.5 * f32::cos(1.53 * t)),
                    // (1.0 + 0.5 * f32::cos(1.53 * t)) * a,
                    // (1.0 + 0.5 * f32::sin(3.15 * t)) * b,
                    50.0,
                )
            });
            freq += 0.00001;


            projection1.update_projection_for_array_buffer(
                &wavelet.shader.buffer,
                &mut proj0_AB,
                &ProjectionPlane::new(Vec3::new(0.0, 0.0, 1.0), -2.0).projection_matrix,
            );
            projection1.update_projection_for_array_buffer(
                &wavelet.shader.buffer,
                &mut proj1_AB,
                &ProjectionPlane::new(Vec3::x(), -5.0).projection_matrix,
            );
            projection1.update_projection_for_array_buffer(
                &wavelet.shader.buffer,
                &mut proj2_AB,
                &ProjectionPlane::new(Vec3::new(-0.0001, 1.0, 0.0), -2.0).projection_matrix,
            );
        }
        wavelet.draw_on(0,0.50);
        wavelet.draw_ab(&proj0_AB, 1.0);
        wavelet.draw_ab(&proj1_AB, 1.0);
        wavelet.draw_ab(&proj2_AB, 1.0);
        window.m_window.swap_buffers();
    }
}

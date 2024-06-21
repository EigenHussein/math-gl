use freetype::Library;
use math_gl_core::{
    texts_utility::{FreeTypeGenerator, Textures},
    texture_utility::VirtualScreen,
};
use nalgebra_glm::Vec3;

use gl;
use glfw::{Action, Context, WindowEvent};
use math_gl_core::{
    scene::GLWindow,
    shaders::{Camera, CameraProjection, CameraView},
};

fn main() {
    let mut _generator = FreeTypeGenerator::new(Library::init().unwrap(), None);

    let mut window = GLWindow::new_configured(1024, 1024);

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );

    let virt_screen = VirtualScreen::new(1024, 1024);
    // cam.add_program(virt_screen.shader.program);
    let mut my_texts = Textures::new();
    my_texts
        .add_latex_function(
            "$\\displaystyle\nf(a)=\\sum_{n=0}^{\\infty}{\\frac{f^{(n)}(a)}{n!} }$",
            0.4,
            None,
            None,
        )
        .add_text(
            "$\\displaystyl",
            0.5,
            200,
            Some(Vec3::new(0.10, 0.0, 0.2)),
            None,
        )
        .update_array_buffer()
        .add_camera(&mut cam);

    window.m_glfw.set_swap_interval(glfw::SwapInterval::Sync(2));
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            update_camera(&event, &mut cam);
        }

        virt_screen.render_new_to_frame_buffer(|| unsafe {
            // gl::UseProgram(my_texts)
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            my_texts.draw_textures();
        });
        window.m_window.swap_buffers();
    }
}

// fn main2() {
//     let mut window = GLWindow::new_configured(1024, 1024);

//     let shader = ShaderCreator::new(math_gl_core::shaders::ColorMode::ColorTexture, false);
//     let mut program = BasicShader::new(shader.program_id);

//     program
//         .buffer
//         .allocateBuffer(0 as _, 6 * 6 * 4)
//         // .allocate_buffer_from_slice(&[
//         //     (-1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32),
//         //     (1.0, 1.0, 0.0, 1.0, 1.0, 1.0),
//         //     (-1.0, 1.0, 0.0, 1.0, 0.0, 1.0),
//         //     (-1.0, -1.0, 0.0, 1.0, 0.0, 0.0),
//         //     (1.0, -1.0, 0.0, 1.0, 1.0, 0.0),
//         //     (1.0, 1.0, 0.0, 1.0, 1.0, 1.0),
//         // ])
//         .allocate_buffer_from_slice(
//             Texts::generate_texture_quad(
//                 1.0,
//                 0.4,
//                 Some(Vec3::new(0.0, 0.0, 0.5)),
//                 Some(Vec3::new(1.0, 1.0, 1.0)),
//             )
//             .as_slice(),
//         )
//         .setAttributes(&[GLAttrib::new(0, 4, 6, 0), GLAttrib::new(3, 2, 6, 4)].to_vec());

//     let mut tex_generator = latex_utility::LatexGenerator::new();
//     // for i in 0..10 {
//     let img = tex_generator
//         .set_function(
//             "$\\displaystyle\nf(a)=\\sum_{n=0}^{\\infty}{\\frac{f^{(n)}(a)}{n!} }$".to_string(),
//         )
//         .generate_image()
//         .load_gray_image();
//     // let img2 = tex_generator
//     //     .set_function("$\\displaystyle\nf(x)=\\sum_{n=0}^{\\infty}{HEHE}$".to_string())
//     //     .generate_image()
//     //     .load_image()
//     //     .to_rgba8();
//     // println!("{:?}", img2);
//     // // }

//     // println!("{:?}",img);

//     let equation_texture = Texture::new_mono(img.width() as _, img.height() as _, img.as_ptr());
//     //let equation_texture = Texture::new_rgba(img2.width() as _, img2.height() as _, img2.as_ptr() as _);
//     unsafe {
//         gl::UseProgram(program.get_program());
//         equation_texture.set_active_texture(0).bind_texture();
//         //gl::Uniform1i(gl::GetUniformLocation(program.get_program(), "TX\0".as_ptr() as _), 0);
//     }

//     let mut cam = Camera::new(
//         CameraProjection::default_perspective(1.0, 1.5),
//         CameraView::default(),
//     );
//     cam.add_program(program.get_program()).update_uniforms();

//     window.m_glfw.set_swap_interval(glfw::SwapInterval::Sync(2));
//     while !window.m_window.should_close() {
//         window.m_glfw.poll_events();
//         for (_, event) in glfw::flush_messages(&window.m_receiver) {
//             update_camera(&event, &mut cam);
//         }
//         unsafe {
//             gl::ClearColor(0.0, 0.0, 0.2, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
//             gl::UseProgram(shader.program_id);
//             program.buffer.bindAB();
//             gl::DrawArrays(gl::TRIANGLES, 0, 6);
//         }
//         window.m_window.swap_buffers();
//     }
// }

fn update_camera(ev: &WindowEvent, camera_instance: &mut Camera) {
    match ev {
        WindowEvent::Key(key, scancode, action, modifier) => {
            let act = *action;
            if *action == Action::Repeat || *action == Action::Press {
                match *key {
                    glfw::Key::A => {
                        camera_instance
                            .get_camera_view()
                            .increase_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::D => {
                        camera_instance
                            .get_camera_view()
                            .decrease_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::W => {
                        camera_instance
                            .get_camera_view()
                            .increase_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::S => {
                        camera_instance
                            .get_camera_view()
                            .decrease_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }

                    _ => {}
                };
            }
        }
        _ => {}
    };
}

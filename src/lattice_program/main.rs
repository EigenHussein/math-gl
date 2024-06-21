use std::mem::{size_of, size_of_val};

use gl;
use glfw::{Action, Context, WindowEvent};
use math_gl_core::{
    glbuffers::{ArrayBuffer, GLAttrib},
    scene::GLWindow,
    shaders::{BasicShader, Camera, CameraProjection, CameraView, ShaderCreator, ShaderProgram},
};
// mod utility;

use assimp::{self, import, Importer};
use nalgebra_glm::{Mat4, Vec3, Vec4};

mod utility;
use utility::{Bonds, *};

// fn slice_to_arraybuffer<T: Sized>(vao: Option<u32>, data: &[T], binding: Option<u32>) -> u32 {
//     let mut vbo = 0u32;
//     unsafe {
//         gl::GenBuffers(1, &mut vbo);
//         if let Some(valid_vao) = vao {
//             gl::BindVertexArray(valid_vao);
//         }

//         gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, vbo);
//         gl::BufferData(
//             gl::SHADER_STORAGE_BUFFER,
//             (std::mem::size_of::<T>() * data.len()) as _,
//             data.as_ptr() as _,
//             gl::STATIC_DRAW,
//         );
//         if let Some(binding_point) = binding {
//             gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding_point, vbo);
//         }
//         vbo
//     }
// }

fn main() {
    let mut window = GLWindow::new_configured(1024, 1024);
    let shader = ShaderCreator::new(math_gl_core::shaders::ColorMode::ColorPerPosition, false);
    let mut program = BasicShader::new(shader.program_id);

    //get vertices ready
    let mymeshes =
        utility::Mesh::load_from_model("/home/hussein/Documents/3D Models/SphereCylinder.obj");

    let sphere = &mymeshes[0];
    let cylinder = &mymeshes[1];

    let mut all_triangles = sphere.vertices.clone();
    all_triangles.extend(&cylinder.vertices);

    //getting program + camera ready
    program
        .buffer
        // .allocateBuffer(all_triangles.as_ptr() as _, 1000)
        .allocate_buffer_from_slice(&all_triangles)
        .setAttributes(
            &[
                GLAttrib::new(0, 3, 8, 0),
                GLAttrib::new(2, 3, 8, 3),
                GLAttrib::new(3, 2, 8, 6),
            ]
            .to_vec(),
        );

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );

    // generate instantiation

    let mut sphere_inst = utility::simple_lattice::generate_simple_lattice(1.00, 50);
    let atoms = sphere_inst.0.len();
    let bonds = sphere_inst.1.len();

    sphere_inst.0.extend(&sphere_inst.1);
    ArrayBuffer::slice_to_buffer(
        Some(program.buffer.VAO.0),
        gl::SHADER_STORAGE_BUFFER,
        sphere_inst.0.as_slice(),
        Some(0),
    );

    //bing a program to a camera
    cam.add_program(program.get_program());
    cam.set_camera(&Vec3::new(5.0, 5.0, 5.0), &Vec3::zeros(), &Vec3::z());
    cam.update_uniforms();

    window.m_glfw.set_swap_interval(glfw::SwapInterval::Sync(2));
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            update_camera(&event, &mut cam);
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program.get_program());
            program.buffer.bindAB();
            // gl::DrawArrays(gl::TRIANGLES, 0, sphere.vertices.len() as _);

            gl::Uniform1ui(2, 0);

            gl::DrawArraysInstanced(gl::TRIANGLES, 0, sphere.vertices.len() as _, atoms as _);
            gl::Uniform1ui(2, atoms as _);
            gl::DrawArraysInstanced(
                gl::TRIANGLES,
                sphere.vertices.len() as _,
                cylinder.vertices.len() as _,
                bonds as _,
            );
        }

        window.m_window.swap_buffers();
    }
}

// fn main2() {
//     let mymeshes =
//         utility::Mesh::load_from_model("/home/hussein/Documents/3D Models/SphereCylinder.obj");
//     let sphere = &mymeshes[0];
//     let cylinder = &mymeshes[1];

//     let mut window = GLWindow::new_configured(1024, 1024);
//     unsafe {
//         gl::Enable(gl::DEPTH_TEST);
//         gl::Enable(gl::BLEND);
//         gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
//         gl::Disable(gl::CULL_FACE_MODE);
//     }

//     let shader_program =
//         ShaderCreator::new(math_gl_core::shaders::ColorMode::ColorPerPosition, false);
//     let mut main_shader = BasicShader::new(shader_program.program_id);

//     main_shader.add_data(
//         0 as _, //sphere.vertices.as_ptr() as _,
//         ((sphere.vertices.len() + cylinder.vertices.len())
//             * size_of_val(&sphere.vertices[0]) as usize) as _,
//         &[
//             GLAttrib::new(0, 3, 8, 0),
//             GLAttrib::new(2, 3, 8, 3),
//             GLAttrib::new(3, 2, 8, 6),
//         ]
//         .to_vec(),
//     );

//     main_shader.buffer.updateBuffer(
//         sphere.vertices.as_ptr() as _,
//         0,
//         (sphere.vertices.len() * size_of_val(&sphere.vertices[0])) as _,
//     );
//     main_shader.buffer.updateBuffer(
//         cylinder.vertices.as_ptr() as _,
//         (sphere.vertices.len() * size_of_val(&sphere.vertices[0])) as _,
//         (cylinder.vertices.len() * size_of_val(&cylinder.vertices[0])) as _,
//     );
//     // main_shader.buffer.updateBuffer(
//     //     cylinder.vertices.as_ptr() as _,
//     //     (sphere.vertices.len() * size_of::<Mat4>()) as u32,
//     //     (cylinder.vertices.len() * size_of::<Mat4>()) as _,
//     // );
//     //generating instancing matrices
//     // nalgebra_glm::rotate(&Mat4::identity().append_translation(Vec4::new(1.0, 0.0, 0.0,0.0)), i*std::f32::consts::FRAC_PI_2, &Vec3::z()) ;
//     // let mut sphere_instances: Vec<Mat4> = (0..4)
//     //     .map(|i| {
//     //         // nalgebra_glm::rotation(i&, v)
//     //         nalgebra_glm::rotate(&nalgebra_glm::Mat4::identity(), 1.5 * i as f32, &Vec3::z())
//     //             * &Mat4::identity().append_translation(&Vec3::new(2.0 as f32, 0.0, 0.0))
//     //         // nalgebra_glm::rotate(
//     //         //     &Mat4::identity().append_translation(&Vec3::new(1.0, 0.0, 0.0)),
//     //         //     i as f32 * std::f32::consts::FRAC_PI_2,
//     //         //     &Vec3::z(),
//     //         // )
//     //     })
//     //     .collect();
//     // let sphere_instances = [
//     //     // utility::Bonds::generate_atom_matrix(Vec3::new(1.0, 1.0, 1.0), 0.3),
//     //     // utility::Bonds::generate_atom_matrix(Vec3::new(1.0, 1.0, 2.0), 0.3),
//     //     // utility::Bonds::generate_atom_matrix(Vec3::new(1.0, 1.0, 3.0), 0.3),
//     //     // utility::Bonds::generate_atom_matrix(Vec3::new(1.0, 1.0, 4.0), 0.3),
//     //     utility::Bonds::generate_bond_matrix(Vec3::zeros(), Vec3::new(1.0, 1.0, 1.0), 0.1),
//     //     utility::Bonds::generate_bond_matrix(Vec3::zeros(), Vec3::new(1.0, 1.0, 2.0), 0.1),
//     //     utility::Bonds::generate_bond_matrix(Vec3::zeros(), Vec3::new(1.0, 1.0, 3.0), 0.1),
//     //     utility::Bonds::generate_bond_matrix(Vec3::zeros(), Vec3::new(1.0, 1.0, 4.0), 0.1),
//     // ]
//     // .to_vec();

//     let (sphere_instances, cylinder_instances) =
//         utility::simple_lattice::generate_simple_lattice(2.0, 25);

//     // println!("{:?}", sphere_instances);

//     let mut atoms_buffer = slice_to_arraybuffer(
//         Some(main_shader.buffer.VAO.0),
//         sphere_instances.as_slice(),
//         Some(0),
//     );

//     let mut bonds_buffer = slice_to_arraybuffer(
//         Some(main_shader.buffer.VAO.0),
//         cylinder_instances.as_slice(),
//         Some(1),
//     );

//     // unsafe {
//     //     gl::GenBuffers(1, &mut atoms_buffer);
//     //     gl::BindVertexArray(main_shader.buffer.VAO.0);
//     //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, atoms_buffer);
//     //     gl::BufferData(
//     //         gl::SHADER_STORAGE_BUFFER,
//     //         (std::mem::size_of::<Mat4>() * sphere_instances.len()) as _,
//     //         sphere_instances.as_ptr() as _,
//     //         gl::STATIC_DRAW,
//     //     );
//     //     gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, atoms_buffer);
//     //     gl::GenBuffers(1, &mut bonds_buffer);
//     //     gl::BindVertexArray(main_shader.buffer.VAO.0);
//     //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, bonds_buffer);
//     //     gl::BufferData(
//     //         gl::SHADER_STORAGE_BUFFER,
//     //         (std::mem::size_of::<Mat4>() * cylinder_instances.len()) as _,
//     //         cylinder_instances.as_ptr() as _,
//     //         gl::STATIC_DRAW,
//     //     );
//     //     gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, bonds_buffer);
//     // }

//     let mut cam = Camera::new(
//         CameraProjection::default_perspective(1.0, 1.5),
//         CameraView::default(),
//     );

//     cam.add_program(main_shader.get_program());
//     cam.set_camera(&Vec3::new(15.0, 15.0, 15.0), &Vec3::zeros(), &Vec3::z());
//     cam.update_uniforms();

//     let mut counter = 0;

//     while !window.m_window.should_close() {
//         window.m_glfw.poll_events();

//         let mut is_clicked = true;

//         for (_, event) in glfw::flush_messages(&window.m_receiver) {
//             update_camera(&event, &mut cam);
//         }

//         unsafe {
//             gl::ClearColor(0.0, 0.0, 0.0, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
//             gl::UseProgram(main_shader.program);
//             gl::BindVertexArray(main_shader.buffer.VAO.0 as _);
//             //gl::DrawArrays(gl::TRIANGLES, 0, (mymesh[0].vertices.len()) as i32); //counter % (mesh.num_vertices) as i32);

//             // gl::Uniform1i(gl::GetUniformLocation(main_shader.program, "shape\0".as_ptr() as _), 0);
//             gl::Uniform1ui(2, 0);
//             gl::DrawArraysInstanced(
//                 gl::TRIANGLES,
//                 0,
//                 (sphere.vertices.len()) as i32,
//                 sphere_instances.len() as _,
//             );

//             // // gl::Uniform1i(gl::GetUniformLocation(main_shader.program, "shape\0".as_ptr() as _), 1);
//             // gl::Uniform1i(2, 1);
//             // gl::DrawArraysInstanced(
//             //     gl::TRIANGLES,
//             //     sphere.vertices.len() as _,
//             //     (cylinder.vertices.len()) as i32,
//             //     cylinder_instances.len() as _,
//             // );
//         }
//         counter += 1;
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

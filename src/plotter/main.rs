use std::mem::{size_of, size_of_val};

use gl;
use glfw::{Action, Context, WindowEvent};
use math_gl_core::{
    basic_components::{MathGrid, OrthogonalCoordinates},
    functions::ParametricLine,
    glbuffers::{ArrayBuffer, GLAttrib},
    scene::GLWindow,
    shaders::{BasicShader, Camera, CameraProjection, CameraView, ShaderCreator, ShaderProgram},
};
// mod utility;

use assimp::{self, import, Importer};
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

use math_gl_core;

struct Graphs2D {
    grid: MathGrid,
    lines: ParametricLine,
}
impl Graphs2D {
    fn new<Func2D>(function: Func2D, t0: f32, t1: f32, steps: u32) -> Self
    where
        Func2D: Fn(f32) -> Vec2,
    {
        let mut grid =
            MathGrid::generate_default_grid(Vec3::zeros(), Vec3::z(), Vec3::y(), (10, 10), None);

        let link_func = |t: f32| -> Vec4 {
            let tmp = function(t);
            Vec4::new(tmp.x, tmp.y, -0.1, 1.0)
        };

        let mut functions = ParametricLine::new_line(
            link_func,
            t0,
            t1,
            steps,
            Some(|t| Vec4::new(1.0, 1.0, 1.0, 1.0)),
        );
        Self {
            grid: grid,
            lines: functions,
        }
    }
    fn bind_camera(&self, camera: &mut Camera) -> &Self {
        camera
            .add_program(self.grid.get_program())
            .add_program(self.lines.get_program());
        self
    }
    fn draw_graph(&mut self) {
        self.grid.clean_draw();
        self.lines.draw_on(0, 1.0);
    }
}

fn main() {
    let mut window = GLWindow::new_configured(1024, 1024);
    let shader = ShaderCreator::new(math_gl_core::shaders::ColorMode::ColorPerPosition, false);
    let mut program = BasicShader::new(shader.program_id);

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );

    let mut text = math_gl_core::texts_utility::Textures::new();
    text.add_text(
        "Half a circle",
        0.20,
        200,
        Some(Vec3::new(0.0, 1.0, 0.11)),
        Some(Vec3::z()),
    )
    .update_array_buffer()
    .add_camera(&mut cam);
    // text/* .add_latex_function(
    //     "$\\displaystyle\nf(a)=\\sum_{n=0}^{\\infty}{\\frac{f^{(n)}(a)}{n!} }$",
    //     0.4,
    //     None,
    //     None,
    // ) */
    // .add_text(
    //     "$\\displaystyl",
    //     0.5,
    //     200,
    //     Some(Vec3::new(0.10, 0.0, 0.2)),
    //     None,
    // )
    // .update_array_buffer()
    // .add_camera(&mut cam);

    let mut my_graph = Graphs2D::new(
        |t| Vec2::new(f32::cos(t), f32::sin(t)),
        0.0,
        std::f32::consts::PI / 2.0,
        1000,
    );

    my_graph.bind_camera(&mut cam);
    // cam.set_ortho(-2.0, 2.0, -2.0, 2.0, 0.001, 100.0)
    //     .set_camera(&Vec3::new(0.0, 0.0, 2.0), &Vec3::zeros(), &Vec3::y())
    cam.update_uniforms();

    // let mut grid =
    //     MathGrid::generate_default_grid(Vec3::zeros(), Vec3::z(), Vec3::y(), (11, 11), None);
    // let mut line = ParametricLine::new_line(
    //     |t| Vec4::new(f32::cos(5.0 * t), f32::sin(5.0 * t), t / 10.0, 1.0),
    //     0.0,
    //     10.0,
    //     1000,
    //     Some(|t| Vec4::new(0.5 + f32::sin(t), 0.5 + f32::cos(t), 0.5, 1.0)),
    // );
    // cam.add_program(grid.get_program())
    //     .add_program(line.get_program());
    // cam.update_uniforms();

    window.m_glfw.set_swap_interval(glfw::SwapInterval::Sync(2));
    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            update_camera(&event, &mut cam);
        }

        // grid.draw();
        // line.draw_on(0, 3.0);
        my_graph.draw_graph();
        text.draw_textures();
        window.m_window.swap_buffers();
    }
}

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

use glfw::{ffi::glfwGetTime, Context, Window};
use math_gl_core::{
    basic_components::MathGrid, glbuffers::ArrayBuffer, la_projection::ProjectionCompute,
    shaders::BasicShader,
};
use math_gl_core::{
    basic_components::OrthogonalCoordinates,
    functions::ParametricLine,
    glbuffers::GLAttrib,
    graphics_raw::{self, file2string, Shaders::ComputeShader},
    la_projection::ProjectionPlane,
    scene::GLWindow,
    shaders::{self, Camera, CameraProjection, CameraView, ShaderCreator, ShaderProgram},
};
use nalgebra_glm::{Vec3, Vec4};

enum MyOption<T>
where
    T: Clone + Default,
{
    None,
    Some(T),
}
impl<T> MyOption<T>
where
    T: Clone + Default,
{
    pub fn unwrap(&self) -> T {
        if let MyOption::Some(val) = self {
            return val.clone();
        } else {
            return T::default();
        }
    }
    pub fn is_none(&self) -> bool {
        matches!(self, MyOption::None)
    }
}

static mut projection_compute: MyOption<ProjectionCompute> = MyOption::None;
pub struct ProjectionBox {
    // shader: BasicShader,
    compute_shader: ProjectionCompute,

    x_grid: MathGrid,
    y_grid: MathGrid,
    z_grid: MathGrid,

    x_projection: ArrayBuffer,
    y_projection: ArrayBuffer,
    z_projection: ArrayBuffer,
}

impl ProjectionBox {
    pub fn new(
        box_x: u32,
        box_y: u32,
        box_z: u32,
        x_shift: Option<f32>,
        y_shift: Option<f32>,
        z_shift: Option<f32>,
    ) -> Self {
        let mut x_grid = MathGrid::generate_default_grid(
            Vec3::new(box_x as f32 + x_shift.unwrap_or(0.0), 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::z(),
            (box_y, box_z),
            None,
        );
        let mut y_grid = MathGrid::generate_default_grid(
            Vec3::new(0.0, box_y as f32 + y_shift.unwrap_or(0.0), 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::z(),
            (box_x, box_z),
            Some(x_grid.get_program()),
        );
        let mut z_grid = MathGrid::generate_default_grid(
            Vec3::new(0.0, 0.0, -(box_z as f32 + z_shift.unwrap_or(0.0))),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::y(),
            (box_x, box_y),
            Some(x_grid.get_program()),
        );

        unsafe {
            if projection_compute.is_none() {
                projection_compute = MyOption::Some(ProjectionCompute::new(None));
            }
        }

        Self {
            compute_shader: unsafe { projection_compute.unwrap() },
            x_grid: x_grid,
            y_grid: y_grid,
            z_grid: z_grid,
            x_projection: ArrayBuffer::new(),
            y_projection: ArrayBuffer::new(),
            z_projection: ArrayBuffer::new(),
        }
    }
    pub fn generate_projection(&mut self, src: &ArrayBuffer) {
        self.x_projection = self.compute_shader.generate_projection_for_array_buffer(
            &src,
            &ProjectionPlane::new_from_coordinates(&self.x_grid.coordinates).projection_matrix,
        );
        self.y_projection = self.compute_shader.generate_projection_for_array_buffer(
            &src,
            &ProjectionPlane::new_from_coordinates(&self.y_grid.coordinates).projection_matrix,
        );
        self.z_projection = self.compute_shader.generate_projection_for_array_buffer(
            &src,
            &ProjectionPlane::new_from_coordinates(&self.z_grid.coordinates).projection_matrix,
        );
    }

    pub fn update_projection(&mut self, src: &ArrayBuffer) {
        self.compute_shader.update_projection_for_array_buffer(
            &src,
            &mut self.x_projection,
            &ProjectionPlane::new_from_coordinates(&self.x_grid.coordinates).projection_matrix,
        );
        self.compute_shader.update_projection_for_array_buffer(
            &src,
            &mut self.y_projection,
            &ProjectionPlane::new_from_coordinates(&self.y_grid.coordinates).projection_matrix,
        );
        self.compute_shader.update_projection_for_array_buffer(
            &src,
            &mut self.z_projection,
            &ProjectionPlane::new_from_coordinates(&self.z_grid.coordinates).projection_matrix,
        );
    }
    /// ### Requirements
    /// * Requires unbind mechanism
    pub fn bind_camera(&self, cam: &mut Camera) {
        cam.add_program(self.x_grid.get_program());
        cam.add_program(self.y_grid.get_program());
        cam.add_program(self.z_grid.get_program());
    }

    pub fn clean_draw_grid(&self) {
        unsafe {
            gl::ClearColor(0.00, 0.00, 0.00, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            // gl::UseProgram(self.x_grid.get_program());
            for grid in [&self.x_grid, &self.y_grid, &self.z_grid] {
                grid.draw_on();
            }
        }
    }
    pub fn draw_grid_on(&self) {
        unsafe {
            // gl::UseProgram(self.x_grid.get_program());
            for grid in [&self.x_grid, &self.y_grid, &self.z_grid] {
                grid.draw_on();
            }
        }
    }
    pub fn draw_projection_on(&self, shader: &BasicShader, line_width: f32) {
        unsafe {
            for proj in [&self.x_projection, &self.y_projection, &self.z_projection] {
                gl::UseProgram(shader.program);
                gl::LineWidth(line_width);
                proj.bindAB();
                gl::DrawArrays(
                    shader.draw_info.unwrap().0,
                    shader.draw_info.unwrap().1,
                    shader.draw_info.unwrap().2,
                );
            }
        }
    }
}

use std::mem::size_of;

use nalgebra_glm::{Vec3, Vec4};

use crate::{
    glbuffers::GLAttrib,
    shaders::{BasicShader, ShaderCreator, ShaderProgram},
};

/// ### Description
/// * A structure for orthogonal coordinates, it also can map vectors using specific coordinates
#[derive(Clone, Copy)]
pub struct OrthogonalCoordinates {
    pub origin: Vec3,
    pub u_base: Vec3,
    pub v_base: Vec3,
    pub w_base: Vec3,
}

impl OrthogonalCoordinates {
    /// ### Description
    /// * create new coordinates, by giving it two axes the third is calculated using cross product
    pub fn new(origin: Vec3, U: Vec3, V: Vec3) -> Self {
        Self {
            origin: origin,
            u_base: U,
            v_base: V,
            w_base: nalgebra_glm::cross(&U, &V),
        }
    }

    pub fn new_from_normal(origin: Vec3, normal: Vec3, up: Vec3) -> Self {
        let mut right = nalgebra_glm::cross(&up, &normal);
        Self {
            origin: origin,
            u_base: right,
            v_base: up,
            w_base: normal,
        }
    }
    /// ### Description
    /// * Calculate the vector for given bases `vec`
    pub fn map(&self, vec: &Vec3) -> Vec3 {
        self.origin + vec.x * self.u_base + vec.y * self.v_base + vec.z * self.w_base
    }

    /// ### Description
    /// * Calculate the vector for given bases `vec`
    pub fn map_4f(&self, vec: &Vec4) -> Vec4 {
        let tmp = vec.x * self.u_base + vec.y * self.v_base + vec.z * self.w_base;
        Vec4::new(
            self.origin.x + tmp.x,
            self.origin.y + tmp.y,
            self.origin.z + tmp.z,
            1.0,
        )
    }
}
impl Default for OrthogonalCoordinates {
    fn default() -> Self {
        Self {
            origin: Vec3::zeros(),
            u_base: Vec3::x(),
            v_base: Vec3::y(),
            w_base: Vec3::z(),
        }
    }
}

/// ### Description
/// * Define 2D grids for give orthogonal coordinates and colors
pub struct MathGrid {
    pub coordinates: OrthogonalCoordinates,
    pub vertices: Vec<(Vec4, Vec4)>,
    pub shader: BasicShader,
    pub primary_color: Vec4,
    pub secondary_color: Vec4,
    line_offset_count_width: [(i32, i32, f32); 3],
}

impl MathGrid {
    pub fn generate_coordinates(
        coords: &OrthogonalCoordinates,
        divisions: (i32, i32),
        subdivisions: (i32, i32),
        range: (f32, f32),
        axes_color: Vec4,
        primary_color: Option<Vec4>,
        secondary_color: Option<Vec4>,
    ) -> (Vec<(Vec4, Vec4)>, [(i32, i32, f32); 3]) {
        let divx = (divisions.0) * subdivisions.0;
        let divy = (divisions.1) * subdivisions.1;
        let x_spacing = range.0 / (divx) as f32;
        let y_spacing = range.1 / (divy) as f32;

        //let translation = Vec4::new(coords.origin.x, coords.origin.y, coords.origin.z, 0.0);

        // let mut vertices: Vec<(Vec3, Vec4)> = Vec::new();

        let mut l1: Vec<(Vec4, Vec4)> = Vec::new();
        let mut l2: Vec<(Vec4, Vec4)> = Vec::new();
        let mut l3: Vec<(Vec4, Vec4)> = Vec::new();

        let mut counter = 0;
        for x_line in -1 * (divx)..=(divx) {
            let v1 = Vec4::new(x_line as f32 * x_spacing, range.1, 0.0, 1.0);
            let v2 = Vec4::new(x_line as f32 * x_spacing, -range.1, 0.0, 1.0);

            if x_line == 0 {
                l1.push((coords.map_4f(&v1), axes_color));
                l1.push((coords.map_4f(&v2), axes_color));
                counter += 1;
                continue;
            }

            let mut line_color = Vec4::zeros();

            if counter % subdivisions.0 == 0 {
                line_color = primary_color.unwrap_or(Vec4::new(1.0, 1.0, 1.0, 1.0));
                l2.push((coords.map_4f(&v1), line_color));
                l2.push((coords.map_4f(&v2), line_color));
                //current_set = &mut l2;
            } else {
                line_color = secondary_color.unwrap_or(Vec4::new(0.3, 0.3, 0.3, 1.0));
                l3.push((coords.map_4f(&v1), line_color));
                l3.push((coords.map_4f(&v2), line_color));
                // current_set = &mut l3;
            }

            counter += 1;
        }

        counter = 0;
        //vertical lines
        for y_line in -1 * (divy)..=(divy) {
            let v1 = Vec4::new(range.0, y_line as f32 * y_spacing, 0.0, 0.0);
            let v2 = Vec4::new(-range.0, y_line as f32 * y_spacing, 0.0, 0.0);
            if y_line == 0 {
                l1.push((coords.map_4f(&v1), axes_color));
                l1.push((coords.map_4f(&v2), axes_color));
                counter += 1;
                continue;
            }

            let mut line_color = Vec4::zeros();
            if counter % subdivisions.0 == 0 {
                line_color = primary_color.unwrap_or(Vec4::new(1.0, 1.0, 1.0, 1.0));
                l2.push((coords.map_4f(&v1), line_color));
                l2.push((coords.map_4f(&v2), line_color));
                //current_set = &mut l2;
            } else {
                line_color = secondary_color.unwrap_or(Vec4::new(0.3, 0.3, 0.3, 1.0));
                l3.push((coords.map_4f(&v1), line_color));
                l3.push((coords.map_4f(&v2), line_color));
                // current_set = &mut l3;
            }

            // current_set.push((coords.map(&v1), line_color));
            // current_set.push((coords.map(&v2), line_color));
            counter += 1;
        }
        // current_set = &mut Vec::new();

        let mut vertices: Vec<(Vec4, Vec4)> = Vec::with_capacity(l1.len() + l2.len() + l3.len());
        // let l2_size = l2.len() as i32;

        let lines_info = [
            (0, l1.len() as i32, 2.0f32),
            (l1.len() as i32, l2.len() as i32, 1.5f32),
            ((l1.len() + l2.len()) as i32, l3.len() as i32, 0.75),
        ];

        vertices.extend(l1);
        vertices.extend(l2);
        vertices.extend(l3);
        (vertices, lines_info)
    }

    /// ### Description
    /// * Create full symmetric grid
    /// #### Parameters
    /// * `coords` : Coordinates of the grid
    /// * `divisions` : Number of divisions per half (e.g. `(10,10)` means 10 steps in the `+X` and 10 steps in the `-X`)
    /// * `subdivisions`: Number of subdivisions per step
    /// * `axes_color` : The color of `X=0,Y=0` lines,
    /// * `primary_color` : The color of steps lines,
    /// * `secondary_color` : The color of sub-steps lines,
    /// #### Example
    /// ```
    /// let mut grid = MathGrid::new_full(
    ///     OrthogonalCoordinates::default(),
    ///     (5, 5),
    ///     (3, 3),
    ///     (5.0, 5.0),
    ///     Vec4::new(1.0, 1.0, 1.0, 0.20),
    ///     Some(Vec4::new(0.2, 0.8, 1.0, 0.70)),
    ///     Some(Vec4::new(1.0, 1.0, 1.0, 0.20)),
    ///     None
    /// );
    /// ```

    pub fn new_full(
        coords: OrthogonalCoordinates,
        divisions: (i32, i32),
        subdivisions: (i32, i32),
        range: (f32, f32),
        axes_color: Vec4,
        primary_color: Option<Vec4>,
        secondary_color: Option<Vec4>,
        program: Option<u32>,
    ) -> Self {
        let grid_program = if let Some(prog_id) = program {
            prog_id
        } else {
            ShaderCreator::new(crate::shaders::ColorMode::ColorPerVertex, false).program_id
        };

        let mut grid_shader = BasicShader::new(grid_program);

        let (vertices, lines_info) = Self::generate_coordinates(
            &coords,
            divisions,
            subdivisions,
            range,
            axes_color,
            primary_color,
            secondary_color,
        );

        grid_shader
            .add_data(
                vertices.as_ptr() as _,
                (std::mem::size_of_val(&vertices[0]) * vertices.len()) as _,
                &[GLAttrib::new(0, 4, 8, 0), GLAttrib::new(1, 4, 8, 4)],
            )
            .add_draw_info((gl::LINE, 0, vertices.len() as _));

        Self {
            coordinates: coords,
            vertices: vertices,
            shader: grid_shader,
            primary_color: primary_color.unwrap_or(Vec4::new(1.0, 1.0, 1.0, 1.0)),
            secondary_color: secondary_color.unwrap_or(Vec4::new(0.7, 0.7, 0.7, 1.0)),
            //line_type_offset: (0, 4, 4 + 2 * (2 * divisions.0 + 2 * divisions.1)),
            line_offset_count_width: lines_info,
        }
    }

    pub fn generate_default_grid(
        origin: Vec3,
        normal: Vec3,
        up: Vec3,
        grid: (u32, u32),
        program: Option<u32>,
    ) -> Self {
        MathGrid::new_full(
            OrthogonalCoordinates::new_from_normal(origin, normal, up), //OrthogonalCoordinates::new(Vec3::new(0.0, 0.0, -2.0), Vec3::x(), Vec3::y()),
            (grid.0 as _, grid.1 as _),
            (4, 4),
            (grid.0 as f32, grid.1 as f32),
            Vec4::new(1.0, 1.0, 1.0, 0.80),
            Some(Vec4::new(0.2, 0.8, 1.0, 0.50)),
            Some(Vec4::new(1.0, 1.0, 1.0, 0.40)),
            program,
        )
    }

    pub fn set_widths(&mut self, w1: f32, w2: f32, w3: f32) {
        self.line_offset_count_width[0].2 = w1;
        self.line_offset_count_width[1].2 = w2;
        self.line_offset_count_width[2].2 = w3;
    }

    pub fn get_program(&self) -> u32 {
        self.shader.get_program()
    }
    /// ### Description
    /// * Clear screen then draw grid
    pub fn clean_draw(&self) {
        unsafe {
            gl::ClearColor(0.01, 0.01, 0.01, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.draw_on()
        }
    }
    /// ### Description
    /// * Clear screen then draw grid
    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader.program);
            self.shader.clean_draw(None, None, None, Some(|| {}));
        }
    }
    pub fn draw_on(&self) {
        unsafe {
            gl::UseProgram(self.get_program());
            self.shader.buffer.bindAB();
            for i in self.line_offset_count_width {
                gl::LineWidth(i.2);
                gl::DrawArrays(gl::LINES, i.0, i.1);
            }
        }
    }
}

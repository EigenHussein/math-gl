use nalgebra_glm::{Vec3, Vec4};

use crate::{
    glbuffers::{ArrayBuffer, GLAttrib},
    shaders::{BasicShader, ShaderCreator, ShaderProgram},
};

pub struct ParametricLine {
    // m_program_id: u32,
    // m_buffer: ArrayBuffer,
    // function: Box<dyn Fn(f32) -> Vec3>,
    pub shader: BasicShader,
    // pub nVertices: u32,
    pub points: Vec<(Vec4, Vec4)>,
    lines_locations: Vec<(usize, usize)>,
}

impl ParametricLine {
    pub fn new_line<PFunc, CFunc>(
        function: PFunc,
        t0: f32,
        t1: f32,
        steps: u32,
        color_function: Option<CFunc>,
    ) -> Self
    where
        PFunc: Fn(f32) -> Vec4,
        CFunc: Fn(f32) -> Vec4,
    {
        let mut points: Vec<(Vec4, Vec4)> = Vec::with_capacity(steps as _);
        for i in 0..steps {
            let t = t0 + (t1 - t0) * i as f32 / (steps as f32 - 1.0);
            if let Some(color_fn) = &color_function {
                points.push((function(t), color_fn(t)))
            } else {
                points.push((function(t), Vec4::new(1.0, 1.0, 1.0, 1.0)))
            }
            // points.push((function(t), Vec4::new(1.0, 1.0, 1.0, 1.0)));
            // println!("{:?}",(t,tmp));
        }

        let function_progam = ShaderCreator::new(crate::shaders::ColorMode::ColorPerVertex, false);
        let mut function_shader = BasicShader::new(function_progam.program_id);
        function_shader
            .add_data(
                points.as_ptr() as _,
                (points.len() * std::mem::size_of_val(&points[0])) as _,
                &[GLAttrib::new(0, 4, 8, 0), GLAttrib::new(1, 4, 8, 4)],
            )
            .add_draw_info((gl::LINE_STRIP, 0, steps as _));

        ParametricLine {
            shader: function_shader,
            points: points, // nVertices: points.len(), // points: points,
            lines_locations: vec![(0usize, steps as _)],
        }
    }

    pub fn update_function<T>(&mut self, function_index: u32, t0: f32, t1: f32, mut new_function: T)
    where
        T: FnMut(f32) -> Vec4,
    {
        let func_info = self.lines_locations[function_index as usize];
        for i in 0..func_info.1 as usize {
            let t = t0 + (t1 - t0) * i as f32 / (func_info.1 as f32 - 1.0);
            self.points[func_info.0 + i].0 = new_function(t);
        }
        self.shader.buffer.updateBuffer(
            self.points.as_ptr() as _,
            (func_info.0 * std::mem::size_of_val(&self.points[0])) as u32,
            std::mem::size_of_val(&self.points[0]) as u32 * func_info.1 as u32,
        );
    }
    pub fn append_function<PFunc, CFunc>(
        &mut self,
        function: PFunc,
        t0: f32,
        t1: f32,
        steps: u32,
        color_function: Option<CFunc>,
    ) where
        PFunc: Fn(f32) -> Vec4,
        CFunc: Fn(f32) -> Vec4,
    {
        self.lines_locations.push((self.points.len(),steps as usize));
        self.points.resize_with(
            self.points.len() + steps as usize,
            Default::default,
        );
        for i in 0..steps {
            let t = t0 + (t1 - t0) * i as f32 / (steps as f32 - 1.0);
            if let Some(color_fn) = &color_function {
                self.points.push((function(t), color_fn(t)))
            } else {
                self.points
                    .push((function(t), Vec4::new(1.0, 1.0, 1.0, 1.0)))
            }
            // points.push((function(t), Vec4::new(1.0, 1.0, 1.0, 1.0)));
            // println!("{:?}",(t,tmp));
        }
        
    }

    pub fn update_function_and_color<T, CFunc>(
        &mut self,
        function_index: u32,
        t0: f32,
        t1: f32,
        mut new_function: T,
        mut new_color: CFunc,
    ) where
        T: FnMut(f32) -> Vec4,
        CFunc: FnMut(f32) -> Vec4,
    {
        let func_info = self.lines_locations[function_index as usize];
        for i in 0..func_info.1 as usize {
            let t = t0 + (t1 - t0) * i as f32 / (func_info.1 as f32 - 1.0);
            self.points[func_info.0 + i].0 = new_function(t);
            self.points[func_info.0 + i].1 = new_color(t);
        }
        self.shader.buffer.updateBuffer(
            self.points.as_ptr() as _,
            (func_info.0 * std::mem::size_of_val(&self.points[0])) as u32,
            std::mem::size_of_val(&self.points[0]) as u32 * func_info.1 as u32,
        );
    }

    pub fn draw_on(&mut self, function_index: i32, width: f32) {
        unsafe {
            gl::UseProgram(self.shader.program);
            gl::LineWidth(width);
            self.shader.buffer.bindAB();
            if function_index < 0 {
                for line in &self.lines_locations {
                    gl::DrawArrays(gl::LINE_STRIP, line.0 as _, line.1 as _);
                    // gl::DrawArrays(
                    //     self.shader.draw_info.unwrap().0,
                    //     self.shader.draw_info.unwrap().1,
                    //     self.shader.draw_info.unwrap().2,
                    // );
                }
            } else {
                gl::DrawArrays(
                    gl::LINE_STRIP,
                    self.lines_locations[function_index as usize].0 as _,
                    self.lines_locations[function_index as usize].1 as _,
                );
            }
        }
    }

    pub fn draw_ab(&self, array_buffer: &ArrayBuffer, width: f32) {
        unsafe {
            gl::UseProgram(self.shader.program);
            gl::LineWidth(width);
            array_buffer.bindAB();
            // gl::DrawArrays(
            //     self.shader.draw_info.unwrap().0,
            //     self.shader.draw_info.unwrap().1,
            //     self.shader.draw_info.unwrap().2,
            // );
            for line in &self.lines_locations {
                gl::DrawArrays(gl::LINE_STRIP, line.0 as _, line.1 as _);
                // gl::DrawArrays(
                //     self.shader.draw_info.unwrap().0,
                //     self.shader.draw_info.unwrap().1,
                //     self.shader.draw_info.unwrap().2,
                // );
            }
        }
    }
    pub fn get_program(&self) -> u32 {
        self.shader.get_program()
    }
}
impl Drop for ParametricLine {
    fn drop(&mut self) {
        // unsafe {
        //     gl::DeleteShader(self.shader.program);
        // }
    }
}

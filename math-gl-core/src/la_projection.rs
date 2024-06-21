use glm::{Matrix2x3, Matrix3x2, Vector3};
use nalgebra_glm::{Mat3, Mat3x2, Mat4, Vec3};

use crate::{
    basic_components::OrthogonalCoordinates,
    glbuffers::ArrayBuffer,
    graphics_raw::{file2string, Shaders::ComputeShader},
};

pub struct ProjectionPlane {
    pub mNormal: Vec3,
    // pub mOrigin: Vec3,
    pub projection_matrix: nalgebra_glm::Mat4,
}

impl ProjectionPlane {
    pub fn new_from_coordinates(coords: &OrthogonalCoordinates) -> Self {
        let mut up = coords.v_base;
        let mut right = nalgebra_glm::normalize(&up.cross(&coords.w_base));
        up = nalgebra_glm::normalize(&coords.w_base.cross(&right));

        let A = Mat3x2::new(right.x, up.x, right.y, up.y, right.z, up.z);

        //projection matrix
        let p3d = A * (A.transpose() * A).try_inverse().unwrap() * A.transpose();
        
        //translation vector
        let distance = nalgebra_glm::dot(&coords.map(&Vec3::new(1.0, 1.0, 0.0)), &coords.w_base);

        // println!("Distance: {}",distance);
        
        let tr = coords.w_base.normalize() * distance;

        let mut projection4d_matrix = Mat4::new(
            p3d[0], p3d[1], p3d[2], tr[0], p3d[3], p3d[4], p3d[5], tr[1], p3d[6], p3d[7], p3d[8],
            tr[2], 0.0, 0.0, 0.0, 1.0,
        );

        ProjectionPlane {
            mNormal: coords.w_base,
            projection_matrix: projection4d_matrix,
        }
    }

    pub fn new(normal: Vec3, distance: f32) -> Self {
        // let mut up = Vector3::new(0.0f32, 1.0, 0.0);
        // nalgebra_glm::Mat3::
        let mut up = Vec3::new(0.0f32, 1.0, 0.0);
        let mut right = nalgebra_glm::normalize(&up.cross(&normal));
        up = nalgebra_glm::normalize(&normal.cross(&right));

        let A = Mat3x2::new(right.x, up.x, right.y, up.y, right.z, up.z);
        //projection matrix
        let p3d = A * (A.transpose() * A).try_inverse().unwrap() * A.transpose();
        //translation vector
        let tr = normal.normalize() * distance;

        let mut projection4d_matrix = Mat4::new(
            p3d[0], p3d[1], p3d[2], tr[0], p3d[3], p3d[4], p3d[5], tr[1], p3d[6], p3d[7], p3d[8],
            tr[2], 0.0, 0.0, 0.0, 1.0,
        );

        ProjectionPlane {
            mNormal: normal,
            // mOrigin: origin,
            projection_matrix: projection4d_matrix,
        }
    }
}

/// ## Description
/// * Compute shader to project a vertex (vert: vec4,color: vec4) into a specfic plane define by normals
#[derive(Default, Clone)]
pub struct ProjectionCompute {
    compute_shader_id: u32,
    projection_uniform: i32,
}

impl ProjectionCompute {
    pub fn new(compute_shader_filename: Option<&str>) -> Self {
        let compute_shader_id = ComputeShader(&file2string(
            compute_shader_filename
                .unwrap_or("/home/hussein/Dev/Rust/math-gl/shaders/projection_compute.glsl"),
        ));
        let projection_uniform =
            unsafe { gl::GetUniformLocation(compute_shader_id, "projection\0".as_ptr() as _) };

        Self {
            compute_shader_id: compute_shader_id,
            projection_uniform: projection_uniform,
        }
    }

    pub fn generate_projection_for_array_buffer(
        &self,
        src_buffer: &ArrayBuffer,
        projection_matrix: &Mat4,
    ) -> ArrayBuffer {
        let mut dest_buffer = ArrayBuffer::new();
        dest_buffer
            .allocateBuffer(0 as _, src_buffer.VBO.1)
            .setAttributes(&src_buffer.VAO.1);

        let number_of_vertices = src_buffer.VBO.1 / (8 * 4);
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, src_buffer.VBO.0);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, dest_buffer.VBO.0);
            gl::UseProgram(self.compute_shader_id);
            gl::UniformMatrix4fv(
                self.projection_uniform,
                1,
                0,
                projection_matrix.as_ptr() as _,
            );
            gl::DispatchCompute(number_of_vertices, 1, 1);
            gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
        }

        dest_buffer
    }
    pub fn update_projection_for_array_buffer(
        &self,
        src_buffer: &ArrayBuffer,
        dest_buffer: &ArrayBuffer,
        projection_matrix: &Mat4,
    ) {
        let number_of_vertices = src_buffer.VBO.1 / (8 * 4);
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, src_buffer.VBO.0);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, dest_buffer.VBO.0);
            gl::UseProgram(self.compute_shader_id);
            gl::UniformMatrix4fv(
                self.projection_uniform,
                1,
                0,
                projection_matrix.as_ptr() as _,
            );
            gl::DispatchCompute(number_of_vertices, 1, 1);
            gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
        }
    }
}

impl Drop for ProjectionCompute {
    fn drop(&mut self) {
        println!("Dropping ProjectionCompute");
        unsafe {
            gl::DeleteShader(self.compute_shader_id);
        }
    }
}

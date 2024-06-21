use std::time::Duration;

use super::{
    glbuffers::GLAttrib,
    shaders::{
        self, BasicShader, Camera, CameraProjection, CameraView, ShaderCreator, ShaderProgram,
    },
};
use nalgebra_glm::{Mat4, Vec2, Vec4};

use crate::texture_utility;

pub struct Texture {
    texture_id: u32,
    width: i32,
    height: i32,
}
impl Texture {
    pub fn texture_set_default_parameters() {
        unsafe {
            // gl::TexParameteri(
            //     gl::TEXTURE_2D,
            //     gl::TEXTURE_MIN_FILTER,
            //     gl::LINEAR_MIPMAP_LINEAR as _,
            // ); // Trilinear filtering
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
        }
    }
    pub fn new(
        width: i32,
        height: i32,
        internal: Option<u32>,
        external_format: Option<u32>,
        data_type: Option<u32>,
        data_ptr: Option<*const u8>,
    ) -> Self {
        let mut texture = 0u32;
        unsafe {
            gl::GenTextures(1, &mut texture as _);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal.unwrap_or(gl::RGBA16F) as _,
                width,
                height,
                0,
                external_format.unwrap_or(gl::RGBA) as _,
                data_type.unwrap_or(gl::UNSIGNED_BYTE),
                data_ptr.unwrap_or(0 as _) as _,
            );
            Self::texture_set_default_parameters();
            // gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self {
            texture_id: texture,
            width: width,
            height: height,
        }
    }
    pub fn new_rgba(width: i32, height: i32, data: *const u8) -> Self {
        Self::new(
            width,
            height,
            Some(gl::RGBA8UI),
            Some(gl::RGBA),
            None,
            Some(data),
        )
    }
    pub fn new_mono(width: i32, height: i32, data: *const u8) -> Self {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }
        Self::new(
            width,
            height,
            Some(gl::R32F),
            Some(gl::RED),
            None,
            Some(data),
        )
    }
    pub fn get_all(&self) -> (u32, i32, i32) {
        return (self.texture_id, self.width, self.height);
    }
    pub fn set_active_texture(&self, texture_number: u32) -> &Self {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_number);
        }
        self
    }
    pub fn bind_texture(&self) -> &Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
        self
    }
    pub fn unbind_texture(&self) -> &Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        println!("Deleting texture!");
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}

pub struct FrameBuffer {
    pub fbo: u32,
    pub rbo_depth: u32,
    pub rbo_color: u32,
    pub fbo_texture: u32,
    pub destination_texture: u32,
    pub width: i32,
    pub height: i32,
}

impl FrameBuffer {
    pub fn texture_set_default_parameters() {
        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as _,
            ); // Trilinear filtering
               //    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
        }
    }

    pub fn texture_generate(
        width: i32,
        height: i32,
        internal: Option<u32>,
        external_format: Option<u32>,
    ) -> u32 {
        let mut texture = 0u32;
        unsafe {
            gl::GenTextures(1, &mut texture as _);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal.unwrap_or(gl::RGBA16F) as _,
                width,
                height,
                0,
                external_format.unwrap_or(gl::RGBA) as _,
                gl::FLOAT,
                0 as _,
            );
            // // Texture::texture_set_default_parameters();
            // // gl::TexParameteri(
            // //     gl::TEXTURE_2D,
            // //     gl::TEXTURE_MIN_FILTER,
            // //     gl::LINEAR_MIPMAP_LINEAR as _,
            // // );
            // gl::TexParameteri(
            //     gl::TEXTURE_2D,
            //     gl::TEXTURE_MIN_FILTER,
            //     gl::LINEAR_MIPMAP_LINEAR as _,
            // ); // Trilinear filtering
            //    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            Self::texture_set_default_parameters();

            // Trilinear filtering
            // gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        texture
    }

    pub fn framebuffer_generate() -> u32 {
        unsafe {
            let mut fbo = 0u32;
            gl::GenFramebuffers(1, &mut fbo as _);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            return fbo;
        }
    }

    pub fn renderbuffer_generate(width: i32, height: i32, multisample: bool) -> (u32, u32) {
        let mut depthBuffer = 0u32;
        let mut colorBuffer = 0u32;
        unsafe {
            gl::GenRenderbuffers(1, &mut depthBuffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depthBuffer);
            gl::RenderbufferStorageMultisample(
                gl::RENDERBUFFER,
                16,
                gl::DEPTH24_STENCIL8,
                width,
                height,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                depthBuffer,
            );

            // Create multisampled color attachment

            gl::GenRenderbuffers(1, &mut colorBuffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, colorBuffer);
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 16, gl::RGBA8, width, height);
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::RENDERBUFFER,
                colorBuffer,
            );
        }
        (depthBuffer, colorBuffer)
    }

    pub fn new(width: i32, height: i32) -> Self {
        let mut fbo = 0u32;
        let (mut rbo_depth, mut rbo_color) = (0u32, 0u32);
        let mut texture = 0u32;
        unsafe {
            fbo = Self::framebuffer_generate();
            println!("Generated FBO");

            // Generate and bind RBO with multisampling
            (rbo_depth, rbo_color) = Self::renderbuffer_generate(width, height, true);

            // gl::GenTextures(1, &mut texture);
            println!("Generated Render Buffer");

            texture = Self::texture_generate(width, height, None, None);
            Self::texture_set_default_parameters();

            // gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture, 0);

            // Check framebuffer completeness
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer is not complete");
            } else {
                println!("FrameBuffer: Success");
            }

            // Unbind framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Self {
            fbo: fbo,
            rbo_depth: rbo_depth,
            rbo_color: rbo_color,
            destination_texture: 0,
            fbo_texture: texture,
            height: height,
            width: width,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

pub struct VirtualScreen {
    pub shader: BasicShader,
    camera: Camera,
    pub frame_buffer: FrameBuffer,
    pub simple_fbo: u32,
    pub simple_tx: u32,
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

        let virtualframe = texture_utility::FrameBuffer::new(width, height);

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
                gl::RGBA16F as _,
                width,
                height,
                0,
                gl::RGBA,
                gl::FLOAT,
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
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as _);
        }
    }
}

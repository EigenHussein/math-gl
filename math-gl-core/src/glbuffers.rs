use std::os::raw::c_void;

use gl::NONE;

extern crate gl;
pub struct GLAttrib {
    // valid: bool,
    attribute_index: u32,
    size: gl::types::GLint,
    type_: gl::types::GLenum,
    normalized: gl::types::GLboolean,
    stride: gl::types::GLsizei,
    pointer: gl::types::GLuint,
}

impl GLAttrib {
    pub fn set(&self) {
        unsafe {
            gl::VertexAttribPointer(
                self.attribute_index,
                self.size,
                self.type_,
                self.normalized,
                self.stride,
                self.pointer as _,
            );
            gl::EnableVertexAttribArray(self.attribute_index);
        }
    }
}
impl Default for GLAttrib {
    fn default() -> Self {
        GLAttrib {
            attribute_index: 0,
            size: 0,
            type_: gl::FLOAT,
            normalized: 0,
            stride: 0,
            pointer: 0,
        }
    }
}
impl Clone for GLAttrib {
    fn clone(&self) -> Self {
        GLAttrib {
            attribute_index: self.attribute_index,
            size: self.size,
            type_: self.type_,
            normalized: self.normalized,
            stride: self.stride,
            pointer: self.pointer,
        }
    }
}
impl GLAttrib {
    pub fn new(idx: u32, size: i32, stride: i32, offset: u32) -> Self {
        GLAttrib {
            attribute_index: idx,
            size: size,
            type_: gl::FLOAT,
            normalized: 0,
            stride: stride * (std::mem::size_of::<f32>() as i32),
            pointer: offset * (std::mem::size_of::<f32>() as u32),
        }
    }
}
enum ABInitType {
    INTIALIZED_VBO = 1,
    INTIALIZED_EBO = 2,
    INTIALIZED_SSBO = 4,
}
enum ABState {
    NONE_INTIALIZED,
    INTIALIZED(u8),
    ALLOCATED,
}
pub struct ArrayBuffer {
    State: ABState,
    pub VAO: (u32, Vec<GLAttrib>),
    pub VBO: (u32, u32),  //VBO, VBO size (in bytes)
    pub EBO: (u32, u32),  //EBO, EBO size (in bytes)
    pub SSBO: (u32, u32), //...,
}
impl Default for ArrayBuffer {
    fn default() -> Self {
        ArrayBuffer {
            State: ABState::NONE_INTIALIZED,
            VAO: (0, Vec::new()),
            VBO: (0, 0),
            EBO: (0, 0),
            SSBO: (0, 0),
        }
    }
}

impl ArrayBuffer {
    pub fn slice_to_buffer<T: Sized>(
        vao: Option<u32>,
        target: u32,
        data: &[T],
        binding: Option<u32>,
    ) -> u32 {
        let mut vbo = 0u32;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            if let Some(valid_vao) = vao {
                gl::BindVertexArray(valid_vao);
            }

            gl::BindBuffer(target, vbo);
            gl::BufferData(
                target,
                (std::mem::size_of::<T>() * data.len()) as _,
                data.as_ptr() as _,
                gl::STATIC_DRAW,
            );
            if let Some(binding_point) = binding {
                gl::BindBufferBase(target, binding_point, vbo);
            }
            vbo
        }
    }
    pub fn new() -> Self {
        let mut VAO: u32 = 0;
        let mut VBO: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
        }
        ArrayBuffer {
            State: (ABState::INTIALIZED(ABInitType::INTIALIZED_VBO as u8)),
            VAO: (VAO, Vec::new()),
            VBO: (VBO, 0),
            EBO: (0, 0),
            SSBO: (0, 0),
        }
    }
    pub fn allocateBuffer(&mut self, buf: *const c_void, buf_size: u32) -> &mut ArrayBuffer {
        unsafe {
            match self.State {
                ABState::INTIALIZED(prev) => {
                    if (prev & (ABInitType::INTIALIZED_VBO as u8)) > 0 {
                        gl::BindVertexArray(self.VAO.0);
                        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO.0);
                        gl::BufferData(gl::ARRAY_BUFFER, buf_size as _, buf, gl::DYNAMIC_DRAW);
                        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                        gl::BindVertexArray(0);
                        self.VBO.1 = buf_size;
                        self.State = ABState::INTIALIZED(prev | (ABInitType::INTIALIZED_VBO as u8));
                    } else {
                        println!("allocateBuffer(): The buffer is already allocated!");
                    }
                }
                _ => (println!("ELSAWY")),
            }
        }
        self
    }
    pub fn allocate_buffer_from_slice<T: Sized>(&mut self, data: &[T]) -> &mut Self {
        unsafe {
            match self.State {
                ABState::INTIALIZED(prev) => {
                    if (prev & (ABInitType::INTIALIZED_VBO as u8)) > 0 {
                        gl::BindVertexArray(self.VAO.0);
                        gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO.0);
                        gl::BufferData(
                            gl::ARRAY_BUFFER,
                            (std::mem::size_of::<T>() * data.len()) as _,
                            data.as_ptr() as _,
                            gl::DYNAMIC_DRAW,
                        );
                        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                        gl::BindVertexArray(0);
                        self.VBO.1 = (std::mem::size_of::<T>() * data.len()) as _;
                        self.State = ABState::INTIALIZED(prev | (ABInitType::INTIALIZED_VBO as u8));
                    } else {
                        println!("allocateBuffer(): The buffer is already allocated!");
                    }
                }
                _ => (println!("ELSAWY")),
            }
        }
        return self;
    }

    pub fn updateBuffer(&self, buf: *const c_void, offset: u32, buf_size: u32) {
        unsafe { gl::NamedBufferSubData(self.VBO.0, offset as _, buf_size as _, buf) };
    }

    pub fn allocateElements(&mut self, buf: *const c_void, buf_size: u32) -> &mut ArrayBuffer {
        unsafe {
            match self.State {
                ABState::INTIALIZED(prev) => {
                    if (prev & (ABInitType::INTIALIZED_EBO as u8)) == 0 {
                        //if not intialized

                        gl::GenBuffers(1, &mut self.EBO.0);
                        gl::BindVertexArray(self.VAO.0);
                        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO.0);
                        gl::BufferData(
                            gl::ELEMENT_ARRAY_BUFFER,
                            buf_size as _,
                            buf,
                            gl::STATIC_DRAW,
                        );

                        self.EBO.1 = buf_size;
                        self.State = ABState::INTIALIZED(prev | (ABInitType::INTIALIZED_EBO as u8));
                        println!("alocated indices!");
                    } else {
                        println!("allocateElements(): The buffer is already allocated!");
                    }
                }
                _ => (),
            }
        }
        self
    }

    pub fn setAttributes(&mut self, attributes: &Vec<GLAttrib>) -> &mut ArrayBuffer {
        unsafe {
            gl::BindVertexArray(self.VAO.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO.0);
        }
        self.VAO.1 = attributes.clone();
        for i in &self.VAO.1 {
            i.set();
        }
        self
    }
    pub fn bindAB(&self) {
        unsafe {
            gl::BindVertexArray(self.VAO.0);
            // gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO.0);
            if let ABState::INTIALIZED(prev) = self.State {
                if prev & ABInitType::INTIALIZED_EBO as u8 > 0 {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.EBO.0);
                }
            }
        }
    }
}

impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        println!("Destructing ArrayBuffer...");
        match self.State {
            ABState::NONE_INTIALIZED => (),
            ABState::INTIALIZED(init_type) => unsafe {
                gl::DeleteVertexArrays(1, &self.VAO.0);
                if (init_type & (ABInitType::INTIALIZED_VBO as u8)) > 0 {
                    gl::DeleteBuffers(1, &self.VBO.0);
                }
                if (init_type & (ABInitType::INTIALIZED_EBO as u8)) > 0 {
                    gl::DeleteBuffers(1, &self.EBO.0);
                }
                if (init_type & (ABInitType::INTIALIZED_SSBO as u8)) > 0 {
                    gl::DeleteBuffers(1, &self.SSBO.0);
                }
            },
            _ => (),
        }
    }
}

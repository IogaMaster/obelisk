use std::mem;
use std::os::raw::*;
use std::ptr;

use gl::types::*;

/// # Vertex Array Object
///
/// ## Example
/// ```
/// let vao = Vao::new();
/// vao.bind();
/// ```
pub struct Vao {
    id: gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Vao {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

/// # Buffer Object
/// An object for storing data
///
/// ## Example
/// ```
/// let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
/// vbo.bind();
///
/// vbo.store_f32_data(&float32_array);
/// ```
pub struct BufferObject {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferObject {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferObject {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            );
        }
    }

    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            );
        }
    }
}

/// # Vertex Attribute
/// Discribes vertex data
///
/// ## Example
/// ```
/// let position_attribute = VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
/// position_attribute.enable()
/// ```
pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

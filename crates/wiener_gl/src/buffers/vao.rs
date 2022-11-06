use super::buffer::Buffer;

use gl;

/// Vertex array that specifies the vertex layout on GPU memory.
pub struct VertexArrays {
    _id: u32,
    _stride: u32,
    _size: u32,
    _layout: Vec<u32>,
}

impl VertexArrays {
    /// Generate a builder for a vertex array.
    pub fn builder() -> Self {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }

        return VertexArrays {
            _id: vao_id,
            _stride: 0,
            _size: 4,
            _layout: [].to_vec(),
        };
    }

    /// Set the size in bytes of each number.
    pub fn size(mut self, new_size: u32) -> Self {
        self._size = new_size;
        return self;
    }

    /// Specify the layout of the vertex array. This layout corresponds
    /// to a vector containing the size of each attribute.
    /// 
    /// For example, if your vertex has 3 spatial coordinates, 3 colors
    /// (RGB) and 2 UV coordinates, then the layout would be (3, 3, 2).
    pub fn layout(mut self, new_layout: &Vec<u32>) -> Self {
        self._layout = new_layout.to_vec();
        self._stride = new_layout.iter().sum();

        return self;
    }

    /// Build the vertex array, creating the attributes.
    pub fn build(self) -> Self {
        unsafe {
            for i in 0..self._layout.len() {
                let l = self._layout[i];
                gl::VertexAttribPointer(
                    i as u32,
                    l as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    self._stride as i32,
                    (self._size * self._layout[0..i].iter().sum::<u32>()) as *const _,
                );
                gl::EnableVertexArrayAttrib(self._id, i as u32);
            }
        }
        return self;
    }
}

impl Buffer for VertexArrays {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self._id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self._id);
        }
    }
}

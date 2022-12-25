use crate::Bindable;

use gl;
use gl::types::*;
use log;

/// Vertex array that specifies the vertex layout on GPU memory.
#[derive(Clone, Debug)]
pub struct VertexArray {
    /// Unique ID associated to the object.
    _id: u32,

    /// Number of elements in each vertex.
    pub stride: u32,

    /// Size in bytes of each element.
    pub size: u32,

    /// Type of contained data.
    pub data_type: GLenum,

    /// Layout in GPU memory of the vertex.
    pub layout: Vec<u32>,
}

impl VertexArray {
    /// Set the size in bytes of each number.
    pub fn size(mut self, new_size: u32) -> Self {
        self.size = new_size;
        return self;
    }

    /// Specify the layout of the vertex array. This layout corresponds
    /// to a vector containing the size of each attribute.
    ///
    /// For example, if your vertex has 3 spatial coordinates, 3 colors
    /// (RGB) and 2 UV coordinates, then the layout would be (3, 3, 2).
    pub fn layout(mut self, new_layout: &[u32]) -> Self {
        self.layout = new_layout.to_vec();
        self.stride = new_layout.iter().sum();
        self.update();
        return self;
    }

    /// Set the data type.
    pub fn data_type(mut self, new_type: GLenum) -> Self {
        self.data_type = new_type;
        return self;
    }

    /// Update the vertex array, creating the attributes.
    pub fn update(&self) {
        log::info!("VertexArray :: Updating layout");
        unsafe {
            for i in 0..self.layout.len() {
                let l = self.layout[i];
                gl::VertexAttribPointer(
                    i as u32,
                    l as i32,
                    self.data_type,
                    gl::FALSE,
                    (self.size * self.stride) as i32,
                    (self.size * self.layout[0..i].iter().sum::<u32>()) as *const _,
                );
                gl::EnableVertexArrayAttrib(self._id, i as u32);
            }
        }
    }
}

impl Bindable for VertexArray {
    fn bind(&self) {
        log::trace!("VertexArray :: Binding");
        unsafe {
            gl::BindVertexArray(self._id);
        }
    }

    fn unbind(&self) {
        log::trace!("VertexArray :: Unbinding");
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        log::info!("VertexArray :: Deleting");
        unsafe {
            gl::DeleteVertexArrays(1, &self._id);
        }
    }
}

impl Default for VertexArray {
    /// Generate a builder for a vertex array.
    fn default() -> Self {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }
        log::info!("VertexArray :: Creating new VertexArray {:?}", vao_id);

        return VertexArray {
            _id: vao_id,
            stride: 0,
            size: 4,
            data_type: gl::FLOAT,
            layout: [].to_vec(),
        };
    }
}

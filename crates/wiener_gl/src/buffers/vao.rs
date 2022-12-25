use crate::Bindable;

use gl;
use gl::types::*;
use log;

/// Vertex attribute.
#[derive(Copy, Clone, Debug)]
pub struct VertexAttribute {
    /// Location in memory.
    pub location: usize,

    /// Size of the attribute.
    pub size: u32,

    /// Data type of the attribute.
    pub data_type: GLenum,
}

impl VertexAttribute {
    pub fn new(location: usize, size: u32, data_type: GLenum) -> Self {
        return VertexAttribute { location, size, data_type };
    }

    /// Bind the vertex attribute to a given VAO.
    pub fn bind_vao(&self, vao: &VertexArray) {
        unsafe {
            gl::VertexAttribPointer(
                self.location as u32,
                self.size as i32,
                self.data_type,
                gl::FALSE,
                (vao.size * vao.stride) as i32,
                (vao.size * vao.layout[0..self.location].iter().map(|a| a.size).sum::<u32>()) as *const _,
            );
            gl::EnableVertexArrayAttrib(vao.get_id(), self.location as u32);
        }
    }
}

/// Vertex array that specifies the vertex layout on GPU memory.
#[derive(Clone, Debug)]
pub struct VertexArray {
    /// Unique ID associated to the object.
    _id: u32,

    /// Number of elements in each vertex.
    pub stride: u32,

    /// Size in bytes of each element.
    pub size: u32,

    /// Layout in GPU memory of the vertex.
    pub layout: Vec<VertexAttribute>,
}

impl VertexArray {
    /// Get the VAO's id.
    pub fn get_id(&self) -> u32 {
        return self._id;
    }

    /// Set the size in bytes of each number.
    pub fn size(mut self, new_size: u32) -> Self {
        self.size = new_size;
        return self;
    }

    /// Specify the layout of the vertex array. This layout corresponds
    /// to a vector of VertexAttribute structs.
    ///
    /// For example, if your vertex has 3 spatial coordinates, 3 colors
    /// (RGB) and 2 UV coordinates, then the layout would be (3, 3, 2).
    pub fn layout(mut self, new_layout: &Vec<VertexAttribute>) -> Self {
        self.set_layout(new_layout);
        return self;
    }

    /// Specify the layout of the vertex array. This layout corresponds
    /// to a vector of VertexAttribute structs.
    ///
    /// For example, if your vertex has 3 spatial coordinates, 3 colors
    /// (RGB) and 2 UV coordinates, then the layout would be (3, 3, 2).
    pub fn set_layout(&mut self, new_layout: &Vec<VertexAttribute>) {
        self.layout = new_layout.to_vec();
        self.stride = new_layout.iter().map(|a| a.size).sum();
        log::debug!("VertexArray :: Set layout to {:?}. New stride is {:?}", self.layout, self.stride);
        self.update();
    }

    /// Push a new attribute to the VAO.
    pub fn push_attribute(mut self, new_attribute: VertexAttribute) -> Self {
        self.push_attribute_inplace(new_attribute);
        return self;
    }
    
    /// Push a new attribute to the VAO without returning self.
    pub fn push_attribute_inplace(&mut self, new_attribute: VertexAttribute) {
        self.layout.push(new_attribute);
        self.stride += new_attribute.size;
        log::debug!("VertexArray :: Added {:?}. New layout is {:?} and stride is {:?}", new_attribute, self.layout, self.stride);
        new_attribute.bind_vao(&self);
    }

    /// Update the vertex array, creating the attributes.
    pub fn update(&self) {
        log::info!("VertexArray :: Updating layout");
        self.bind();
        for attr in &self.layout {
            attr.bind_vao(self);
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
            layout: Vec::new(),
        };
    }
}

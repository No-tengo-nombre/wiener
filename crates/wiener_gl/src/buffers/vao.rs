use super::buffer::Buffer;

use gl;

#[derive(Clone, Debug)]
/// Vertex array that specifies the vertex layout on GPU memory.
pub struct VertexArray {
    /// Unique ID associated to the object.
    _id: u32,

    /// Number of elements in each vertex.
    _stride: u32,

    /// Size in bytes of each element.
    _size: u32,

    /// Layout in GPU memory of the vertex.
    _layout: Vec<u32>,
}

impl VertexArray {
    /// Create a new vertex array with size 4 (32 bits).
    pub fn new(layout: &[u32]) -> Self {
        return VertexArray::new_sized(4, layout);
    }

    /// Create a new vertex array with a given size.
    pub fn new_sized(size: u32, layout: &[u32]) -> Self {
        return VertexArray::builder().size(size).layout(layout).build();
    }

    /// Generate a builder for a vertex array.
    pub fn builder() -> Self {
        let mut vao_id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
        }

        return VertexArray {
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
    pub fn layout(mut self, new_layout: &[u32]) -> Self {
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

impl Buffer for VertexArray {
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

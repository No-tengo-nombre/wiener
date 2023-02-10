/// Object that represents a buffer to the GPU.
pub trait Buffer {
    /// Buffer data to this space in GPU memory.
    fn buffer_data<T>(&self, data: &[T]);
}

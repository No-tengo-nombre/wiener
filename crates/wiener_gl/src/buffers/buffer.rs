/// Object that represents a buffer to the GPU.
pub trait Buffer {
    /// Bind the buffer in GPU memory.
    fn bind(&self);

    /// Unbind the buffer in GPU memory.
    fn unbind(&self);

    /// Delete the buffer.
    fn delete(&self);

    /// Buffer data to this space in GPU memory.
    fn buffer_data<T>(&self, data: &[T]) -> Self;
}

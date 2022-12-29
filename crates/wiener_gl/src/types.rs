pub trait Bindable {
    /// Bind the current object.
    fn bind(&self);
    /// Unbind the current object.
    fn unbind(&self);
    /// Delete the current object from memory.
    fn delete(&self);
}

pub trait HasID {
    /// Get the OpenGL ID of the object.
    fn get_id(&self) -> u32;
}

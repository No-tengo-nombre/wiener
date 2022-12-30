/// Trait for any object that can be bound, unbound and removed from some
/// space in GPU memory.
pub trait Bindable {
    /// Bind the current object.
    fn bind(&self);
    /// Unbind the current object.
    fn unbind(&self);
    /// Delete the current object from memory.
    fn delete(&self);
}

/// Trait for objects that have a unique GPU id.
pub trait HasID {
    /// Get the OpenGL ID of the object.
    fn get_id(&self) -> u32;
}

/// Trait of objects that can be drawn onto a frame buffer.
pub trait Drawable {
    /// Draw this object.
    fn draw(&self);
}

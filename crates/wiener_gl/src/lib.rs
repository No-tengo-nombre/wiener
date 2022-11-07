//! Create that contains the code to connect Wiener with OpenGL

pub mod prelude;

mod buffers;
mod gl_manager;
mod window;

pub use buffers::*;
pub use gl_manager::*;
pub use window::*;

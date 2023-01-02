# Wiener changelog

---

## 0.1.1
- **(wiener_gl)** Added option to create meshes from OBJ files.
- **(wiener_gl)** Added an example for loading models from OBJ files.
- **(wiener_gl)** Changed the way meshes are created from files.
    - Made handlers for different file formats.
    - Made a function that creates a `Mesh` from a handler.
    - Made another function that interprets the filename and associates the file extension to a handler.

## 0.1.0
- Wiener was created!
- **(wiener_gl)** Implemented vertex arrays, vertex buffers, element buffers and uniform buffers (the latter are untested).
- **(wiener_gl)** Introduced a `Mesh` struct, which abstracts away many of the core concepts from graphics programming.
- **(wiener_gl)** Made `Mesh` be constructable from an OFF file.
- **(wiener_gl)** Implemented 2D textures.
- **(wiener_gl)** Implemented FrameBuffers and RenderBuffers.
- **(wiener_gl)** Implemented shaders and shader programs, allowing them to be easily buildable from files.
- **(wiener_utils)** Made a crate for mathematical functions that are relevent to the program.

---

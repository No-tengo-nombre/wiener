use std::fs;
use std::ops::AddAssign;
use std::str::FromStr;
use std::{ffi::c_void, fmt::Debug};

use crate::{
    Bindable, Buffer, Drawable, ElementBuffer, ShaderProgram, Texture2D, VertexArray,
    VertexAttribute, VertexBuffer,
};

use gl::types::GLenum;
use log::{info, trace};
use num::traits::Pow;
use num::{Float, Integer, ToPrimitive};

use wiener_utils::math;

/// Structure for a simple mesh, corresponding to the most basic set of
/// triangles.
#[derive(Clone, Copy, Debug)]
pub struct Mesh<'a, U, I> {
    pub vao: VertexArray<'a>,
    pub vbo: VertexBuffer,
    pub ebo: ElementBuffer,
    pub primitive: GLenum,
    _primitive_num: i32,
    pub shader: &'a ShaderProgram<'a>,
    pub textures: &'a [Texture2D],
    pub model_mat: [[U; 4]; 4],
    pub view_mat: [[U; 4]; 4],
    pub projection_mat: [[U; 4]; 4],
    phantom: std::marker::PhantomData<I>,
}

impl<
        'a,
        U: Float + Debug + Copy + FromStr + Pow<u16, Output = U> + AddAssign<U>,
        I: Integer + std::str::FromStr + ToPrimitive + Copy + Debug,
    > Mesh<'a, U, I>
where
    <U as FromStr>::Err: Debug,
    <I as FromStr>::Err: Debug,
{
    /// Create a new mesh associated to a shader program.
    pub fn new(shader: &'a ShaderProgram<'a>) -> Self {
        info!("Mesh :: Creating mesh");
        let size = std::mem::size_of::<U>();
        info!("Mesh :: Setting associated VAO size to {:?}", size);
        let vao = VertexArray::default().size(size as u32);
        vao.bind();
        return Mesh {
            vao,
            vbo: VertexBuffer::new(),
            ebo: ElementBuffer::new(),
            primitive: gl::TRIANGLES,
            _primitive_num: 0,
            shader,
            textures: &[],
            model_mat: math::linalg::eye4::<U>(),
            view_mat: math::linalg::eye4::<U>(),
            projection_mat: math::linalg::eye4::<U>(),
            phantom: std::marker::PhantomData,
        };
    }

    /// Read a mesh from an OFF file, generating normals and a color. The resulting
    /// VAO layout is (3, 3), so the user must make sure to specify this.
    pub fn from_off(filename: &str, shader: &'a ShaderProgram<'a>) -> Self {
        log::info!("Mesh :: Reading mesh from OFF file");

        // Read the file and separate into lines
        let contents = fs::read_to_string(filename).expect("Error reading file.");
        let mut lines = contents.split("\r\n");

        // Verify correctness and get the format
        if lines.next() != Some("OFF") {
            panic!("File doesn't have the OFF format.");
        }
        let file_descriptor = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();
        let vert_num = i32::from_str_radix(file_descriptor[0], 10).unwrap();
        let face_num = i32::from_str_radix(file_descriptor[1], 10).unwrap();
        let mut vertices = Vec::<[U; 6]>::with_capacity(vert_num as usize);
        let mut faces = Vec::<[I; 3]>::with_capacity(face_num as usize);
        log::debug!("Mesh :: Reading {vert_num} vertices and {face_num} faces");

        // Read the vertices
        let mut temp_vert;
        let mut x;
        let mut y;
        let mut z;
        log::trace!("Mesh :: Reading the vertices");
        for _ in 0..vert_num {
            temp_vert = lines
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            x = temp_vert[0].parse::<U>().unwrap();
            y = temp_vert[1].parse::<U>().unwrap();
            z = temp_vert[2].parse::<U>().unwrap();
            vertices.push([
                x,
                y,
                z,
                U::zero(),
                U::zero(),
                U::zero(),
            ]);
        }

        // Read the faces and generate the normals
        let mut temp_face;
        let mut v0;
        let mut v1;
        let mut v2;
        let mut vec1;
        let mut vec2;
        let mut normal_result;
        let mut normalized_v0;
        let mut normalized_v1;
        let mut normalized_v2;
        let mut vertex0_positions;
        let mut vertex1_positions;
        let mut vertex2_positions;
        let mut vertex0_normals;
        let mut vertex1_normals;
        let mut vertex2_normals;
        log::trace!("Mesh :: Reading the faces");
        for _ in 0..face_num {
            // Read the face
            temp_face = lines
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>()[1..]
                .to_owned();
            v0 = temp_face[0].parse::<I>().unwrap();
            v1 = temp_face[1].parse::<I>().unwrap();
            v2 = temp_face[2].parse::<I>().unwrap();
            faces.push([v0, v1, v2]);

            vertex0_positions = [
                vertices[v0.to_usize().unwrap()][0],
                vertices[v0.to_usize().unwrap()][1],
                vertices[v0.to_usize().unwrap()][2],
            ];
            vertex1_positions = [
                vertices[v1.to_usize().unwrap()][0],
                vertices[v1.to_usize().unwrap()][1],
                vertices[v1.to_usize().unwrap()][2],
            ];
            vertex2_positions = [
                vertices[v2.to_usize().unwrap()][0],
                vertices[v2.to_usize().unwrap()][1],
                vertices[v2.to_usize().unwrap()][2],
            ];
            vertex0_normals = [
                vertices[v0.to_usize().unwrap()][3],
                vertices[v0.to_usize().unwrap()][4],
                vertices[v0.to_usize().unwrap()][5],
            ];
            vertex1_normals = [
                vertices[v1.to_usize().unwrap()][3],
                vertices[v1.to_usize().unwrap()][4],
                vertices[v1.to_usize().unwrap()][5],
            ];
            vertex2_normals = [
                vertices[v2.to_usize().unwrap()][3],
                vertices[v2.to_usize().unwrap()][4],
                vertices[v2.to_usize().unwrap()][5],
            ];

            // Generate the normals
            vec1 = math::subtract3(vertex1_positions, vertex0_positions);
            vec2 = math::subtract3(vertex2_positions, vertex1_positions);
            normal_result = math::cross(vec1, vec2);

            normalized_v0 = math::normalize3(math::add3(vertex0_normals, normal_result));
            normalized_v1 = math::normalize3(math::add3(vertex1_normals, normal_result));
            normalized_v2 = math::normalize3(math::add3(vertex2_normals, normal_result));

            vertices[v0.to_usize().unwrap()][3] = normalized_v0[0];
            vertices[v0.to_usize().unwrap()][4] = normalized_v0[1];
            vertices[v0.to_usize().unwrap()][5] = normalized_v0[2];

            vertices[v1.to_usize().unwrap()][3] = normalized_v1[0];
            vertices[v1.to_usize().unwrap()][4] = normalized_v1[1];
            vertices[v1.to_usize().unwrap()][5] = normalized_v1[2];

            vertices[v2.to_usize().unwrap()][3] = normalized_v2[0];
            vertices[v2.to_usize().unwrap()][4] = normalized_v2[1];
            vertices[v2.to_usize().unwrap()][5] = normalized_v2[2];
        }

        // Once we have all the info, we create the mesh
        let vert_slice = vertices.as_slice();
        let face_slice = faces.as_slice();
        log::debug!(
            "Mesh :: Found {:?} vertices and {:?} faces",
            std::mem::size_of_val(vert_slice) / std::mem::size_of::<U>() / 6,
            std::mem::size_of_val(face_slice) / std::mem::size_of::<I>() / 3,
        );
        return Mesh::<U, I>::new(shader)
            .vertices(vert_slice)
            .indices(face_slice);
    }

    /// Buffer vertices to the associated VBO, returning `self`.
    pub fn vertices<T>(mut self, new_vertices: &[T]) -> Self {
        self.set_vertices(new_vertices);
        return self;
    }

    /// Buffer indices to the associated EBO, returning `self`.
    pub fn indices<T>(mut self, new_indices: &[T]) -> Self {
        self.set_indices(new_indices);
        return self;
    }

    /// Set the associated shader program, returning `self`.
    pub fn shader(mut self, new_shader: &'a ShaderProgram<'a>) -> Self {
        trace!("Mesh :: Setting shader");
        self.shader = new_shader;
        return self;
    }

    /// Set the associated textures, returning `self`.
    pub fn textures(mut self, new_textures: &'a [Texture2D]) -> Self {
        trace!("Mesh :: Setting textures");
        self.textures = new_textures;
        return self;
    }

    /// Set the usage of the mesh, returning `self`.
    pub fn usage(mut self, new_usage: GLenum) -> Self {
        trace!("Mesh :: Setting usage");
        self.vbo.usage = new_usage;
        return self;
    }

    /// Set the layout of the data, returning `self`.
    pub fn layout(mut self, new_layout: &'a [VertexAttribute]) -> Self {
        trace!("Mesh :: Setting layout");
        self.vao.set_layout(new_layout);
        return self;
    }

    /// Set the primitive to use for drawing, returning `self`.
    pub fn primitive(mut self, new_primitive: GLenum) -> Self {
        trace!("Mesh :: Setting primitive type");
        self.primitive = new_primitive;
        return self;
    }

    /// Buffer vertices to the associated VBO inplace, without returning anything.
    pub fn set_vertices<T>(&mut self, new_vertices: &[T]) {
        trace!("Mesh :: Setting vertices");
        self.vbo.buffer_data(new_vertices);
    }

    /// Buffer indices to the associated EBO inplace, without returning anything.
    pub fn set_indices<T>(&mut self, new_indices: &[T]) {
        trace!("Mesh :: Setting indices");
        self.ebo.buffer_data(new_indices);
        self._primitive_num =
            (new_indices.len() * std::mem::size_of::<T>() / std::mem::size_of::<I>()) as i32;
        info!(
            "Mesh :: Setting EBO number of primitives to {:?}",
            self._primitive_num
        );
    }

    /// Set the usage of the mesh inplace, without returning anything.
    pub fn set_usage(&mut self, new_usage: GLenum) {
        trace!("Mesh :: Setting usage");
        self.vbo.usage = new_usage;
        self.ebo.usage = new_usage;
    }

    /// Set the layout of the data inplace, without returning anything.
    pub fn set_layout(&mut self, new_layout: &'a [VertexAttribute]) {
        trace!("Mesh :: Setting layout");
        self.vao.set_layout(new_layout);
    }

    /// Set the model matrix, returning `self`.
    pub fn model_mat(mut self, new_model_mat: [[U; 4]; 4]) -> Self {
        trace!("Mesh :: Setting model matrix");
        self.model_mat = new_model_mat;
        return self;
    }

    /// Set the view matrix, returning `self`.
    pub fn view_mat(mut self, new_view_mat: [[U; 4]; 4]) -> Self {
        trace!("Mesh :: Setting view matrix");
        self.view_mat = new_view_mat;
        return self;
    }

    /// Set the projection matrix, returning `self`.
    pub fn projection_mat(mut self, new_projection_mat: [[U; 4]; 4]) -> Self {
        trace!("Mesh :: Setting projection matrix");
        self.projection_mat = new_projection_mat;
        return self;
    }
}

impl<'a, U, I> Bindable for Mesh<'a, U, I> {
    fn bind(&self) {
        trace!("Mesh :: Binding");
        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
        self.shader.bind();
        for t in self.textures {
            t.bind();
        }
    }

    fn unbind(&self) {
        trace!("Mesh :: Unbinding");
        self.vao.unbind();
        self.vbo.unbind();
        self.ebo.unbind();
        self.shader.unbind();
        for t in self.textures {
            t.unbind();
        }
    }

    fn delete(&self) {
        trace!("Mesh :: Deleting");
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
        self.shader.delete();
        for t in self.textures {
            t.delete();
        }
    }
}

impl<'a, U: Debug + Copy, I> Drawable for Mesh<'a, U, I> {
    fn draw(&self) {
        trace!(
            "Mesh :: Sending draw call, model {:?}, view {:?}, projection {:?}",
            self.model_mat,
            self.view_mat,
            self.projection_mat
        );
        self.bind();

        // Uniform the MVP matrices
        self.shader.uniform_mat4f("u_model", self.model_mat);
        self.shader.uniform_mat4f("u_view", self.view_mat);
        self.shader
            .uniform_mat4f("u_projection", self.projection_mat);

        unsafe {
            gl::DrawElements(
                self.primitive,
                self._primitive_num,
                gl::UNSIGNED_INT,
                0 as *const c_void,
            );
        }
    }
}

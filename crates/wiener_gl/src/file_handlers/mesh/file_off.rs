use crate::MeshFileHandler;
use wiener_utils::math;

use num::{traits::Pow, Float, ToPrimitive};
use std::ops::AddAssign;
use std::str::FromStr;
use std::{fmt::Debug, fs};

#[derive(Clone, Debug)]
pub struct MeshHandlerOFF {
    filename: String,
}

impl MeshHandlerOFF {
    pub fn new(filename: &str) -> Self {
        return MeshHandlerOFF {
            filename: filename.to_string(),
        };
    }
}

impl MeshFileHandler for MeshHandlerOFF {
    fn load_file<
        U: Float + From<f32> + FromStr + AddAssign<U> + Pow<u16, Output = U>,
        I: FromStr + ToPrimitive + Copy,
    >(
        &self,
    ) -> (Vec<U>, Vec<I>, u32)
    where
        <U as FromStr>::Err: Debug,
        <I as FromStr>::Err: Debug,
    {
        log::info!("MeshHandlerOFF :: Reading mesh from OFF file");

        // Read the file and separate into lines
        let contents = fs::read_to_string(&self.filename).expect("Error reading file.");
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
        let mut faces = Vec::<I>::with_capacity(3 * face_num as usize);
        log::debug!("MeshHandlerOFF :: Reading {vert_num} vertices and {face_num} faces");

        // Read the vertices
        let mut temp_vert;
        let mut x;
        let mut y;
        let mut z;
        log::trace!("MeshHandlerOFF :: Reading the vertices");
        for _ in 0..vert_num {
            temp_vert = lines
                .next()
                .unwrap()
                .split_whitespace()
                .collect::<Vec<&str>>();
            x = temp_vert[0].parse::<U>().unwrap();
            y = temp_vert[1].parse::<U>().unwrap();
            z = temp_vert[2].parse::<U>().unwrap();
            vertices.push([x, y, z, U::zero(), U::zero(), U::zero()]);
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
        log::trace!("MeshHandlerOFF :: Reading the faces");
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
            faces.extend([v0, v1, v2]);

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
        let mut vertices_result = Vec::<U>::with_capacity(6 * vert_num as usize);
        for v in vertices {
            vertices_result.extend(v);
        }

        return (vertices_result, faces, 6);
    }

    fn get_name<'a>() -> &'a str {
        return "OFF";
    }
}

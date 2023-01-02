use crate::MeshFileHandler;

use obj::{load_obj, Obj};
use std::io::BufReader;
use std::str::FromStr;
use std::{fmt::Debug, fs};
use num::{Float, ToPrimitive, traits::Pow};
use std::ops::AddAssign;

#[derive(Clone, Debug)]
pub struct MeshHandlerOBJ {
    filename: String,
}

impl MeshHandlerOBJ {
    pub fn new(filename: &str) -> Self {
        return MeshHandlerOBJ {
            filename: filename.to_string(),
        };
    }
}

impl MeshFileHandler for MeshHandlerOBJ {
    fn load_file<
        U: Float + From<f32> + FromStr + AddAssign<U> + Pow<u16, Output = U>,
        I: FromStr + ToPrimitive + Copy,
    >(
        &self,
    ) -> (Vec<U>, Vec<I>, u32)
    where
        <U as FromStr>::Err: Debug,
        <I as FromStr>::Err: Debug
    {
        log::info!("MeshHandlerOBJ :: Reading mesh from OBJ file");
        let input_buffer =
        BufReader::new(fs::File::open(&self.filename).expect("File could not be opened"));
        let data: Obj = load_obj(input_buffer).expect("Obj file could not be read from");
        log::info!("MeshHandlerOBJ :: Reading {:?} vertices and {:?} faces", data.vertices.len(), data.indices.len());

        // Once we have all the info, we create the mesh
        let mut faces_vec = Vec::with_capacity(data.indices.len());
        for idx in data.indices {
            faces_vec.push(I::from_str(&idx.to_string()).unwrap());
        }

        let mut vertex_result = Vec::<U>::with_capacity(data.vertices.len() * 6);
        for v in data.vertices {
            vertex_result.push(v.position[0].try_into().unwrap());
            vertex_result.push(v.position[1].try_into().unwrap());
            vertex_result.push(v.position[2].try_into().unwrap());
            vertex_result.push(v.normal[0].try_into().unwrap());
            vertex_result.push(v.normal[1].try_into().unwrap());
            vertex_result.push(v.normal[2].try_into().unwrap());
        }
        return (vertex_result, faces_vec, 6);
    }

    fn get_name<'a>() -> &'a str {
        return "OBJ";
    }
}

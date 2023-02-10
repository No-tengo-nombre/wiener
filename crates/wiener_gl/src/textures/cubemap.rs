use std::ptr::null;
use gl::types::GLenum;

use crate::{Bindable, HasID, Texture};


#[derive(Clone, Copy, Debug)]
pub struct CubeMapTexture {
    /// Unique ID associated to the object.
    _id: u32,

    /// Slot to which the image is bound.
    pub tex_num: u32,

    /// Internal format of the data. Use floats for HDR.
    pub internal_format: GLenum,

    /// Format of the data.
    pub format: GLenum,

    /// Type of the data.
    pub data_type: GLenum,

    /// Method to use for S wrapping.
    pub wrap_s: GLenum,

    /// Method to use for T wrapping.
    pub wrap_t: GLenum,

    /// Method to use for R wrapping.
    pub wrap_r: GLenum,

    /// Method for min filter.
    pub min_filter: GLenum,

    /// Method for mag filter.
    pub mag_filter: GLenum,
}

impl Texture for CubeMapTexture {}

impl HasID for CubeMapTexture {
    fn get_id(&self) -> u32 {
        return self._id;
    }
}

impl CubeMapTexture {
    /// Change the slot of the texture.
    pub fn tex_num(mut self, new_bind: u32) -> Self {
        log::trace!("CubeMapTexture :: Setting texture num {:?}", new_bind);
        self.tex_num = new_bind;
        return self;
    }

    /// Change the internal format of the texture.
    pub fn internal_format(mut self, new_format: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting internal format {:?}", new_format);
        self.internal_format = new_format;
        return self;
    }

    /// Change the format of the texture.
    pub fn format(mut self, new_format: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting format {:?}", new_format);
        self.format = new_format;
        return self;
    }

    /// Change the data type of the texture.
    pub fn data_type(mut self, new_type: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting data type {:?}", new_type);
        self.data_type = new_type;
        return self;
    }

    /// Change the S wrapping method.
    pub fn wrap_s(mut self, new_wrap: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting wrap S {:?}", new_wrap);
        self.wrap_s = new_wrap;
        return self;
    }

    /// Change the T wrapping method.
    pub fn wrap_t(mut self, new_wrap: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting wrap T {:?}", new_wrap);
        self.wrap_t = new_wrap;
        return self;
    }

    /// Change the R wrapping method.
    pub fn wrap_r(mut self, new_wrap: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting wrap R {:?}", new_wrap);
        self.wrap_r = new_wrap;
        return self;
    }

    /// Change the min filtering method.
    pub fn min_filter(mut self, new_filter: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting min filter {:?}", new_filter);
        self.min_filter = new_filter;
        return self;
    }

    /// Change the max filtering method.
    pub fn mag_filter(mut self, new_filter: GLenum) -> Self {
        log::trace!("CubeMapTexture :: Setting mag filter {:?}", new_filter);
        self.mag_filter = new_filter;
        return self;
    }

    /// Build the texture. After building, you should buffer the desired
    /// image.
    pub fn build(self) -> Self {
        log::info!("CubeMapTexture :: Building CubeMapTexture with parameters:\nWrap S {:?}\nWrap T {:?}\nWrap R {:?}\nMin filter {:?}\nMag filter {:?}", self.wrap_s, self.wrap_t, self.wrap_r, self.min_filter, self.mag_filter);
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, self.wrap_r as i32);
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                self.min_filter as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                self.mag_filter as i32,
            );
        }
        self.unbind();
        return self;
    }

    /// Buffer the given image to the texture.
    pub fn buffer_img<T>(&self, data: [&[T]; 6], widths: [i32; 6], heights: [i32; 6]) {
        log::info!("CubeMapTexture :: Buffering images to CubeMapTexture");
        self.bind();
        unsafe {
            for i in 0..6_usize {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    self.internal_format as i32,
                    widths[i],
                    heights[i],
                    0,
                    self.format,
                    self.data_type,
                    data[i].as_ptr() as *const _,
                );
                gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
            }
        }
        self.unbind();
    }

    /// Allocate memory for the texture without buffering anything.
    pub fn buffer_empty(&self, widths: [i32; 6], heights: [i32; 6]) {
        log::info!("CubeMapTexture :: Buffering images to CubeMapTexture");
        self.bind();
        unsafe {
            for i in 0..6_usize {
                gl::TexImage2D(
                    gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    self.internal_format as i32,
                    widths[i],
                    heights[i],
                    0,
                    self.format,
                    self.data_type,
                    null(),
                );
                gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
            }
        }
        self.unbind();
    }

    /// Bind the slot associated to the texture.
    pub fn bind_slot(&self) {
        log::trace!("CubeMapTexture :: Binding texture slot");
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.tex_num);
        }
    }
}

impl Default for CubeMapTexture {
    fn default() -> Self {
        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
        }
        log::info!("CubeMapTexture :: Creating new CubeMapTexture {:?}", tex_id);
        return CubeMapTexture {
            _id: tex_id,
            tex_num: 0,
            internal_format: gl::RGB,
            format: gl::RGB,
            data_type: gl::UNSIGNED_BYTE,
            wrap_s: gl::REPEAT,
            wrap_t: gl::REPEAT,
            wrap_r: gl::REPEAT,
            min_filter: gl::LINEAR,
            mag_filter: gl::LINEAR,
        };
    }
}

impl Bindable for CubeMapTexture {
    fn bind(&self) {
        log::trace!(
            "CubeMapTexture :: Binding texture {:?} to slot {:?}",
            self.get_id(),
            self.tex_num
        );
        self.bind_slot();
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.get_id());
        }
    }

    fn unbind(&self) {
        log::trace!(
            "CubeMapTexture :: Unbinding texture {:?} to slot {:?}",
            self.get_id(),
            self.tex_num
        );
        self.bind_slot();
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }
    }

    fn delete(&self) {
        log::trace!(
            "CubeMapTexture :: Deleting texture {:?} bound to slot {:?}",
            self.get_id(),
            self.tex_num
        );
        self.bind();
        unsafe {
            gl::DeleteTextures(1, &self.get_id());
        }
    }
}

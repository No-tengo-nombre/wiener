use std::ptr::null;

use crate::{Bindable, HasID, Texture};

use gl;
use gl::types::*;
use log;
use wiener_utils::image;

/// 2D texture.
#[derive(Clone, Copy, Debug)]
pub struct Texture2D {
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

impl Texture for Texture2D {}

impl HasID for Texture2D {
    fn get_id(&self) -> u32 {
        return self._id;
    }
}

impl Texture2D {
    /// Change the slot of the texture.
    pub fn tex_num(mut self, new_bind: u32) -> Self {
        log::trace!("Texture2D :: Setting texture num {:?}", new_bind);
        self.tex_num = new_bind;
        return self;
    }

    /// Change the internal format of the texture.
    pub fn internal_format(mut self, new_format: GLenum) -> Self {
        log::trace!("Texture2D :: Setting internal format {:?}", new_format);
        self.internal_format = new_format;
        return self;
    }

    /// Change the format of the texture.
    pub fn format(mut self, new_format: GLenum) -> Self {
        log::trace!("Texture2D :: Setting format {:?}", new_format);
        self.format = new_format;
        return self;
    }

    /// Change the S wrapping method.
    pub fn wrap_s(mut self, new_wrap: GLenum) -> Self {
        log::trace!("Texture2D :: Setting wrap S {:?}", new_wrap);
        self.wrap_s = new_wrap;
        return self;
    }

    /// Change the T wrapping method.
    pub fn wrap_t(mut self, new_wrap: GLenum) -> Self {
        log::trace!("Texture2D :: Setting wrap T {:?}", new_wrap);
        self.wrap_t = new_wrap;
        return self;
    }

    /// Change the R wrapping method.
    pub fn wrap_r(mut self, new_wrap: GLenum) -> Self {
        log::trace!("Texture2D :: Setting wrap R {:?}", new_wrap);
        self.wrap_r = new_wrap;
        return self;
    }

    /// Change the min filtering method.
    pub fn min_filter(mut self, new_filter: GLenum) -> Self {
        log::trace!("Texture2D :: Setting min filter {:?}", new_filter);
        self.min_filter = new_filter;
        return self;
    }

    /// Change the max filtering method.
    pub fn mag_filter(mut self, new_filter: GLenum) -> Self {
        log::trace!("Texture2D :: Setting mag filter {:?}", new_filter);
        self.mag_filter = new_filter;
        return self;
    }

    /// Build the texture. After building, you should buffer the desired
    /// image.
    pub fn build(self) -> Self {
        log::info!("Texture2D :: Building Texture2D with parameters:\nWrap S {:?}\nWrap T {:?}\nWrap R {:?}\nMin filter {:?}\nMag filter {:?}", self.wrap_s, self.wrap_t, self.wrap_r, self.min_filter, self.mag_filter);
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_R, self.wrap_r as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                self.min_filter as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                self.mag_filter as i32,
            );
        }
        self.unbind();
        return self;
    }

    /// Buffer the given image to the texture.
    pub fn buffer_img<T>(self, data: &[T], width: i32, height: i32) -> Self {
        log::info!(
            "Texture2D :: Buffering {:?}x{:?} image to Texture2D",
            width,
            height
        );
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.internal_format as i32,
                width,
                height,
                0,
                self.format,
                self.data_type,
                data.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.unbind();
        return self;
    }

    /// Allocate memory for the texture without buffering anything.
    pub fn buffer_empty(self, width: i32, height: i32) -> Self {
        log::info!(
            "Texture2D :: Allocating for a {:?}x{:?} image",
            width,
            height
        );
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                self.internal_format as i32,
                width,
                height,
                0,
                self.format,
                self.data_type,
                null(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.unbind();
        return self;
    }

    /// Buffer an image contained in a file to the texture.
    pub fn buffer_from_file(self, filename: &str) -> Self {
        let (img, width, height) = image::load(filename);
        let data = img.to_rgba8().to_vec();
        return self.buffer_img(&data, width, height);
    }

    /// Bind the slot associated to the texture.
    pub fn bind_slot(&self) {
        log::trace!("Texture2D :: Binding texture slot");
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.tex_num);
        }
    }
}

impl Default for Texture2D {
    fn default() -> Self {
        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
        }
        log::info!("Texture2D :: Creating new Texture2D {:?}", tex_id);
        return Texture2D {
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

impl Bindable for Texture2D {
    fn bind(&self) {
        log::trace!(
            "Texture2D :: Binding texture {:?} to slot {:?}",
            self.get_id(),
            self.tex_num
        );
        self.bind_slot();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.get_id());
        }
    }

    fn unbind(&self) {
        log::trace!(
            "Texture2D :: Unbinding texture {:?} to slot {:?}",
            self.get_id(),
            self.tex_num
        );
        self.bind_slot();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    fn delete(&self) {
        log::trace!(
            "Texture2D :: Deleting texture {:?} bound to slot {:?}",
            self.get_id(),
            self.tex_num
        );
        self.bind();
        unsafe {
            gl::DeleteTextures(1, &self.get_id());
        }
    }
}

use gl;
use gl::types::*;
use wiener_utils::image;

pub struct Texture2D {
    _id: u32,
    pub _tex_num: u32,
    pub _format: GLenum,
    pub _wrap_s: GLenum,
    pub _wrap_t: GLenum,
    pub _min_filter: GLenum,
    pub _max_filter: GLenum,
}

impl Texture2D {
    pub fn builder() -> Self {
        let mut tex_id = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
        }
        return Texture2D {
            _id: tex_id,
            _tex_num: 0,
            _format: gl::RGB,
            _wrap_s: gl::REPEAT,
            _wrap_t: gl::REPEAT,
            _min_filter: gl::LINEAR,
            _max_filter: gl::LINEAR,
        };
    }

    pub fn tex_num(mut self, new_bind: u32) -> Self {
        self._tex_num = new_bind;
        return self;
    }

    pub fn format(mut self, new_format: GLenum) -> Self {
        self._format = new_format;
        return self;
    }

    pub fn wrap_s(mut self, new_wrap: GLenum) -> Self {
        self._wrap_s = new_wrap;
        return self;
    }

    pub fn wrap_t(mut self, new_wrap: GLenum) -> Self {
        self._wrap_t = new_wrap;
        return self;
    }

    pub fn min_filter(mut self, new_filter: GLenum) -> Self {
        self._min_filter = new_filter;
        return self;
    }

    pub fn max_filter(mut self, new_filter: GLenum) -> Self {
        self._max_filter = new_filter;
        return self;
    }

    pub fn build<T>(self) -> Self {
        self.bind();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self._wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self._wrap_t as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                self._min_filter as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                self._max_filter as i32,
            );
        }
        return self;
    }

    pub fn buffer_img<T>(self, data: &Vec<T>, width: i32, height: i32) -> Self {
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                self._format as i32,
                width,
                height,
                0,
                self._format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return self;
    }

    pub fn buffer_from_file(self, filename: &str) -> Self {
        let (img, width, height) = image::load(filename);
        let data = img.to_rgba8().into_vec();
        return self.buffer_img(&data, width, height);
    }

    pub fn bind_num(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self._tex_num);
        }
    }

    pub fn bind(&self) {
        self.bind_num();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self._id);
        }
    }

    pub fn unbind(&self) {
        self.bind_num();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

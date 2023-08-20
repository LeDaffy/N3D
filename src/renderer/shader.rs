use colored::Colorize;
use gl::{self, types::*};
use std::ffi::CString;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn uniform_f32(&self, name: &str, value: f32) {
        let name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }
    pub fn uniform_tex<N: Into<i32>>(&self, name: &str, value: N) {
        let name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value.into());
        }
    }
    pub fn uniform_mat4(&self, name: &str, mat: &nalgebra::base::Matrix4<f32>) {
        let name = CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }
    pub fn uniform_mat3(&self, name: &str, mat: &nalgebra::base::Matrix3<f32>) {
        let name = CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix3fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }
    pub fn uniform_vec2(&self, name: &str, x: f32, y: f32) {
        let name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform2f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y);
        }
    }
    pub fn uniform_vec3v(&self, name: &str, vec: &nalgebra::Vector3<f32>) {
        let name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name.as_ptr()),
                1,
                vec.as_ptr(),
            );
        }
    }
    pub fn from<P: AsRef<std::path::Path> + std::marker::Copy>(vs_path: P, fs_path: P) -> Shader {
        let vs_source = CString::new(std::fs::read_to_string(vs_path).unwrap()).unwrap();
        let vs = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

        unsafe {
            gl::ShaderSource(
                vs,
                1,
                &(vs_source.as_ptr()) as *const *const GLchar,
                std::ptr::null(),
            );
            gl::CompileShader(vs);
        }
        Self::print_shader_compilation(vs, vs_path);

        let fs_source = CString::new(std::fs::read_to_string(fs_path).unwrap()).unwrap();
        let fs = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        unsafe {
            gl::ShaderSource(
                fs,
                1,
                &(fs_source.as_ptr()) as *const *const GLchar,
                std::ptr::null(),
            );
            gl::CompileShader(fs);
        }
        Self::print_shader_compilation(fs, fs_path);

        // shader Program
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vs);
            gl::AttachShader(id, fs);
            gl::LinkProgram(id);
        }
        Self::print_shader_link(id);

        unsafe {
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }
        Shader { id: id }
    }

    pub fn new(vs: &str, fs: &str) -> Shader {
        let vs_source = CString::new(vs).unwrap();
        let vs = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

        unsafe {
            gl::ShaderSource(
                vs,
                1,
                &(vs_source.as_ptr()) as *const *const GLchar,
                std::ptr::null(),
            );
            gl::CompileShader(vs);
        }
        Self::print_shader_compilation(vs, "internal shader");

        let fs_source = CString::new(fs).unwrap();
        let fs = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        unsafe {
            gl::ShaderSource(
                fs,
                1,
                &(fs_source.as_ptr()) as *const *const GLchar,
                std::ptr::null(),
            );
            gl::CompileShader(fs);
        }
        Self::print_shader_compilation(fs,  "internal shader");

        // shader Program
        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vs);
            gl::AttachShader(id, fs);
            gl::LinkProgram(id);
        }
        Self::print_shader_link(id);

        unsafe {
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }
        Shader { id: id }
    }

    fn print_shader_compilation<P: AsRef<std::path::Path>>(shader: u32, path: P) {
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            print!("{0} ", path.as_ref().to_str().unwrap().bold());
            let info_log: Vec<u8> = vec![0; 1024];
            let info_log = unsafe { CString::from_vec_unchecked(info_log) };
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut i8,
                );
            }
            let info_log = info_log.into_string().unwrap();
            info_log.split_whitespace().for_each(|w| match w {
                "error:" => {
                    print!("{0}", "error: ".bold().red());
                }
                "warning:" => {
                    print!("{0}", "warning: ".bold().red());
                }
                _ => {
                    print!("{0} ", w.normal());
                }
            });
            println!("");
        }
    }
    fn print_shader_link(program: u32) {
        let mut success = 0;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let info_log: Vec<u8> = vec![0; 1024];
            let info_log = unsafe { CString::from_vec_unchecked(info_log) };
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    1024,
                    std::ptr::null_mut(),
                    info_log.as_ptr() as *mut i8,
                );
            }
            let info_log = info_log.into_string().unwrap();
            info_log.split_whitespace().for_each(|w| match w {
                "error:" => {
                    print!("{0}", "error: ".bold().red());
                }
                "warning:" => {
                    print!("{0}", "warning: ".bold().red());
                }
                _ => {
                    print!("{0} ", w.normal());
                }
            });
            println!("");
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

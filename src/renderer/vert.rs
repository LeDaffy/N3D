use nalgebra::{Point2, Point3};

#[derive(Debug)]
pub struct Vert {
    pub pos: Point3<f32>,
    pub uv: Point2<f32>,
    pub col: Point3<f32>,
}
impl Vert {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 0.0),
            uv: Point2::new(0.0, 0.0),
            col: Point3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn from(coord: [f32; 3], uv: [f32; 2], col: [f32; 3]) -> Self {
        Self {
            pos: Point3::new(coord[0], coord[1], coord[2]),
            uv: Point2::new(uv[0], uv[1]),
            col: Point3::new(col[0], col[1], col[2]),
        }
    }
    pub fn from_pos(coord: [f32; 3]) -> Self {
        Self {
            pos: Point3::new(coord[0], coord[1], coord[2]),
            uv: Point2::new(0.0, 0.0),
            col: Point3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn from_pos_with_uv(coord: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            pos: Point3::new(coord[0], coord[1], coord[2]),
            uv: Point2::new(uv[0], uv[1]),
            col: Point3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn from_pos_with_col(coord: [f32; 3], col: [f32; 3]) -> Self {
        Self {
            pos: Point3::new(coord[0], coord[1], coord[2]),
            uv: Point2::new(0.0, 0.0),
            col: Point3::new(col[0], col[1], col[2]),
        }
    }
    pub const OFFSET_POS: usize = unsafe {
        let vert = std::mem::MaybeUninit::uninit();
        let vert_ptr: *const Vert = vert.as_ptr();

        // cast to u8 pointers so we get offset in bytes
        let vert_u8_ptr = vert_ptr as *const u8;
        let pos_u8_ptr = std::ptr::addr_of!((*vert_ptr).pos) as *const u8;

        pos_u8_ptr.offset_from(vert_u8_ptr) as usize
    };
    pub const OFFSET_UV: usize = unsafe {
        let vert = std::mem::MaybeUninit::uninit();
        let vert_ptr: *const Vert = vert.as_ptr();

        // cast to u8 pointers so we get offset in bytes
        let vert_u8_ptr = vert_ptr as *const u8;
        let uv_u8_ptr = std::ptr::addr_of!((*vert_ptr).uv) as *const u8;

        uv_u8_ptr.offset_from(vert_u8_ptr) as usize
    };
    pub const OFFSET_COL: usize = unsafe {
        let vert = std::mem::MaybeUninit::uninit();
        let vert_ptr: *const Vert = vert.as_ptr();

        // cast to u8 pointers so we get offset in bytes
        let vert_u8_ptr = vert_ptr as *const u8;
        let col_u8_ptr = std::ptr::addr_of!((*vert_ptr).col) as *const u8;

        col_u8_ptr.offset_from(vert_u8_ptr) as usize
    };
}

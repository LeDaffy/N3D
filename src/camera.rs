use nalgebra::{self};

/// 3D Camera
pub struct Camera {
    /// Position in space
    pub pos: nalgebra::Point3<f32>,
    pub look_at: nalgebra:: Point3<f32>,
    /// Direction the camera is looking
    pub dir: nalgebra::Vector3<f32>,
    /// Right vector of the camera
    pub right: nalgebra::Vector3<f32>,
    /// Up vector of the camera
    pub up: nalgebra::Vector3<f32>,
    pub persp: nalgebra::Matrix4<f32>,
    pub ortho: nalgebra::Matrix4<f32>,
    pub rot: nalgebra::Matrix3<f32>,
    pub fov: f32,
}


impl Camera {
    /// Construct a new default camera
    pub fn new() -> Self {
        let pos = nalgebra::Point3::new(0.0, -5.0, 0.0);
        let look_at = nalgebra::Point3::new(0.0, 0.0, 0.0);
        // dir -- vector from look at to camera
        let dir: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, -1.0, 0.0);
        let right: nalgebra::Vector3<f32> = nalgebra::Vector3::new(1.0, 0.0, 0.0);
        let up: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.0, 1.0);
        Self { pos, look_at, dir, right, up, 
            persp: nalgebra_glm::perspective(1920.0/1080.0, 60f32.to_radians(), 0.01, 100.0),
            ortho: nalgebra::Matrix4::new_orthographic(-1.0, 1.0, -1.0, 1.0, 0.001, 1000.0),
            rot: nalgebra::Matrix3::identity(),
            fov: 60.0,
        }
    }

    /// create a view matrix from the current camera config
    pub fn view(&self) -> nalgebra::Matrix4<f32> {
        let r = self.right;
        let u = self.up;
        let d = self.dir;
        let p = self.pos;
        let mat4 = nalgebra::Matrix4::new(r.x, r.y, r.z, 0.0,
                                          u.x, u.y, u.z, 0.0,
                                          d.x, d.y, d.z, 0.0,
                                          0.0, 0.0, 0.0, 1.0,);
        let pos = nalgebra::Matrix4::new(1.0, 0.0, 0.0, -p.x,
                                         0.0, 1.0, 0.0, -p.y,
                                         0.0, 0.0, 1.0, -p.z,
                                         0.0, 0.0, 0.0, 1.0,);

        mat4 * pos
    }
    /// Move the camera in the direction of its  normal
    pub fn zoom(&mut self, distance: f32) {
        self.pos += (self.pos - self.look_at) / distance;
    }
    /// Return the distance from the camera to the lookat
    pub fn dist(&self) -> f32 {
        (self.look_at - self.pos).magnitude()
    }

    /// Pan the camera left/right and up/down
    pub fn pan(&mut self, right: f32, up: f32) {
        let dist_from_look = (self.pos - self.look_at).magnitude();
        let up_dir = self.up.normalize() * up * dist_from_look;
        let right_dir = self.right.normalize() * right * dist_from_look;
        self.pos += up_dir + right_dir;
        self.look_at += up_dir + right_dir;

    }
    /// Reset the pan of the camera to world origin
    pub fn reset(&mut self) {
        self.pos.x -= self.look_at.x;
        self.pos.y -= self.look_at.y;
        self.pos.z -= self.look_at.z;
        self.look_at = nalgebra::Point3::new(0.0, 0.0, 0.0);
    }

    /// Rotate along the camera local x axis, around the look at
    pub fn rotate_right(&mut self, mut angle: f32) {
        angle = angle.to_radians();
        let axis = self.right;
        let copy = self.look_at;

        self.pos.x -= self.look_at.x;
        self.pos.y -= self.look_at.y;
        self.pos.z -= self.look_at.z;
        self.look_at = nalgebra::Point3::new(0.0, 0.0, 0.0);
        self.pos = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.pos;
        self.dir = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.dir;
        self.up = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.up;

        self.dir = self.dir.normalize();
        self.up = self.up.normalize();

        self.pos.x += copy.x;
        self.pos.y += copy.y;
        self.pos.z += copy.z;
        self.look_at = copy;

        let c = angle.cos();
        let s = angle.sin();
        let a = self.right;
        self.rot = nalgebra::Matrix3::new(c + (1.0-c)*a.x*a.x    ,   (1.0-c)*a.x*a.y - s*a.z, (1.0-c)*a.x*a.z + s*a.y    ,
                                          (1.0-c)*a.x*a.y + s*a.z, c + (1.0-c)*a.y*a.y      ,     (1.0-c)*a.y*a.z - s*a.x,
                                          (1.0-c)*a.x*a.z - s*a.y, (1.0-c)*a.y*a.z + s*a.x  , c + (1.0-c)*a.z*a.z)       * self.rot;
    }

    /// Rotate along the global z axis, around the look at
    pub fn rotate_z(&mut self, mut angle: f32) {
        angle = angle.to_radians();
        let axis = nalgebra::Vector3::new(0.0, 0.0, 1.0);
        self.pos.x -= self.look_at.x;
        self.pos.y -= self.look_at.y;
        self.pos.z -= self.look_at.z;

        self.pos = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.pos;
        self.right = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.right;
        self.up = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.up;
        self.dir = nalgebra::Rotation3::from_axis_angle(&nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(axis.x, axis.y, axis.z)), angle) * self.dir;

        self.up = self.up.normalize();
        self.right = self.right.normalize();
        self.dir = self.dir.normalize();

        self.pos.x += self.look_at.x;
        self.pos.y += self.look_at.y;
        self.pos.z += self.look_at.z;


        self.rot   = nalgebra::Matrix3::new(angle.cos(), -angle.sin(), 0.0,
                                            angle.sin(),  angle.cos(), 0.0,
                                            0.0        ,  0.0        , 1.0) * self.rot;
    }
}

#[cfg(test)]
mod tests {
    use super::Camera;
    use nalgebra::{Point3, Vector3, Matrix4};
    #[test]
    fn new_camera() {
        let cam = Camera::new();
        let view =  Matrix4::look_at_rh(
                &Point3::new(0.0, -5.0, 0.0),
                &Point3::new(0.0, 0.0, 0.0),
                &Vector3::new(0.0, 0.0, 1.0),
                );
        println!("{}", cam.view());
        println!("{}", view);
        assert_eq!(cam.view(), view);
    }
    #[test]
    fn rotate_90_hor() {
        let mut cam = Camera::new();
        cam.rotate_z(90.0);
        println!("Cam Pos {}", cam.pos);
        assert!(cam.pos - Point3::new(5.0, 0.0, 0.0) < Vector3::new(0.01, 0.01, 0.01));
    }
    #[test]
    fn rotate_neg_90_hor() {
        let mut cam = Camera::new();
        cam.rotate_z(-90.0);
        println!("Cam Pos {}", cam.pos);
        assert!(cam.pos - Point3::new(-5.0, 0.0, 0.0) < Vector3::new(0.01, 0.01, 0.01));
    }
    #[test]
    fn rotate_45_hor() {
        let mut cam = Camera::new();
        cam.rotate_z(45.0);
        println!("Cam Pos {}", cam.pos);
        assert!(cam.pos - Point3::new(3.53553, -3.53553, 0.0) < Vector3::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn rotate_45_45() {
        let mut cam = Camera::new();
        cam.rotate_z(45.0);
        cam.rotate_x(45.0);
        println!("Cam Pos {}", cam.pos);
        assert!(cam.pos - Point3::new(2.5, -2.5, 3.53553) < Vector3::new(0.1, 0.1, 0.1));
    }
}

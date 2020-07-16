use glam::*;

use gl::types::GLuint;

use glow::HasContext;

#[derive(Copy, Clone)]
pub struct Camera {
    pub fovy: f32,
    pub z_near: f32,
    pub z_far: f32,

    pub aperture: f32,
    pub shutter_speed: f32,
    pub iso: f32,

    pub position: Vec3,
    pub rotation: Quat,
}

impl Camera {
    pub fn new(fovy: f32, z_near: f32, z_far: f32, aperture: f32, shutter_speed: f32, iso: f32, position: Vec3, rotation: Quat) -> Camera { //eye: Point3<f32>, look_at: Point3<f32>, up: Vector3<f32>
        Camera {
            fovy: fovy,
            z_near: z_near,
            z_far: z_far,

            aperture: aperture,
            shutter_speed: shutter_speed,
            iso: iso,

            position: position,
            rotation: rotation,
        }
    }

    pub fn default() -> Camera {
        Camera {
            fovy: 60.0,
            z_near: 0.02,
            z_far: 1000.0,

            aperture: 16.0,
            shutter_speed: 1.0 / 100.0,
            iso: 100.0,

            position: Vec3::new(0.0, 24.0, -70.0),
            rotation: Quat::from_rotation_y(0.0), //Rotation3::<f32>::from_angle_y(Rad(3.14 / 2.0))
        }
    }

    pub fn get_proj(&self, width: u32, height: u32) -> Mat4 {
        // perspective(Rad(self.fovy / 180.0 * std::f32::consts::PI), width as f32 / height as f32, self.z_near, self.z_far)
        Mat4::perspective_rh_gl(self.fovy / 180.0 * std::f32::consts::PI, width as f32 / height as f32, self.z_near, self.z_far)
    }

    pub fn get_view(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.position)
    }

    pub fn upload_fields(&self, gl: &glow::Context, handle: GLuint) {
        unsafe {
            let pos_loc = gl.get_uniform_location(handle, "camera.position");
            gl.uniform_3_f32(pos_loc, self.position.x(), self.position.y(), self.position.z());

            let set_loc = gl.get_uniform_location(handle, "camera.settings");
            gl.uniform_3_f32(set_loc, self.aperture, self.shutter_speed, self.iso);
        }
    }
}

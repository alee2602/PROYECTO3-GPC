use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool,
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    // Rotación en órbita
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        let radius_xz =
            (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        let new_eye = self.center
            + Vec3::new(
                radius * new_yaw.cos() * new_pitch.cos(),
                -radius * new_pitch.sin(),
                radius * new_yaw.sin() * new_pitch.cos(),
            );

        self.eye = new_eye;
        self.has_changed = true;
    }

    // Movimiento vertical en el eje Y
    pub fn move_vertical(&mut self, delta: f32) {
        self.eye.y += delta;
        self.center.y += delta;
        self.has_changed = true;
    }

    // Movimiento en el plano horizontal (XZ)
    pub fn move_center(&mut self, movement: Vec3) {
        self.eye += movement;
        self.center += movement;
        self.up = Vec3::new(0.0, 1.0, 0.0); 
    }

    // Zoom hacia adelante y atrás
    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * delta;
        self.has_changed = true;
    }
}

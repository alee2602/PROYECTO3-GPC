use nalgebra_glm::{dot, Vec3};

pub struct Intersect {
    pub hit: bool,        // Indica si el rayo interceptó un objeto
    pub distance: f32,    // Distancia desde el origen del rayo al punto de intersección
    pub point: Vec3,      // Punto de intersección en el espacio 3D
    pub normal: Vec3,     // Normal en el punto de intersección
    pub uv: (f32, f32),   // Coordenadas UV para texturas
}

impl Intersect {
    pub fn new(hit: bool, distance: f32, point: Vec3, normal: Vec3, uv: (f32, f32)) -> Self {
        Intersect {
            hit,
            distance,
            point,
            normal,
            uv,
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}

// Estructura que representa una esfera (usada como skybox)
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

// Implementación de la intersección para una esfera
impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersect::new(false, 0.0, Vec3::zeros(), Vec3::zeros(), (0.0, 0.0))
        } else {
            let dist = (-b - discriminant.sqrt()) / (2.0 * a);
            let hit_point = ray_origin + ray_direction * dist;
            let normal = (hit_point - self.center).normalize();

            // Calcula las coordenadas UV basadas en la posición en la esfera
            let u = 0.5 + normal.z.atan2(normal.x) / (2.0 * std::f32::consts::PI);
            let v = 0.5 - normal.y.asin() / std::f32::consts::PI;

            Intersect::new(true, dist, hit_point, normal, (u, v))
        }
    }
}
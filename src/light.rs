use crate::*;

pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    pub in_shadow:bool
}
impl Light {
    pub fn new(
        position: Vec3, 
        intensity: f64
    ) -> Box<Self> {
        Box::new(Self {
            position,
            intensity,
            in_shadow:false
        })
    }
}
// Fonction pour vérifier si un point est dans l'ombre
pub fn is_in_shadow(direction: &Vec3, origin: &Point3, objects: &[Box<dyn Hittable>]) -> bool {
    let shadow_ray = Ray::new(*origin, *direction);
    let mut temp_rec = HitRecord::new();
    for object in objects {
        if object.hit(&shadow_ray, 0.001, common::INFINITY, &mut temp_rec) {
            return true;
        }
    }
    false
}

pub fn subtract(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3(v1.x() - v2.x(), v1.y() - v2.y(), v1.z() - v2.z())
}
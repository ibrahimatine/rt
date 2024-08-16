use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub mat: Option<Rc<dyn Material>>,
}

impl Cube {
    pub fn new(min: Point3, max: Point3, mat: Option<Rc<dyn Material>>) -> Cube {
        Cube { min, max, mat }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Implémentation simplifiée pour détecter si le rayon touche le cube
        let inv_d = Vec3::new(1.0 / ray.direction().x(), 1.0 / ray.direction().y(), 1.0 / ray.direction().z());
        let t0s = (self.min - ray.origin()) * inv_d;
        let t1s = (self.max - ray.origin()) * inv_d;
        
        let tmin_x = t0s.x().min(t1s.x());
        let tmin_y = t0s.y().min(t1s.y());
        let tmin_z = t0s.z().min(t1s.z());

        let tmax_x = t0s.x().max(t1s.x());
        let tmax_y = t0s.y().max(t1s.y());
        let tmax_z = t0s.z().max(t1s.z());

        let tmin = tmin_x.max(tmin_y).max(tmin_z);
        let tmax = tmax_x.min(tmax_y).min(tmax_z);

        if tmax < tmin || tmin > t_max || tmax < t_min {
            return false;
        }

        rec.t = tmin;
        rec.p = ray.at(rec.t);
        rec.normal = Vec3::new(0.0, 1.0, 0.0); // Ajustez pour obtenir la normale correcte
        rec.mat = self.mat.clone();
        rec.set_face_normal(ray, rec.normal);
        true
    }
}

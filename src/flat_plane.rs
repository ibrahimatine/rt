use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
use std::rc::Rc;

pub struct Plane {
    pub point: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, mat: Rc<dyn Material>) -> Plane {
        Plane { point, normal, mat }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let denom = vec3::dot(self.normal, r.direction());
        if denom.abs() > 1e-6 {
            let t = vec3::dot(self.point - r.origin(), self.normal) / denom;
            if t < t_max && t > t_min {
                rec.t = t;
                rec.p = r.at(t);
                rec.set_face_normal(r, self.normal);
                rec.mat = Some(self.mat.clone());
                return true;
            }
        }
        false
    }
}

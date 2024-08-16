use std::rc::Rc;

use material::Material;
use vec3::Vec3;

use crate::*;
pub struct Cylinder {
    center: Vec3,
    radius: f64,
    height: f64,
    mat: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, m: Rc<dyn Material>) -> Box<Self> {
        Box::new(Self {
            center,
            radius,
            height,
            mat: m,
        })
    }

    fn hit_cylinder(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().x() * ray.direction().x() + ray.direction().z() * ray.direction().z();
        let b = 2.0 * (oc.x() * ray.direction().x() + oc.z() * ray.direction().z());
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let mut root = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        if root < t_min || root > t_max {
            root = (-b + f64::sqrt(discriminant)) / (2.0 * a);
            if root < t_min || root > t_max {
                return false;
            }
        }
        
        let t = root;
        let y = ray.origin().y() + t * ray.direction().y();
        if y < self.center.y() || y > self.center.y() + self.height {
            return false;
        }
        
        rec.t = t;
        rec.p = ray.at(t);
        let outward_normal = Vec3::new(rec.p.x() - self.center.x(), 0.0, rec.p.z() - self.center.z()).unit();
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());
        
        true
    }

    fn hit_disk(&self, center: Vec3, radius: f64, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (center.y() - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        
        let p = ray.at(t);
        if (p - center).length() > radius {
            return false;
        }
        
        rec.t = t;
        rec.p = p;
        rec.set_face_normal(ray, Vec3::new(0.0, 1.0, 0.0));
        rec.mat = Some(self.mat.clone());
        
        true
    }

}

impl Hittable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        if self.hit_cylinder(r, t_min, closest_so_far, rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }

        let bottom_center = self.center;
        if self.hit_disk(bottom_center, self.radius, r, t_min, closest_so_far, rec) {
            hit_anything = true;
            closest_so_far = rec.t;
        }

        let top_center = self.center + Vec3::new(0.0, self.height, 0.0);
        if self.hit_disk(top_center, self.radius, r, t_min, closest_so_far, rec) {
            hit_anything = true;
        }

        hit_anything
    }
}

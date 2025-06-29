use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;
// 存储射线命中物体时的记录信息
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,    // 命中点坐标
    pub normal: Vec3, // 命中点法线（单位向量）
    pub t: f64,       // 射线参数 t
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

// 可命中物体的统一接口
pub trait Hittable {
    // 判断射线是否命中物体，若命中则填充 HitRecord
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // assume outward_normal has unit length
        // 确保法线始终指向射线的入射方向的反方向
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: Arc::new(Lambertian::default()), // 或某个默认材质
            t: 0.0,
            front_face: false,
        }
    }
}

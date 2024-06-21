use std::mem::swap;

use nalgebra_glm::{DMat3, DMat4, DVec3, DVec4, Vec3};

pub struct Ray64 {
    // Starting point
    r0: DVec3,
    // velocity vector
    v: DVec3,
}

impl Ray64 {
    pub fn new(starting_point: DVec3, direction: DVec3) -> Self {
        Self {
            r0: starting_point,
            v: nalgebra_glm::normalize(&direction),
        }
    }
    pub fn get_point(&self, t: f64) -> DVec3 {
        self.r0 + t * self.v
    }
}

pub struct CollisionBox {
    center: DVec3,
    x_length: f64,
    y_length: f64,
    z_length: f64,
}

impl CollisionBox {
    pub fn new(x: f64, y: f64, z: f64, center: Option<DVec3>) -> Self {
        Self {
            center: center.unwrap_or(Default::default()),
            x_length: x,
            y_length: y,
            z_length: z,
        }
    }
    pub fn move_to(&mut self, new_center: DVec3) -> &mut Self {
        self.center = new_center;
        self
    }
    pub fn translate_by(&mut self, translation: DVec3) -> &mut Self {
        self.center += translation;
        self
    }
    pub fn scale_by(&mut self, factor: f64) -> &mut Self {
        self.x_length *= factor;
        self.y_length *= factor;
        self.z_length *= factor;
        self
    }
}

pub fn detect_collision(ray: &Ray64, collision_box: &CollisionBox) -> bool {
    let inverses = (1.0 / ray.v.x, 1.0 / ray.v.y, 1.0 / ray.v.z);

    let mut tx0 = (-collision_box.x_length / 2.0 - ray.r0.x) * inverses.0;
    let mut tx1 = (collision_box.x_length / 2.0 - ray.r0.x) * inverses.0;

    let mut ty0 = (-collision_box.y_length / 2.0 - ray.r0.y) * inverses.1;
    let mut ty1 = (collision_box.y_length / 2.0 - ray.r0.y) * inverses.1;

    let mut tz0 = (-collision_box.z_length / 2.0 - ray.r0.z) * inverses.2;
    let mut tz1 = (collision_box.z_length / 2.0 - ray.r0.z) * inverses.2;

    if ray.v.x < 0.0 {
        swap(&mut tx0, &mut tx1);
    }
    if ray.v.y < 0.0 {
        swap(&mut ty0, &mut ty1);
    }
    if ray.v.z < 0.0 {
        swap(&mut tz0, &mut tz1);
    }

    tx0.max(ty0).max(tz0) < tx1.min(ty1).min(tz1)
}

pub struct Ray64V2 {
    // Starting point
    pub r0: DVec4,
    // velocity vector
    pub v: DVec4,
}

impl Ray64V2 {
    pub fn new(starting_point: DVec3, direction: DVec3) -> Self {
        let normalized_direction = nalgebra_glm::normalize(&direction);
        Self {
            r0: DVec4::new(starting_point.x, starting_point.y, starting_point.z, 1.0),
            v: DVec4::new(
                normalized_direction.x,
                normalized_direction.y,
                normalized_direction.z,
                0.0,
            ),
        }
    }
    pub fn new_from_4(starting_point: DVec4, direction: DVec4) -> Self {
        let normalized_direction = nalgebra_glm::normalize(&direction);
        Self {
            r0: DVec4::new(starting_point.x, starting_point.y, starting_point.z, 1.0),
            v: DVec4::new(
                normalized_direction.x,
                normalized_direction.y,
                normalized_direction.z,
                0.0,
            ),
        }
    }
    pub fn get_point(&self, t: f64) -> DVec3 {
        let point = self.r0 + t * self.v;
        return unsafe { *(&point as *const DVec4 as *const DVec3) };
    }
}

pub struct CollisionBoxV2 {
    box_half_size: DVec3,
    size_reciprocal: DVec3,
    translation: DVec3,
    rotation: DMat3,
    pub relative_transformation: DMat4,
}
impl Default for CollisionBoxV2 {
    fn default() -> Self {
        Self {
            translation: DVec3::zeros(),
            box_half_size: DVec3::new(1.0, 1.0, 1.0),
            size_reciprocal: DVec3::new(1.0, 1.0, 1.0),
            rotation: DMat3::identity(),
            relative_transformation: DMat4::identity(),
        }
    }
}

impl CollisionBoxV2 {
    pub fn new_in_origin(box_size: DVec3) -> Self {
        Self {
            box_half_size: box_size / 2.0, // half size is more usefull
            size_reciprocal: DVec3::new(1.0 / box_size.x, 1.0 / box_size.y, 1.0 / box_size.z),
            ..Default::default()
        }
    }

    pub fn rescale(&mut self, factor: &DVec3) -> &mut Self {
        self.box_half_size.x *= factor.x;
        self.box_half_size.y *= factor.y;
        self.box_half_size.z *= factor.z;

        self.size_reciprocal.x /= factor.x;
        self.size_reciprocal.y /= factor.y;
        self.size_reciprocal.z /= factor.z;
        self
    }

    pub fn update_transformation(&mut self) {
        // update rotation (Inversed - Transposed)
        for r in 0..3 {
            for c in 0..3 {
                *self.relative_transformation.get_mut((c, r)).unwrap() =
                    *self.rotation.get((r, c)).unwrap();
            }
        }
        //update translation (Inversed)
        self.relative_transformation[12] = -self.translation.x;
        self.relative_transformation[13] = -self.translation.y;
        self.relative_transformation[14] = -self.translation.z;
    }

    pub fn translate_box(&mut self, translation: DVec3) -> &mut Self {
        self.translation += translation;
        // self.update_transformation();
        self
    }

    pub fn set_translation(&mut self, translation: DVec3) -> &mut Self {
        self.translation = translation;
        // self.update_transformation();
        self
    }

    pub fn rotate_box(&mut self, rotation: DMat3) -> &mut Self {
        self.rotation *= rotation;
        // self.update_transformation();
        self
    }

    pub fn set_rotation(&mut self, rotation: DMat3) -> &mut Self {
        self.rotation = rotation;
        // self.update_transformation();
        self
    }
    pub fn detect_collision(&self, ray_: &Ray64V2) -> Option<f64> {
        // let inverses = (1.0 / ray.v.x, 1.0 / ray.v.y, 1.0 / ray.v.z);
        // let ray = Ray64::new(
        //     ray_.r0 - self.translation,
        //     self.rotation.transpose() * ray_.v,
        // );

        let new_location = self.relative_transformation * ray_.r0;
        let new_direction = self.relative_transformation * ray_.v;
        let direction_reciprocal = (
            1.0 / new_direction.x,
            1.0 / new_direction.y,
            1.0 / new_direction.z,
        );

        let mut tx0 = (-self.box_half_size.x - new_location.x) * direction_reciprocal.0;
        let mut tx1 = (self.box_half_size.x - new_location.x) * direction_reciprocal.0;

        let mut ty0 = (-self.box_half_size.y - new_location.y) * direction_reciprocal.1;
        let mut ty1 = (self.box_half_size.y - new_location.y) * direction_reciprocal.1;

        let mut tz0 = (-self.box_half_size.z - new_location.z) * direction_reciprocal.2;
        let mut tz1 = (self.box_half_size.z - new_location.z) * direction_reciprocal.2;

        if new_direction.x < 0.0 {
            swap(&mut tx0, &mut tx1);
        }
        if new_direction.y < 0.0 {
            swap(&mut ty0, &mut ty1);
        }
        if new_direction.z < 0.0 {
            swap(&mut tz0, &mut tz1);
        }

        let t0 = tx0.max(ty0).max(tz0);
        let t1 = tx1.min(ty1).min(tz1);

        if t0 < t1 {
            return Some(t0);
        } else {
            return None;
        }
    }
}

trait Shape {
    fn test_collision(&self, ray: &Ray64) -> Option<f64>;
}


use nalgebra_glm::{DMat4, DVec2, DVec3, DVec4};

pub struct Ray {
    staring_point: DVec4,
    direction: DVec4,
    // d_square:DVec3,
    // l_square:DVec3,
    // dl:DVec3
}
impl Ray {
    pub fn new(starting_point: DVec3, direction: DVec3) -> Self {
        Self {
            staring_point: DVec4::new(starting_point.x, starting_point.y, starting_point.z, 1.0),
            direction: nalgebra_glm::normalize(&DVec4::new(
                direction.x,
                direction.y,
                direction.z,
                0.0,
            )),
        }
    }
    pub fn new_v4(starting_point: DVec4, direction: DVec4) -> Self {
        Self {
            staring_point: starting_point,
            direction: direction,
        }
        // Self {
        //     staring_point: DVec4::new(starting_point.x, starting_point.y, starting_point.z, 1.0),
        //     direction: nalgebra_glm::normalize(&DVec4::new(
        //         direction.x,
        //         direction.y,
        //         direction.z,
        //         0.0,
        //     )),
        // }
    }
    pub fn get_point(&self, t: f64) -> DVec4 {
        self.staring_point + t * self.direction
    }
    pub fn get_point3d(&self, t: f64) -> DVec3 {
        // let tmp = self.staring_point + t * self.direction;
        return DVec3::new(
            self.staring_point.x + t * self.direction.x,
            self.staring_point.y + t * self.direction.y,
            self.staring_point.z + t * self.direction.z,
        );
    }
}
pub trait Collidable<const N: usize> {
    //returns DVec4 -> [x,y,z,t]
    fn detect_collision(&self, ray: &Ray) -> Option<[(f64, DVec3); N]>;
    fn detect_first_collision(&self, ray: &Ray) -> Option<(f64, DVec3)>;
    //fn find_collision_point(&self,ray:&Ray)->Option<DVec4>;
}

pub struct Ellipse {
    center: DVec4,
    size: DVec2,
    transformation: DMat4,
}
impl Ellipse {
    pub fn new_closure(ellipsoid: &Ellipsoid) -> Self {
        let mut center = ellipsoid.center;
        center.x += ellipsoid.x_section;

        let size_yz = DVec2::new(
            ellipsoid.size.y * (1.0 - (ellipsoid.x_section / ellipsoid.size.x).powi(2)).sqrt(),
            ellipsoid.size.z * (1.0 - (ellipsoid.x_section / ellipsoid.size.x).powi(2)).sqrt(),
        );

        let mut relative_transformation = ellipsoid.relative_transformation;
        relative_transformation.m14 = -center.x;

        Self {
            center: center,
            size: size_yz,
            transformation: relative_transformation,
        }
    }
}
impl Collidable<1> for Ellipse {
    fn detect_collision(&self, world_ray: &Ray) -> Option<[(f64, DVec3); 1]> {
        let ray = Ray::new_v4(
            self.transformation * world_ray.staring_point,
            self.transformation * world_ray.direction,
        );

        let t = -ray.staring_point.x / ray.direction.x;

        if t < 0.0 {
            return None;
        }

        let collision = ray.get_point3d(t);

        if (collision.y / self.size[0]).powi(2) + (collision.z / self.size[1]).powi(2) < 1.0 {
            return Some([(t, collision)]);
        }

        None
    }
    fn detect_first_collision(&self, ray: &Ray) -> Option<(f64, DVec3)> {
        self.detect_collision(&ray);
        None
    }
}

/// ## Description
/// A struct to describe an ellipsoid, with a center , axis lengths and section end
pub struct Ellipsoid {
    center: DVec4,
    size: DVec3,
    size_reciprocal: DVec3,
    x_direction: DVec4,
    //section: DVec3,
    x_section: f64,
    relative_transformation: DMat4,
}

impl Ellipsoid {
    pub fn new(size: DVec3, center: DVec3, x_direction: DVec3, x_section: f64) -> Self {
        let x_dir = nalgebra_glm::normalize(&x_direction);
        let y_dir = nalgebra_glm::DVec3::y();
        let z_dir = x_dir.cross(&y_dir);

        // let transformation = [
        //     x_dir.x,
        //     y_dir.x,
        //     z_dir.x,
        //     0.0,
        //     x_dir.y,
        //     y_dir.y,
        //     z_dir.y,
        //     0.0,
        //     x_dir.z,
        //     y_dir.z,
        //     z_dir.z,
        //     0.0 - center.x,
        //     -center.y,
        //     -center.z,
        //     1.0,
        // ];

        Self {
            center: DVec4::new(center.x, center.y, center.z, 1.0),
            size: size,
            size_reciprocal: DVec3::new(1.0 / size.x, 1.0 / size.y, 1.0 / size.z),

            x_direction: DVec4::new(x_dir.x, x_dir.y, x_dir.z, 0.0),
            x_section: x_section,
            relative_transformation: DMat4::new(
                x_dir.x, x_dir.y, x_dir.z, -center.x, y_dir.x, y_dir.y, y_dir.z, -center.y,
                z_dir.x, z_dir.y, z_dir.z, -center.z, 0.0, 0.0, 0.0, 1.0,
            ), //unsafe{* (&transformation[0] as *const f64 as *const DMat4)},
        }
    }
}

impl Ellipsoid {
    /// ## Description
    /// * Finds collision points of a `Ray` on an `Ellipsoid`
    /// ### Notes
    /// * The ray is described in absolute coordinates, relative coordinated is calculated under this function
    /// * This function doesn't provide information about direction of incidence
    /// * Always returns two points (All solutions)
    pub fn detect_ellipsoid_collision(&self, world_ray: &Ray) -> Option<[(f64, DVec3); 2]> {
        let ray = Ray::new_v4(
            self.relative_transformation * world_ray.staring_point,
            self.relative_transformation * world_ray.direction,
        );

        let A = ray.direction.x.powi(2) * self.size_reciprocal.x
            + ray.direction.y.powi(2) * self.size_reciprocal.y
            + ray.direction.z.powi(2) * self.size_reciprocal.z;

        let B = 2.0 * ray.direction.x * ray.staring_point.x * self.size_reciprocal.x
            + 2.0 * ray.direction.y * ray.staring_point.y * self.size_reciprocal.y
            + 2.0 * ray.direction.z * ray.staring_point.z * self.size_reciprocal.z;

        let C = ray.staring_point.x.powi(2) * self.size_reciprocal.x
            + ray.staring_point.y.powi(2) * self.size_reciprocal.y
            + ray.staring_point.z.powi(2) * self.size_reciprocal.z
            - 1.0; //t4 + t5 + t6 - 1.0;

        let discriminant = B * B - 4.0 * A * C;
        if discriminant < 0.0 {
            return None;
        } else {
            let d_sqrt = discriminant.sqrt();
            let res1 = (-B + d_sqrt) / (2.0 * A);
            let res2 = (-B - d_sqrt) / (2.0 * A);
            let intersections = if res1 < res2 {
                [(res1, ray.get_point3d(res1)), (res2, ray.get_point3d(res2))]
            } else {
                [(res2, ray.get_point3d(res2)), (res1, ray.get_point3d(res1))]
            };

            return Some(intersections);
        }
    }
}
impl Collidable<2> for Ellipsoid {
    fn detect_collision(&self, ray: &Ray) -> Option<[(f64, DVec3); 2]> {
        self.detect_ellipsoid_collision(&ray)
    }
    /// ## Description
    /// * Returns the first `Ellipsoid`-`Ray` intersection
    fn detect_first_collision(&self, world_ray: &Ray) -> Option<(f64, DVec3)> {
        let collisions = self.detect_ellipsoid_collision(&world_ray);
        if let Some(valid_collisions) = collisions {
            let p1_valid =
                (valid_collisions[0].1.x > self.x_section) && valid_collisions[0].0 >= 0.0;
            let p2_valid =
                (valid_collisions[1].1.x > self.x_section) && valid_collisions[1].0 >= 0.0;
            if p1_valid {
                if p2_valid {
                    return if valid_collisions[0].0 < valid_collisions[1].0 {
                        Some(valid_collisions[0])
                    } else {
                        Some(valid_collisions[1])
                    };
                }
                return Some(valid_collisions[0]);
            } else if p2_valid {
                return Some(valid_collisions[1]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

pub struct LensType1 {
    ellipsoid: Ellipsoid,
    ellipse: Ellipse,
}

// impl Collidable<2> for LensType1 {
//     fn detect_collision(&self, ray: &Ray) -> Option<[(f64, DVec3); 2]> {
//         let ellipsoid_collision = self.ellipsoid.detect_collision(&ray);
//         if let Some(valid_ellipsoid_collisions) = ellipsoid_collision {
//             let ellipse_collision = self.ellipse.detect_collision(&ray);
//             if let Some(valid_ellipse_collision) = ellipse_collision {
                
//             }
//         }
//     }
//     fn detect_first_collision(&self, ray: &Ray) -> Option<(f64, DVec3)> {}
// }

use std::{f64::consts::PI, time::Instant};

use nalgebra_glm::{DMat4, DVec3, DVec4};
use ray_tracing::{
    self,
    core::{detect_collision, CollisionBoxV2, Ray64V2},
    coreium::{self, Collidable},
};

struct Sphere {
    center: DVec4,
    radius: f64,
}

fn ray_sphere_collision(sphere: &Sphere, ray: &Ray64V2) -> Option<f64> {
    let relative_position = ray.r0 - sphere.center;
    let tmp_ray = Ray64V2::new_from_4(relative_position, ray.v);

    let A = ray.v.x.powi(2) + ray.v.y.powi(2);
    // let tx = ray.r0.x - sphere.center.x;
    // let ty = ray.r0.y - sphere.center.y;
    let B = 2.0 * relative_position.sum(); //2.0 * ray.v.x * tx + 2.0 * ray.v.y * ty;
    let C = relative_position.x.powi(2) + relative_position.y.powi(2) + relative_position.z.powi(2)
        - sphere.radius.powi(2); //tx.powi(2) + ty.powi(2) - sphere.radius.powi(2);

    let field = B * B - 4.0 * A * C;
    if field < 0.0 {
        return None;
    } else {
        let a = field.sqrt();
        let b = -B;
        // let result1 = (-B + field.sqrt()) / (2.0 * A);
        // let result2 = result1+
        let r1 = (b + a) / (2.0 * A);
        let r2 = (b - a) / (2.0 * A);
        if r1 < 0.0 && r2 < 0.0 {
            return None;
        } else if r1 > 0.0 && r2 > 0.0 {
            return Some(r1.min(r2));
        } else {
            return Some(r1.max(r2));
        }
    }
}

fn ray_sphere_collision2(sphere: &Sphere, ray: &Ray64V2) -> Option<f64> {
    let relative_position = ray.r0 - sphere.center;
    let tmp_ray = Ray64V2::new_from_4(relative_position, ray.v);

    let t = -unsafe {
        nalgebra_glm::dot(
            &*(&tmp_ray.r0 as *const DVec4 as *const DVec3),
            &*(&tmp_ray.v as *const DVec4 as *const DVec3),
        )
    };
    let pnt = tmp_ray.get_point(t);
    // let pnt3 = &unsafe { *(&pnt as *const DVec4 as *const DVec3) };

    if pnt.magnitude() > sphere.radius {
        return None;
    }

    let A = ray.v.x.powi(2) + ray.v.y.powi(2);
    // let tx = ray.r0.x - sphere.center.x;
    // let ty = ray.r0.y - sphere.center.y;
    let B = 2.0 * relative_position.sum(); //2.0 * ray.v.x * tx + 2.0 * ray.v.y * ty;
    let C = relative_position.x.powi(2) + relative_position.y.powi(2) + relative_position.z.powi(2)
        - sphere.radius.powi(2); //tx.powi(2) + ty.powi(2) - sphere.radius.powi(2);

    let field = B * B - 4.0 * A * C;
    if field < 0.0 {
        return None;
    } else {
        let a = field.sqrt();
        let b = -B;
        // let result1 = (-B + field.sqrt()) / (2.0 * A);
        // let result2 = result1+
        let r1 = (b + a) / (2.0 * A);
        let r2 = (b - a) / (2.0 * A);
        if r1 < 0.0 && r2 < 0.0 {
            return None;
        } else if r1 > 0.0 && r2 > 0.0 {
            return Some(r1.min(r2));
        } else {
            return Some(r1.max(r2));
        }
    }
}

use opencv::{
    highgui::{imshow, wait_key},
    imgcodecs::IMREAD_ANYCOLOR,
    prelude::*,
};

fn main() {
    let test_object =
        coreium::Ellipsoid::new(DVec3::new(1.0, 1.0, 1.0), DVec3::zeros(), DVec3::x(), 0.700);
    let ellipse_closure = coreium::Ellipse::new_closure(&test_object);
    let my_ray = coreium::Ray::new(DVec3::new(2.0, 2.0, 0.0), DVec3::new(-1.0, -1.0, 0.0));

    println!("{:?}", test_object.detect_ellipsoid_collision(&my_ray));
    println!("{:?}", test_object.detect_first_collision(&my_ray));
    println!("{:?}", ellipse_closure.detect_collision(&my_ray));
}

fn main4() {
    let img = opencv::imgcodecs::imread(
        "/home/hussein/Pictures/sergio-grana-abandonedtrain-shot-010-cc-03.jpg",
        IMREAD_ANYCOLOR,
    )
    .unwrap();
    imshow("HELLO", &img);
    wait_key(0);

    let mysphere = Sphere {
        radius: 1.0,
        center: DVec4::new(0.0, 0.0, 0.0, 1.0),
    };

    let mut sum = 0.0;
    let a = Instant::now();
    for i in 0..100000000 {
        let myray = Ray64V2::new(DVec3::new(-10.0, 0.0, 0.0), DVec3::new(1.0, 0.0, 0.0));
        let collision_time = ray_sphere_collision2(&mysphere, &myray);
        sum += collision_time.unwrap_or(0.0);
    }
    let b = Instant::now();
    println!("{:?}", (sum, (b - a).as_millis()));
}

fn main3() {
    let vec1 = DVec3::new(1.0, 1.0, 1.0);
    let mut rotation = nalgebra_glm::translate(&DMat4::identity(), &DVec3::new(1.1, 2.0, 3.0))
        * &nalgebra_glm::rotation(PI / 2.0, &DVec3::new(0.0, 0.0, 1.0));

    for i in 0..16 {
        println!("{i}: {:?}", rotation[i]);
    }

    let mut tmp: Option<f64> = None;
    let a = Instant::now();
    let mut x = CollisionBoxV2::new_in_origin(DVec3::new(2.0, 2.0, 2.0));
    let mut counter = 0;
    x.set_translation(DVec3::new(1.0, 2.0, 0.0))
        .update_transformation();

    for i in 0..1000000 {
        tmp = x.detect_collision(&Ray64V2::new(
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(i as f64, 0.500002, 0.0),
        ));
        if tmp.is_none() {
            counter += 1;
        }
    }
    let b = Instant::now();
    println!("{:?}", (tmp, (b - a).as_micros(), counter));

    println!("{:?}", tmp); //(x.relative_transformation,res));
}

fn main2() {
    let x = ray_tracing::core::CollisionBox::new(2.0, 2.0, 2.0, None);
    let r =
        ray_tracing::core::Ray64::new(DVec3::new(2.0, 0.10, 0.0), DVec3::new(-1.0, -1.101, 0.0));
    let mut tmp: bool = false;
    let a = Instant::now();
    for i in 0..1000000 {
        tmp = detect_collision(&r, &x);
    }
    let b = Instant::now();
    println!("{:?}", (tmp, (b - a).as_micros()));
}

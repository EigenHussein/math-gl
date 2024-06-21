use freetype::Library;
use math_gl_core::{
    glbuffers::GLAttrib,
    shaders::{BasicShader, ShaderCreator, ShaderProgram},
    texts_utility::{FreeTypeGenerator, Texts},
    texture_utility::VirtualScreen,
};
use nalgebra_glm::Vec3;

use gl;
use glfw::{Action, Context, WindowEvent};
use math_gl_core::{
    scene::GLWindow,
    shaders::{Camera, CameraProjection, CameraView},
};
use serde::{de::Visitor, ser::SerializeStruct, Deserialize, Serialize, Serializer};

trait Sensor {
    fn get_sensor_value(&self) -> String;
}

struct SensorA;
struct SensorB;
struct SensorC;

impl Sensor for SensorA {
    fn get_sensor_value(&self) -> String {
        return "SensorA".to_string();
    }
}
impl Sensor for SensorB {
    fn get_sensor_value(&self) -> String {
        return "SensorB".to_string();
    }
}
impl Sensor for SensorC {
    fn get_sensor_value(&self) -> String {
        return "SensorC".to_string();
    }
}

fn get_sensor(name: &str) -> Box<dyn Sensor> {
    match name {
        "A" => Box::new(SensorA),
        "B" => Box::new(SensorB),
        _ => Box::new(SensorC),
    }
}

impl Serialize for Box<dyn Sensor> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tmp = serializer.serialize_struct("Sensor", 2).unwrap();
        tmp.serialize_field("SensorID", &self.get_sensor_value());
        return tmp.end();
    }
}

struct VisitorSensor;

impl<'de> Visitor<'de> for VisitorSensor {
    type Value = Box<dyn Sensor>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting Sensors {A,B,C}\n")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let sensor_id: &str = map.next_value().unwrap();
        // Some(match sensor_id {
        //     "SensorA" => Box::new(SensorA),
        //     "SensorB" => Box::new(SensorB),
        //     _ => Box::new(SensorC),
        // })
        if sensor_id == "SensorA" {
            return Ok(Box::new(SensorA));
        }
        if sensor_id == "SensorB" {
            return Ok(Box::new(SensorB));
        } else {
            return Ok(Box::new(SensorC));
        }
    }
}

impl<'de> Deserialize<'de> for Box<dyn Sensor> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["SensorID"];
        deserializer.deserialize_struct("Sensor", FIELDS, VisitorSensor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct test1 {
    a: String,
    b: i32,
}

fn main2() {
    let mut sensors: Vec<Box<dyn Sensor>> = Vec::new();
    sensors.push(get_sensor("A"));
    sensors.push(get_sensor("B"));
    sensors.push(get_sensor("C"));
    sensors.push(get_sensor("B"));
    sensors.push(get_sensor("C"));

    for sensor in sensors.iter_mut() {
        println!("{:?}", sensor.get_sensor_value());
    }

    println!(
        "{}",
        serde_json::to_string(&test1 {
            a: "Hello".to_string(),
            b: 10
        }).unwrap()
    );
    let kkk = serde_json::to_string(&sensors).unwrap();
    println!("{}", serde_json::to_string(&sensors).unwrap());
    let mut sens: Vec<Box<dyn Sensor>> = serde_json::from_str(kkk.as_str()).unwrap();
}

fn main() {
    let mut window = GLWindow::new_configured(1024, 1024);

    let vertices = [
        (Vec3::new(0.0, 0.0, 0.0), Vec3::new(5.0, 0.0, 0.0)),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 10.0, 0.0)),
        (Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 10.0)),
    ];

    let shader = ShaderCreator::new(math_gl_core::shaders::ColorMode::ColorPerVertex, false);
    let mut program = BasicShader::new(shader.program_id);
    program
        .buffer
        .allocateBuffer(vertices.as_ptr() as _, 3 * 2 * 3 * 4)
        .setAttributes(&[GLAttrib::new(0, 3, 6, 0), GLAttrib::new(1, 3, 6, 3)].to_vec());

    let mut cam = Camera::new(
        CameraProjection::default_perspective(1.0, 1.5),
        CameraView::default(),
    );

    cam.add_program(program.get_program()).update_uniforms();

    // let mut virt = VirtualScreen::new(1024, 1024);
    window.m_glfw.set_swap_interval(glfw::SwapInterval::Sync(2));

    // virt.frame_buffer.bind();

    while !window.m_window.should_close() {
        window.m_glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window.m_receiver) {
            update_camera(&event, &mut cam);
        }

        unsafe {
            gl::UseProgram(program.get_program());
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.m_window.swap_buffers();
    }
}

fn update_camera(ev: &WindowEvent, camera_instance: &mut Camera) {
    match ev {
        WindowEvent::Key(key, scancode, action, modifier) => {
            let act = *action;
            if *action == Action::Repeat || *action == Action::Press {
                match *key {
                    glfw::Key::A => {
                        camera_instance
                            .get_camera_view()
                            .increase_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::D => {
                        camera_instance
                            .get_camera_view()
                            .decrease_azimuth(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::W => {
                        camera_instance
                            .get_camera_view()
                            .increase_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }
                    glfw::Key::S => {
                        camera_instance
                            .get_camera_view()
                            .decrease_altitude(0.02)
                            .update_matrix();
                        camera_instance.update_uniforms();
                    }

                    _ => {}
                };
            }
        }
        _ => {}
    };
}

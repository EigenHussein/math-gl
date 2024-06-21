use assimp;
use math_gl_core::shaders::{BasicShader, ShaderProgram};
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
pub struct Instanced {
    program_id: u32,
}

impl ShaderProgram for Instanced {
    fn clean_draw<T>(
        &self,
        mode: Option<u32>,
        offset: Option<i32>,
        count: Option<i32>,
        draw_costume_call: Option<T>,
    ) where
        T: Fn() -> (),
    {
    }
    fn draw<T>(
        &self,
        mode: Option<u32>,
        offset: Option<i32>,
        count: Option<i32>,
        draw_costume_call: Option<T>,
    ) where
        T: Fn() -> (),
    {
    }
    fn get_program(&self) -> u32 {
        self.program_id
    }
}

// impl Copy for (Vec3,Vec3,Vec2){
// }
// #[derive(Clone, Copy)]

pub struct Mesh {
    pub vertices: Vec<(Vec3, Vec3, Vec2)>,
}
impl Mesh {
    pub fn mesh_from_ai_mesh(scene_mesh: assimp::Mesh) -> Mesh {
        let mut mesh_data: Vec<(Vec3, Vec3, Vec2)> =
            Vec::with_capacity(scene_mesh.num_vertices as _);
        unsafe {
            for vertex_id in 0..scene_mesh.num_vertices {
                mesh_data.push((
                    *(scene_mesh.vertices.add(vertex_id as _) as *const Vec3),
                    *(scene_mesh.normals.add(vertex_id as _) as *const Vec3),
                    *(scene_mesh.texture_coords[0].add(vertex_id as _) as *const Vec2),
                ));
            }
        }
        Mesh {
            vertices: mesh_data,
        }
    }

    pub fn model_loader(filename: &str) -> Vec<Mesh> {
        let mut importer = assimp::Importer::new();
        importer.triangulate(true);
        // importer.ac_eval_subdivision(true);
        // importer.ac_separate_bf_cull(true);
        // importer.ifc_skip_curve_representations(true);
        // importer.ifc_skip_space_representations(true);
        // importer.join_identical_vertices(true);
        let scene = importer.read_file(filename).unwrap();
        let mut meshes: Vec<Mesh> = Vec::with_capacity(scene.num_meshes as _);

        for mesh_id in 0..scene.num_meshes {
            meshes.push(Self::mesh_from_ai_mesh(scene.mesh(mesh_id as _).unwrap()));
        }

        meshes
        // let mut mesh = unsafe { &**sphere_model.meshes };
    }
    pub fn load_from_model(filename: &str) -> Vec<Self> {
        Self::model_loader(filename)
    }
}

pub struct Bonds {
    atoms_matrices: Vec<Mat4>,
    bonds_matrices: Vec<Mat4>,
}

impl Bonds {
    pub fn new_default() -> Self {
        Self {
            atoms_matrices: Vec::new(),
            bonds_matrices: Vec::new(),
        }
    }

    pub fn generate_atom_matrix(location: Vec3, radius: f32) -> Mat4 {
        let mut model_matrix = Mat4::identity();
        model_matrix[0] *= radius;
        model_matrix[5] *= radius;
        model_matrix[10] *= radius;
        nalgebra_glm::translation(&location) * model_matrix // nalgebra_glm::translate(&model_matrix, &location)
    }
    pub fn generate_bond_matrix(start_point: Vec3, end_point: Vec3, thickness: f32) -> Mat4 {
        let direction = end_point - start_point;
        let length = nalgebra_glm::length(&direction);

        let mut model_matrix = Mat4::identity();
        model_matrix[0] *= length;
        model_matrix[5] *= thickness;
        model_matrix[10] *= thickness;

        let rotation_axis =
            -1.0 * nalgebra_glm::normalize(&nalgebra_glm::cross(&direction, &Vec3::x()));
        let angle = nalgebra_glm::angle(&Vec3::x(), &direction);

        model_matrix = nalgebra_glm::rotation(angle, &rotation_axis) * model_matrix;
        model_matrix[12] = start_point.x;
        model_matrix[13] = start_point.y;
        model_matrix[14] = start_point.z;
        model_matrix[15] = 1.0;
        model_matrix
    }
}

pub mod simple_lattice {
    use std::ops::Add;

    use nalgebra_glm::Mat4;

    pub fn generate_simple_lattice(A: f32, atoms_per_size: u32) -> (Vec<Mat4>, Vec<Mat4>) {
        let mut atoms: Vec<Mat4> =
            Vec::with_capacity((atoms_per_size * atoms_per_size * atoms_per_size) as usize);
        let mut bonds: Vec<Mat4> = Vec::with_capacity((3 * atoms_per_size.pow(3)) as usize);
        let half = (atoms_per_size / 2) as i32;
        for x in -half / 2..half / 2 {
            for y in -half / 2..half / 2 {
                for z in -half / 2..half / 2 {
                    let current_pos = A * super::Vec3::new(x as f32, y as f32, z as f32);

                    let next_pos_x = A * super::Vec3::new(x.add(1) as f32, y as f32, z as f32);
                    let next_pos_y = A * super::Vec3::new(x as f32, y.add(1) as f32, z as f32);
                    let next_pos_z = A * super::Vec3::new(x as f32, y as f32, z.add(1) as f32);

                    atoms.push(super::Bonds::generate_atom_matrix(
                        A * super::Vec3::new(x as f32, y as f32, z as f32),
                        A / 5.0,
                    ));

                    bonds.push(super::Bonds::generate_bond_matrix(
                        current_pos,
                        next_pos_x,
                        A / 20.0,
                    ));
                    bonds.push(super::Bonds::generate_bond_matrix(
                        current_pos,
                        next_pos_y,
                        A / 20.0,
                    ));
                    bonds.push(super::Bonds::generate_bond_matrix(
                        current_pos,
                        next_pos_z,
                        A / 20.0,
                    ));
                }
            }
        }

        (atoms, bonds)
    }
}

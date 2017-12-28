use {ARENA_HEIGHT,ARENA_WIDTH};

use amethyst::assets::Loader;
use amethyst::ecs::World;
use amethyst::renderer::{Projection, Camera, Material, MeshHandle, PosTex};
use amethyst::core::components::Transform;
use cgmath::{Matrix4, Vector3};

pub fn initialise_camera(world: &mut World) {
    world
    .create_entity()
    .with(Camera::from(Projection::orthographic(
        0.0,
        ARENA_WIDTH,
        ARENA_HEIGHT,
        0.0,
    )))
    .with(Transform(
        Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
    ))
    .build();
}

pub fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

pub fn create_colour_material(world: &World, colour: [f32; 4]) -> Material {
    use amethyst::renderer::MaterialDefaults;

    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

pub fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
    PosTex {
        position: [left, bottom, 0.],
        tex_coord: [0.0, 0.0],
    },
    PosTex {
        position: [right, bottom, 0.0],
        tex_coord: [1.0, 0.0],
    },
    PosTex {
        position: [left, top, 0.0],
        tex_coord: [1.0, 1.0],
    },
    PosTex {
        position: [right, top, 0.],
        tex_coord: [1.0, 1.0],
    },
    PosTex {
        position: [left, top, 0.],
        tex_coord: [0.0, 1.0],
    },
    PosTex {
        position: [right, bottom, 0.0],
        tex_coord: [0.0, 0.0],
    },
    ]
}

/// Generates vertices for a circle. The circle will be made of `resolution`
/// triangles.
pub fn generate_circle_vertices(radius: f32, resolution: usize) -> Vec<PosTex> {
    use std::f32::consts::PI;

    let mut vertices = Vec::with_capacity(resolution * 3);
    let angle_offset = 2.0 * PI / resolution as f32;

    // Helper function to generate the vertex at the specified angle.
    let generate_vertex = |angle: f32| {
        let x = angle.cos();
        let y = angle.sin();
        PosTex {
            position: [x * radius, y * radius, 0.0],
            tex_coord: [x, y],
        }
    };

    for index in 0..resolution {
        vertices.push(PosTex {
            position: [0.0, 0.0, 0.0],
            tex_coord: [0.0, 0.0],
        });

        vertices.push(generate_vertex(angle_offset * index as f32));
        vertices.push(generate_vertex(angle_offset * (index + 1) as f32));
    }

    vertices
}

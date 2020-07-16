use voxel_dag::octree::Octree;
use glam::*;

use luminance::context::GraphicsContext;
use luminance::tess::{Mode, TessBuilder, Tess, TessError};
use crate::rasterizer::{
    Vertex,
    VertexIndex,
    VertexPosition,
};

pub struct RenderMesh {
    pub tess: Tess,
    pub triangulated: bool,

    pub vert_count: usize,
}

impl RenderMesh {
    pub fn new<C>(ctx: &mut C, triangulated: bool, vertices: Vec<Vertex>, indices: Vec<VertexIndex>) -> Result<RenderMesh, TessError>
    where
        C: GraphicsContext,
    {
        let vert_count = vertices.len();
        println!("Vertices: {}", vert_count);
        let tess = TessBuilder::new(ctx)
            .set_mode(Mode::Triangle)
            .add_vertices(vertices)
            .set_indices(indices)
            .build()?;
        Ok(RenderMesh {
            tess: tess,
            triangulated: triangulated,

            vert_count: vert_count,
        })
    }

    pub fn from_tess(tess: Tess, triangulated: bool, vert_count: usize) -> RenderMesh {
        RenderMesh {
            tess: tess,
            triangulated: triangulated,
            vert_count: vert_count,
        }
    }

    pub fn TRIANGLE<C>(ctx: &mut C) -> Result<RenderMesh, TessError>
    where
        C: GraphicsContext,
    {
        let tess = TessBuilder::new(ctx)
            .set_mode(Mode::Triangle)
            .add_vertices(TRIANGLE_VERTICES)
            .build()?;
        Ok(RenderMesh {
            tess: tess,
            triangulated: true,

            vert_count: 3,
        })
    }

    pub fn from_vox_data<C>(ctx: &mut C, vox_data: &[u8], data_size: (u32, u32, u32)) -> Result<RenderMesh, TessError>
    where
        C: GraphicsContext,
    {
        let mut builder = TessBuilder::new(ctx)
            .set_mode(Mode::Line);

        let mut vertices = Vec::new();
        // vertices.append(&mut get_cube_lines(Vec3::new(0.0, 0.0, 0.0)));
        // vertices.append(&mut get_cube_lines(Vec3::new(0.0, 1.0, 0.0)));

        let mut vox_count = 0;

        for x in 0..data_size.0 {
            for y in 0..data_size.1 {
                for z in 0..data_size.2 {
                    if vox_data[(x + y * data_size.0 + z * data_size.0 * data_size.1) as usize] > 0 {
                        vertices.append(&mut get_cube_lines(Vec3::new(x as f32,y as f32,z as f32) - Vec3::new(63.0, 63.0, 63.0), 1.0));
                        vox_count += 1;
                    }
                }
            }
        }

        builder = builder.add_vertices(vertices);

        debug!("Voxels: {}", vox_count);

        Ok(RenderMesh {
            tess: builder.build()?,
            triangulated: true,
            vert_count: 0,
        })
    }

    pub fn from_octree<C>(ctx: &mut C, octree: &Octree, size: f32) -> Result<RenderMesh, TessError>
    where
        C: GraphicsContext,
    {
        let mut builder = TessBuilder::new(ctx)
            .set_mode(Mode::Line);

        let mut vertices = Vec::new();

        for octant in &octree.octants {
            let octant_size = size / 2.0f32.powi(octant.level as i32);
            let position = Vec3::new(octant.position.x(), octant.position.y(), octant.position.z());
            vertices.append(&mut get_cube_lines(position * size - Vec3::new(size / 2.0,  size / 2.0, size / 2.0), octant_size));
        }

        builder = builder.add_vertices(vertices);

        Ok(RenderMesh {
            tess: builder.build()?,
            triangulated: true,
            vert_count: 0,
        })
    }
}

const TRIANGLE_VERTICES: [Vertex; 3] = [
    Vertex {
        position: VertexPosition::new([-0.5, -0.5, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0.5, -0.5, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0., 0.5, 0.0]),
    },
];

fn get_cube_lines(pos: Vec3, scale: f32) -> Vec<Vertex> {
    vec![//X
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y(), pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y(), pos.z()]),
    },

    Vertex {
        position: VertexPosition::new([pos.x(), pos.y()+scale, pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y()+scale, pos.z()]),
    },

    Vertex {
        position: VertexPosition::new([pos.x(), pos.y(), pos.z()+scale]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y(), pos.z()+scale]),
    },

    Vertex {
        position: VertexPosition::new([pos.x(), pos.y()+scale, pos.z()+scale]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y()+scale, pos.z()+scale]),
    },

    //Y
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y(), pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y()+scale, pos.z()]),
    },

    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y(), pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y()+scale, pos.z()]),
    },

    Vertex {
        position: VertexPosition::new([pos.x(), pos.y(), pos.z()+scale]),
    },
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y()+scale, pos.z()+scale]),
    },

    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y(), pos.z()+scale]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y()+scale, pos.z()+scale]),
    },

    //Z
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y(), pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y(), pos.z()+scale]),
    },

    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y(), pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y(), pos.z()+scale]),
    },

    Vertex {
        position: VertexPosition::new([pos.x(), pos.y()+scale, pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x(), pos.y()+scale, pos.z()+scale]),
    },

    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y()+scale, pos.z()]),
    },
    Vertex {
        position: VertexPosition::new([pos.x()+scale, pos.y()+scale, pos.z()+scale]),
    },
    ]
}

/*
const CUBE_LINES: [Vertex; 24] = [
    //X
    Vertex {
        position: VertexPosition::new([0.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 0.0, 0.0]),
    },

    Vertex {
        position: VertexPosition::new([0.0, 1.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 1.0, 0.0]),
    },

    Vertex {
        position: VertexPosition::new([0.0, 0.0, 1.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 0.0, 1.0]),
    },

    Vertex {
        position: VertexPosition::new([0.0, 1.0, 1.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 1.0, 1.0]),
    },

    //Y
    Vertex {
        position: VertexPosition::new([0.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0.0, 1.0, 0.0]),
    },

    Vertex {
        position: VertexPosition::new([1.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 1.0, 0.0]),
    },

    Vertex {
        position: VertexPosition::new([0.0, 0.0, 1.0]),
    },
    Vertex {
        position: VertexPosition::new([0.0, 1.0, 1.0]),
    },

    Vertex {
        position: VertexPosition::new([1.0, 0.0, 1.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 1.0, 1.0]),
    },

    //Z
    Vertex {
        position: VertexPosition::new([0.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0.0, 0.0, 1.0]),
    },

    Vertex {
        position: VertexPosition::new([1.0, 0.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 0.0, 1.0]),
    },

    Vertex {
        position: VertexPosition::new([0.0, 1.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([0.0, 1.0, 1.0]),
    },

    Vertex {
        position: VertexPosition::new([1.0, 1.0, 0.0]),
    },
    Vertex {
        position: VertexPosition::new([1.0, 1.0, 1.0]),
    },
];
*/

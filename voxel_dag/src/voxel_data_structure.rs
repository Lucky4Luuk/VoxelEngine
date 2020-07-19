use glam::*;
use serde::{Serialize, Deserialize};

use std::time::Instant;

//Data structure:
// childmask        [24 bits empty; 8 bits for mask]
// child index 1    [u32]
//Explanation:
// childmask        Specifies if said child is a leaf
// child index 1    Index of first non-empty child

fn node_contains_geometry(position: Vec3, size: usize, data: &[u8], data_size: (u32,u32,u32)) -> bool {
    if position.x() < 0.0 || position.y() < 0.0 || position.z() < 0.0 { return false; }
    if position.x() >= data_size.0 as f32 || position.y() >= data_size.1 as f32 || position.z() >= data_size.2 as f32 { return false; }
    for px in position.x() as usize .. position.x() as usize + size {
        for py in position.y() as usize .. position.y() as usize + size {
            for pz in position.z() as usize .. position.z() as usize + size {
                if data[px as usize + py as usize * data_size.0 as usize + pz as usize * data_size.0 as usize * data_size.1 as usize] > 0 {
                    return true;
                }
            }
        }
    }
    false
}

//Struct used internally
struct Octant {
    pub parent: u32,
    pub first_child: u32,
    pub level: u32,
    pub is_leaf: bool,
    pub position: Vec3,
    pub size: usize,
}

#[derive(Serialize, Deserialize)]
pub struct VoxelDAG {
    data: Vec<u32>, //raw data, only used internally for OpenGL
}

//Voxels (on 1 axis) per level of the octree: pow(2, level)
//Required length for size: log2(biggest_voxel_count_axis)//.ceil() to get the proper level
//TODO: Use a proper error type, not a String.
impl VoxelDAG {
    pub fn from_voxel_data(data: &[u8], data_size: (u32, u32, u32), level: u32, octree_size: usize) -> Result<Self, String> {
        if level < 1 { return Err("Unable to create octree with level lower than 1!".to_string()); }

        let mut nodes = vec![Octant {
            parent: 0,
            first_child: 0,
            level: 0,
            is_leaf: false,
            position: Vec3::new(0.0, 0.0, 0.0),
            size: octree_size,
        }];

        let mut idx = 0;
        'gen: loop {
            let node = &nodes[idx];
            //Check if node contains geometry, then generate children
            if node_contains_geometry(node.position, node.size, data, data_size) {
                for i in 0..8 {
                    
                }
            } else {
                nodes[idx].is_leaf = true;
            }

            idx += 1;

            if idx >= nodes.len() {
                break 'gen;
            }
        }

        unimplemented!();
    }
}

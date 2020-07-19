use glam::*;
use serde::{Serialize, Deserialize};

use std::time::Instant;
use std::cmp;

//Data structure:
// childmask        [24 bits empty; 8 bits for mask]
// child index 1    [u32]
//Explanation:
// childmask        Specifies if said child is a leaf
// child index 1    Index of first non-empty child

fn node_contains_geometry(position: Vec3, level: u32, data: &[u8], data_size: (u32,u32,u32)) -> bool {
    let mut biggest_axis_size = data_size.0;
    if data_size.1 > biggest_axis_size { biggest_axis_size = data_size.1; }
    if data_size.2 > biggest_axis_size { biggest_axis_size = data_size.2; }

    let node_size = biggest_axis_size as f32 / 2.0f32.powi(level as i32);
    let vox_space_pos = (position * biggest_axis_size as f32).floor();

    let ux = vox_space_pos.x() as usize;
    let uy = vox_space_pos.y() as usize;
    let uz = vox_space_pos.z() as usize;
    let us = node_size as usize;

    for px in ux .. ux + us {
        for py in uy .. uy + us {
            for pz in uz .. uz + us {
                if data[px as usize + py as usize * data_size.0 as usize + pz as usize * data_size.0 as usize * data_size.1 as usize] > 0 {
                    return true;
                }
            }
        }
    }
    false
}

//Struct used internally
pub struct Octant {
    pub parent: u32,
    pub first_child: u32,
    pub level: u32,
    pub is_leaf: bool,
    pub position: Vec3,
}

fn generate_node(nodes: &mut Vec<Octant>, data: &[u8], data_size: (u32,u32,u32), parent: u32, cur_level: u32, level: u32) {
    let node = &nodes[parent as usize];
    //Check if node contains geometry, then generate children
    if cur_level >= level {
        println!("Reached max level!");
        return;
    }
    if node_contains_geometry(node.position, node.level, data, data_size) {
        let mut children = Vec::new();
        let parent_pos = nodes[parent as usize].position;
        for j in 0..8 {
            let child_x = (j % 2) as f32 * 0.5;
            let child_y = (j / 2 % 2) as f32 * 0.5;
            let child_z = (j / 4 % 2) as f32 * 0.5;
            let child_pos = Vec3::new(child_x, child_y, child_z);
            let new_child_pos = parent_pos + child_pos / 2.0f32.powi(cur_level as i32); //Could possibly lead to loss of information, but the level probably won't go past 2 bilion lol
            let child_idx = nodes.len() as u32;
            if j == 0 { nodes[parent as usize].first_child = child_idx; }

            //We have the child, let's now generate it's children
            nodes.push(Octant {
                parent: parent,
                first_child: 0,
                level: cur_level + 1,
                is_leaf: false,
                position: new_child_pos,
            });
            children.push(child_idx);
        }
        for child in children {
            //Call this function again
            generate_node(nodes, data, data_size, child, cur_level + 1, level);
        }
    } else {
        nodes[parent as usize].is_leaf = true;
        return;
    }
}

#[derive(Serialize, Deserialize)]
pub struct VoxelDAG {
    data: Vec<u32>, //raw data, only used internally for OpenGL
}

//Voxels (on 1 axis) per level of the octree: pow(2, level)
//Required length for size: log2(biggest_voxel_count_axis)//.ceil() to get the proper level
//TODO: Use a proper error type, not a String.
impl VoxelDAG {
    pub fn from_voxel_data(data: &[u8], data_size: (u32, u32, u32), level: u32) -> Vec<Octant> { //Result<Self, String>
        // if level < 1 { return Err("Unable to create octree with level lower than 1!".to_string()); }

        let mut nodes = vec![Octant {
            parent: 0,
            first_child: 0,
            level: 0,
            is_leaf: false,
            position: Vec3::new(0.0, 0.0, 0.0),
        }];

        generate_node(&mut nodes, data, data_size, 0, 0, level);

        debug!("Node count: {}", nodes.len());

        // unimplemented!();
        nodes
    }
}

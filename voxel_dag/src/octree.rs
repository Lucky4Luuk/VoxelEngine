use std::rc::Rc;
use std::cmp;

use glam::*;

//Octree implementation for constructing a DAG
//Not the most optimized implementation, but I don't care too much about memory usage, as this
//will only be used as an intermediate stage for constructing DAGs, and we do not need to
//generate the entire octree at once, we can simply generate a part of it, then turn it to a DAG,
//generate more, create a DAG based off that and merge it with the main one, etc

//Vec3 takes 12 bytes but has no SIMD acceleration, Vec3A takes 16 bytes but has SIMD acceleration.
//Please help, I don't know which trade-off is better

pub struct Octant {
    pub children: [Option<usize>; 8], //usize as an index into the octree
    pub level: u32,
    pub is_leaf: bool,
    pub position: Vec3A, //Top left, 16 bytes instead of 12 but might be worth the trade off
}

impl std::fmt::Debug for Octant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Octant")
         .field("level", &self.level)
         .finish()
    }
}

pub struct Octree<'a> {
    pub level: u32,
    pub current_level: (u32, Vec<usize>), //(current level, indices of nodes in vector)
    pub level_indices: Vec<usize>,
    pub octants: Vec<Octant>,
    voxel_data: &'a [u8],
    data_size: (u32, u32, u32)
}

//Voxels (on 1 axis) per level of the octree: pow(2, level)
//Required length for size: log2(biggest_voxel_count_axis)//.ceil() to get the proper level
impl<'a> Octree<'a> {
    pub fn from_voxel_data(data: &'a [u8], data_size: (u32, u32, u32), level: u32) -> Result<Self, String> {
        if level < 2 { return Err(format!("Unable to create octree with level {}", level)); }

        //x + y * x_size + z * x_size * y_size
        let octants = vec![Octant {
            children: [None; 8],
            level: 0,
            is_leaf: false,
            position: Vec3A::new(0.0, 0.0, 0.0),
        }];

        Ok(Self {
            level: level,
            current_level: (0, vec![0]),
            level_indices: vec![0],
            octants: octants,
            voxel_data: data,
            data_size: data_size,
        })
    }
    //Prints raw data to console
    pub fn debug_print(&self) {
        for i in 0..cmp::min(self.octants.len(), 32) {
            debug!("Octant: {:?} - Index: {}", self.octants[i], i);
        }
    }

    /// Generates a single level of the octree
    pub fn generate_level(&mut self) {
        let mut next_level = Vec::new();
        let mut first_child_idx = usize::MAX;
        for idx in &self.current_level.1 {
            //Check if node contains geometry
            if !self.octants[*idx].is_leaf {
                if !self.check_empty(*idx) {
                    //Node is not a leaf and is also not empty
                    //This means its ready to be explored for the next level
                    //We need to generate children for the current node and add those to the next_level vec
                    let parent_pos = self.octants[*idx].position;
                    for j in 0..8 {
                        let child_x = (j % 2) as f32 * 0.5;
                        let child_y = (j / 2 % 2) as f32 * 0.5;
                        let child_z = (j / 4 % 2) as f32 * 0.5;
                        let child_pos = Vec3A::new(child_x, child_y, child_z);
                        let new_child_pos = parent_pos + child_pos / 2.0f32.powi(self.octants[*idx].level as i32); //Might need to do +1
                        let child_idx = self.octants.len();
                        if child_idx < first_child_idx { first_child_idx = child_idx; }
                        self.octants.push(Octant {
                            children: [None; 8],
                            level: self.octants[*idx].level + 1,
                            is_leaf: false,
                            position: new_child_pos,
                        });
                        self.octants[*idx].children[j] = Some(child_idx);
                        next_level.push(child_idx);
                    }
                } else {
                    self.octants[*idx].is_leaf = true;
                }
            }
        }
        trace!("Found {} nodes containing geometry to process for the next level!", next_level.len());
        self.current_level = (self.current_level.0 + 1, next_level);
    }

    fn check_empty(&self, idx: usize) -> bool {
        let mut biggest_axis_size = self.data_size.0;
        if self.data_size.1 > biggest_axis_size { biggest_axis_size = self.data_size.1; }
        if self.data_size.2 > biggest_axis_size { biggest_axis_size = self.data_size.2; }

        let node = &self.octants[idx];
        let pos = node.position;
        let node_size = biggest_axis_size as f32 / 2.0f32.powi(node.level as i32);
        let vox_space_pos = (pos * biggest_axis_size as f32).floor();

        trace!("top_left: {}", vox_space_pos.x() as usize);
        trace!("size:     {}", node_size as usize);

        let ux = vox_space_pos.x() as usize;
        let uy = vox_space_pos.y() as usize;
        let uz = vox_space_pos.z() as usize;
        let us = node_size as usize;

        for x in ux .. ux + us {
            for y in uy .. uy + us {
                for z in uz .. uz + us {
                    if self.voxel_data[x + y * self.data_size.0 as usize + z * self.data_size.0 as usize * self.data_size.1 as usize] > 0 {
                        trace!("Found a voxel containing geometry!");
                        return false;
                    }
                }
            }
        }

        true
    }
}

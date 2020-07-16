use std::rc::Rc;
use glam::*;

//Octree implementation for constructing a DAG
//Not the most optimized implementation, but I don't care too much about memory usage, as this
//will only be used as an intermediate stage for constructing DAGs, and we do not need to
//generate the entire octree at once, we can simply generate a part of it, then turn it to a DAG,
//generate more, create a DAG based off that and merge it with the main one, etc

//Vec3 takes 3 bytes but has no SIMD acceleration, Vec3A takes 4 bytes but has SIMD acceleration.
//Please help, I don't know which trade-off is better

pub struct Node {
    pub children: [Rc<Option<Node>>; 8],
    pub level: u32,
}

impl Node {
    pub fn get_leaf_nodes(&self) -> Vec<Node> {
        let mut result = Vec::new();

        //Check if node contains voxels (separate method)
        //If so, add node to results
        //If not, check if the node is not a leaf,

        result
    }
}

pub struct Octree {
    pub root: Node,
    pub level: (u32, u32), //(start, end)
}

/*
fn node_step<'a>(node_pos: Vec3A, node: &mut Node<'a>, data: &[u8], data_size: (u32, u32, u32)) -> Vec<(Node<'a>, Vec3A)> {
    let mut biggest_axis_size = data_size.0;
    if data_size.1 > biggest_axis_size { biggest_axis_size = data_size.1; }
    if data_size.2 > biggest_axis_size { biggest_axis_size = data_size.2; }

    //Scale to go from octree space (range [0; 1]) to vox space (range variable but needs to be treated as smallest octree size that wraps it)
    let vox_space_scale = 1.0;

    let mut non_empty_children = Vec::new();
    for j in 0..8 {
        //Node location within parent node (range [0; 1])
        let child_x = (j % 2) as f32 * 0.5;
        let child_y = (j / 2 % 2) as f32 * 0.5;
        let child_z = (j / 4 % 2) as f32 * 0.5;
        let child_pos = Vec3A::new(child_x, child_y, child_z);
        //Node location within full octree (range [0; 1])
        //Probably node_pos + child_pos * 10.0f32.pow(node.level)
        let new_child_pos = node_pos + child_pos * 10.0f32.powi(node.level as i32);
        //Now we need to check if this child actually contains any geometry, by checking the voxel data for any voxels
        let vox_space_pos = new_child_pos * vox_space_scale;
    }
    non_empty_children
}
*/

//Voxels (on 1 axis) per level of the octree: pow(2, level)
//Required length for size: log2(biggest_voxel_count_axis)//.ceil() to get the proper level
impl Octree {
    pub fn from_voxel_data(data: &[u8], data_size: (u32, u32, u32), level: (u32, u32)) -> Result<Self, String> {
        if level.1 - level.0 < 1 { return Err(format!("Unable to create octree with level {:?}", level)); }

        //x + y * x_size + z * x_size * y_size
        let mut root = Node {
            children: [Rc::new(None); 8],
            level: level.0,
        };

        Ok(Self {
            root: root,
            level: level,
        })
    }
}

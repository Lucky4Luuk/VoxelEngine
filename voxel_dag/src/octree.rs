use glam::*;

//Octree implementation for constructing a DAG
//Not the most optimized implementation, but I don't care too much about memory usage, as this
//will only be used as an intermediate stage for constructing DAGs, and we do not need to
//generate the entire octree at once, we can simply generate a part of it, then turn it to a DAG,
//generate more, create a DAG based off that and merge it with the main one, etc

pub struct Node<'a> {
    pub children: [Option<&'a Node<'a>>; 8],
    pub level: u32,
}

pub struct Octree<'a> {
    pub root: Node<'a>,
    pub level: (u32, u32), //(start, end)
}

//Voxels (on 1 axis) per level of the octree: pow(2, level)
//Required length for size: log2(biggest_voxel_count_axis)//.ceil() to get the proper level
impl Octree<'_> {
    pub fn from_voxel_data(data: &[u8], data_size: (u32, u32, u32), level: (u32, u32)) -> Result<Self, &str> {
        if level.1 - level.0 < 1 { return Err(format!("Unable to create octree with level {:?}", level)); }

        //x + y * x_size + z * x_size * y_size
        let mut root = Node {
            children: [None; 8],
            level: level.0,
        };

        for i in level.0..level.1 {
            let mut cur_node = &root;
            for j in 0..8 {
                //Check if node contains any geometry
            }
        }

        Ok(Self {
            root: root,
            level: level,
        })
    }
}

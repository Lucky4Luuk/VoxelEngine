use glam::*;
use serde::{Serialize, Deserialize};

use std::time::Instant;

use super::octree::Octree;

//In the original paper, it suggests storing only a pointer to the first child, and then
//have all the other children stored in memory consecutively, but I'm not sure how I'd do this
//in Rust and I have to send it to the GPU anyway, which I plan on doing through an SSBO.
//Therefore, I'm going to stick with an identical layout on the CPU, making use of indices
//instead of pointers.

//slice.as_ptr() as *const c_void
//slice.len() as isize

//Data structure:
// childmask        [24 bits empty; 8 bits for mask]
// child index 1    [usize]

//In the above data structure, we only need the first index, as we can simply check the childmask
//to see which children contain geometry, and so we only need to know where the first
//non-empty child is located, as they are stored consecutively in the buffer.
//This is essentially the same way the original paper did it, but we are essentially
//using a buffer as memory here, which is mostlikely less efficient, but
//it allows us to very easily upload this data to the GPU.

//NOTE: It should not be super hard to eventually switch this out to use memory directly,
//      but changing it now makes no sense. It's much easier to develop it like this as well,
//      so I'll leave it like this for now :)

#[derive(Serialize, Deserialize)]
pub struct DAG {
    data: Vec<u32>, //raw data, only used internally
}

impl DAG {
    //Creation
    pub fn from_voxel_data(data: &[u8], data_size: (u32, u32, u32)) -> Self {
        // unimplemented!();
        //First generate an Octree
        //then turn it into a DAG by removing all duplicate leafs.
        //We can save on memory by generating the octree only partially
        //before turning it into a DAG, then generate more parts.

        //For now, we won't bother with generating only partial Octrees.
        //I first want it to work before I optimize this :)
        let mut biggest_axis_size = data_size.0;
        if data_size.1 > biggest_axis_size { biggest_axis_size = data_size.1; }
        if data_size.2 > biggest_axis_size { biggest_axis_size = data_size.2; }

        let required_level = (biggest_axis_size as f32).log2().ceil() as u32;
        debug!("Required octree level: {}", required_level);
        let now = Instant::now();
        let mut octree = Octree::from_voxel_data(data, data_size, required_level).expect("Failed to create octree!");
        for _ in 0..required_level {
            octree.generate_level();
        }
        let duration = Instant::now() - now;
        debug!("Time to generate octree: {}ms", duration.as_millis());
        debug!("Octants: {}", octree.octants.len());

        debug!("Octree generated, ready to convert to DAG!");
        unimplemented!();
    }

    //Functions


    //Memory
    pub fn get_ptr(&self) -> *const u32 {
        self.data.as_ptr()
    }

    pub fn get_len(&self) -> usize {
        self.data.len()
    }
}

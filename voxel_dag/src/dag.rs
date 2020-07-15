use glam::*;
use serde::{Serialize, Deserialize};

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

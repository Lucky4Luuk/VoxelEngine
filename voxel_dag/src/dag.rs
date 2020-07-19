use glam::*;
use serde::{Serialize, Deserialize};

use std::time::Instant;

use super::octree::Octree;

//NOTE: See NOTE in src/octree.rs; This implementation is awful and only stays for cross referencing.

//In the original paper, it suggests storing only a pointer to the first child, and then
//have all the other children stored in memory consecutively, but I'm not sure how I'd do this
//in Rust and I have to send it to the GPU anyway, which I plan on doing through an SSBO.
//Therefore, I'm going to stick with an identical layout on the CPU, making use of indices
//instead of pointers.

//slice.as_ptr() as *const c_void
//slice.len() as isize

//Data structure:
// childmask        [24 bits empty; 8 bits for mask]
// child index 1    [u32] where 0 means this is a leaf node

//In the above data structure, we only need the first index, as we can simply check the childmask
//to see which children contain geometry, and so we only need to know where the first
//non-empty child is located, as they are stored consecutively in the buffer.
//This is essentially the same way the original paper did it, but we are essentially
//using a buffer as memory here, which is mostlikely less efficient, but
//it allows us to very easily upload this data to the GPU.

//NOTE: It should not be super hard to eventually switch this out to use memory directly,
//      but changing it now makes no sense. It's much easier to develop it like this as well,
//      so I'll leave it like this for now :)

//Nice wrapper around the data structure above, used as a nice interface
pub struct Node {
    pub childmask: u32,
    pub child: u32,
}

impl Node {
    pub fn new(childmask: u32, child: u32) -> Self {
        Self {
            childmask: childmask,
            child: child,
        }
    }

    pub fn get_raw(&self) -> (u32, u32) {
        (self.childmask, self.child)
    }
}

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

        //Generating the DAG
        let mut current_level = 0;
        let current_idx = 0;
        let mut unique_nodes = Vec::new();
        for octant in &octree.octants {
            if octant.level != current_level { current_level = octant.level; }
            if current_idx >= data.len() {
                //First node for this level
                //All indices ill be the wrong index to index into the data index
                //which needs to be fixed afterwards
                unique_nodes.push(octant);
            } else {
                //Not the first node for this level, so first check if
                //there are any duplicates already added to the data array
                let mut found_duplicate = false;
                'search: for node in &unique_nodes {
                    if node == &octant {
                        //Nodes match!
                        //Duplicate found, so we do not add it to the list
                        found_duplicate = true;
                        break 'search;
                    }
                }
                if found_duplicate {
                    unique_nodes.push(octant);
                }
            }
        }

        //Now we need to loop through our unique nodes and fix all the indices
        //NOTE: This is probably way more expensive than needed, but it's 3:28 AM right now
        //      so this is how I'll solve this problem for now.
        for node in &unique_nodes {
            //Basically, we need to get the original children nodes from the octree
            //and then go through the unique nodes and see where they ended up now.
            //We cannot simply go through our octree backwards originally to make sure children
            //have already been moved to a new location either, because then we still need to loop
            //through our unique nodes, although we might have to loop through less of them.
            /*
            for i in 0..8 {
                if let Some(old_idx) = node.children[i] {
                    let old_child = &octree.octants[old_idx as usize];
                    //Now that we have the old child, we need to find the new childs idx
                    //TODO: Find a way to only loop through nodes of the right level
                    'search: for i in 0..unique_nodes.len() {
                        if unique_nodes[i] == old_child {
                            //We found the new node that we should refer to
                            node.children[i] = Some(i as u32);
                            break 'search;
                        }
                    }
                }
            }
            */
        }

        let mut data = Vec::new();

        Self {
            data: data,
        }
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

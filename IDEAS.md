# Research / ideas

## CPU -> GPU
Send voxel data from CPU to GPU using Uniform Buffer Objects.

## Data structure
* [DAG](http://www.cse.chalmers.se/~uffe/HighResolutionSparseVoxelDAGs.pdf)
* [Modifying said dag](https://graphics.tudelft.nl/Publications-new/2020/CBE20/ModifyingCompressedVoxels-main.pdf)

## GPU Data structure
The DAG will not store pointers, but in fact store the location in the buffer where its first child is located.
This should be the only modification needed to what the paper describes to get it to work nicely with my opengl code.

## CPU Data structure
I'm still quite unsure about this, but it preferably has to mimic the GPU data structure as much as possible, so it's much easier to get the data to the GPU.

## Raytracing
The raytracing will most likely be done in a compute shader.
I'd like to spawn a thread per ray (see [this](https://research.nvidia.com/sites/default/files/pubs/2013-07_Megakernels-Considered-Harmful/laine2013hpg_paper.pdf)), but this will probably not be implemented at the start.

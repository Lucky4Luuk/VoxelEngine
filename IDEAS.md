# Research / ideas

## CPU -> GPU
Send voxel data from CPU to GPU using Uniform Buffer Objects.

## Data structure
* [Sparse voxel DAG](http://www.cse.chalmers.se/~uffe/HighResolutionSparseVoxelDAGs.pdf)
* [Modifying said SVDAG](https://graphics.tudelft.nl/Publications-new/2020/CBE20/ModifyingCompressedVoxels-main.pdf)

## GPU Data structure
The DAG will not store pointers, but in fact store the location in the buffer where its first child is located.
This should be the only modification needed to what the paper describes to get it to work nicely with my OpenGL code.
It will be stored in a [SSBO](https://www.khronos.org/opengl/wiki/Shader_Storage_Buffer_Object), as this is most likely the most efficient way to store it.

## CPU Data structure
I'm still quite unsure about this, but it preferably has to mimic the GPU data structure as much as possible, so it's much easier to get the data to the GPU.

## Raytracing
The raytracing will most likely be done in a compute shader.
I'd like to spawn a thread per ray (see [this paper](https://research.nvidia.com/sites/default/files/pubs/2013-07_Megakernels-Considered-Harmful/laine2013hpg_paper.pdf)), but this will probably not be implemented at the start.
About octree raytracing, [here's what I'll probably use for the basic implementation](https://daeken.svbtle.com/a-stupidly-simple-fast-octree-traversal-for-ray-intersection).

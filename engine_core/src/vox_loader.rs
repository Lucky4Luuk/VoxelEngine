/// If this project goes anywhere, this should definitely be moved to a VFS or something

use glam::*;

pub fn load_vox(filename: &str) -> [u8; 126*126*126] { //1d array of voxels
    use dot_vox::load;
    let vox_data = load(filename).expect("Failed to open file!"); //No error checking because chad testing code
    //Lets only use the first model for now
    let model = &vox_data.models[0];
    let mut voxels: [u8; 126*126*126] = [0; 126*126*126];

    for voxel in &model.voxels {
        // let pos = Vec3A::new(voxel.x as f32, voxel.y as f32, voxel.z as f32);
        // let mat_id = voxel.i;
        voxels[voxel.x as usize + voxel.y as usize * 64 + voxel.z as usize * 64 * 64] = voxel.i;
    }

    voxels
}

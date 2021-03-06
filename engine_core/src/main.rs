#![feature(box_syntax)]

#[macro_use] extern crate log;
#[macro_use] extern crate imgui;

use std::ops::Deref;
use std::os::raw::c_void;
use std::time::Instant;

use luminance_sdl2::SDL2Surface;

use sdl2::event::Event;

use glow::HasContext;

use voxel_dag::*;

mod camera;
mod shader;
mod mesh;
mod compute;

mod ui;
mod vox_loader;
mod rasterizer;

pub fn initialize(width: u32, height: u32) -> Result<(SDL2Surface, glow::Context, sdl2::video::GLContext), &'static str> {
    let surface = SDL2Surface::new(
        (4, 5), //Opengl version
        "voxel stuff x)",
        (width, height),
        false
    );

    match surface {
        Err(e) => {
            error!("Couldn't open window!\n{}", e);
            return Err("Couldn't open window!")
        },
        Ok(surface) => {
            let gl_context = surface.window.gl_create_context().expect("Couldn't create GL context");
            let gl = glow::Context::from_loader_function(|s| {
                    surface.video.gl_get_proc_address(s) as *const c_void
                });
            debug!("Window opened and opengl initialized!");
            return Ok((surface, gl, gl_context));
        }
    }
}

fn main() {
    // let level_filter = log::LevelFilter::max();
    let level_filter = log::LevelFilter::Debug;

    pretty_env_logger::formatted_builder()
        .filter_level(level_filter)
        .init();

    debug!("Hello, world!");

    let vox_data = vox_loader::load_vox("teapot.vox");
    debug!("Vox data loaded!");
    // let dag = dag::DAG::from_voxel_data(&vox_data[..], (126, 126, 126));
    // let mut octree = octree::Octree::from_voxel_data(&vox_data[..], (126, 126, 126), 2).expect("Failed to create octree!");
    // octree.generate_level();
    // octree.generate_level();
    // octree.generate_level();
    // octree.generate_level();
    // octree.generate_level();
    // octree.generate_level();
    // debug!("Old octree node count: {}", octree.octants.len());
    // octree.debug_print();

    let (mut surface, gl, _gl_context) = initialize(1280, 720).expect("Failed to open a window!");

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    ui::style_ui(imgui.style_mut());

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &surface.window);

    gl::load_with(|s| surface.video.gl_get_proc_address(s) as _);

    let test_shader = shader::RawShader::from_compute(include_str!("shaders/compute.glsl"));

    let new_octree = voxel_data_structure::VoxelDAG::from_voxel_data(&vox_data[..], (126, 126, 126), 6);

    //quick debug for max ssbo size
    let mut ssbo_max_size = 0;
    unsafe { gl::GetIntegerv(gl::MAX_SHADER_STORAGE_BLOCK_SIZE, &mut ssbo_max_size); }
    debug!("MAX_SHADER_STORAGE_BLOCK_SIZE: {}", ssbo_max_size);
    debug!("MAX_COMPUTE_WORK_GROUP_COUNT: {:?}", compute::get_workgroup_count());
    debug!("MAX_COMPUTE_WORK_GROUP_SIZE: {:?}", compute::get_workgroup_size());
    debug!("MAX_COMPUTE_WORK_GROUP_INVOCATIONS: {}", compute::get_workgroup_invocations());

    //let render_texture = rasterizer::create_texture((1280, 720));
    // rasterizer::create_frame_buffer((1280, 720));

    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| surface.video.gl_get_proc_address(s) as *const c_void);

    let mut event_pump = surface.sdl.event_pump().expect("Failed to get event pump!");

    let mut last_frame = Instant::now();
    let mut delta_s: f32 = 1.0;

    //test shit
    let mesh = mesh::RenderMesh::from_vox_data(&mut surface, &vox_data[..], (126, 126, 126)).expect("Failed to create mesh!");
    // let octree_mesh = mesh::RenderMesh::from_octants(&mut surface, &new_octree, 128.0).expect("Failed to create mesh!");
    let mut camera = camera::Camera::default();

    let shader = shader::Shader::from_source(shader::ShaderSource{
        vertex_shader: include_str!("shaders/vertex.glsl").to_string(),
        geometry_shader: None,
        tesselation_shader: None,
        fragment_shader: include_str!("shaders/fragment.glsl").to_string(),
    });

    let quad_shader = shader::Shader::from_source(shader::ShaderSource{
        vertex_shader: include_str!("shaders/passthrough_vertex.glsl").to_string(),
        geometry_shader: None,
        tesselation_shader: None,
        fragment_shader: include_str!("shaders/passthrough_fragment_textured.glsl").to_string(),
    });

    let quad_va = rasterizer::create_render_quad();

    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) { continue; }

            match event {
                Event::Quit{..} => {
                    break 'main;
                },
                _ => {},
            }
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &surface.window, &event_pump.mouse_state());

        // rasterizer::prepare_frame(&gl);

        // unsafe {
        //     gl.clear_color(127.0 / 255.0, 103.0 / 255.0, 181.0 / 255.0, 1.0);
        //     gl.clear(glow::COLOR_BUFFER_BIT);
        // }

        //CODE STUFF HERE
        // camera.position.set_z(camera.position.z() + delta_s);

        /*unsafe {
            let col_pos = gl.get_uniform_location(shader.program().deref().handle(), "colour");
            gl.uniform_3_f32(col_pos, 0.0, 1.0, 1.0);
            rasterizer::draw_mesh(&mut surface, &gl, &camera, &shader, &mesh);
            // gl.uniform_3_f32(col_pos, 1.0, 0.0, 0.0);
            // rasterizer::draw_mesh(&mut surface, &gl, &camera, &shader, &octree_mesh);
        }*/

        //Render textured quad
        unsafe {
            let handle = quad_shader.program().deref().handle();
            gl::UseProgram(handle);
            gl::BindVertexArray(quad_va);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }

        //UI
        let ui = imgui.frame();

        let debug_window = imgui::Window::new(im_str!("Debug window"))
            .position([10.0, 10.0], imgui::Condition::Appearing)
            .size([320.0, 120.0], imgui::Condition::Appearing)
            .focused(false)
            .collapsible(true);

        debug_window.build(&ui, || {
            ui.text(format!("fps: {:.2}", 1.0 / delta_s));
            ui.separator();
            ui.text(format!("cam pos: {:?}", camera.position));
        });

        imgui_sdl2.prepare_render(&ui, &surface.window);
        renderer.render(ui);

        //FINISHING FRAME
        let now = Instant::now();
        let delta = now - last_frame;
        delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;

        surface.swap_buffer();
    }
}

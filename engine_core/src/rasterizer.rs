use glam::*;

use glow::HasContext;

use std::ops::Deref;

use luminance::{
    context::GraphicsContext,
    pipeline::PipelineState,
    render_state::RenderState,
    tess::TessSliceIndex,

    face_culling::{
        FaceCulling,
        FaceCullingOrder,
        FaceCullingMode
    },

    shader::program::{
        Uniform,
        Uniformable,
    },
    linear::M44
};
use luminance_derive::{Semantics, Vertex, UniformInterface};

use crate::{
    camera::Camera,
    shader::Shader,
    mesh::RenderMesh,
};

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)] //Tells luminance that it shouldn't generate an error if the GPU variable doesn't exist
    pub projection: Uniform<M44>,
    #[uniform(unbound)] //#[uniform(name = "foo")] can be used to rename a uniform
    pub view: Uniform<M44>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
}

#[derive(Vertex, Copy, Clone)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    pub position: VertexPosition,
}

pub type VertexIndex = u32;

pub fn prepare_frame(gl: &glow::Context) {
    unsafe {
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        gl.clear_color(127.0 / 255.0, 103.0 / 255.0, 181.0 / 255.0, 1.0);
    }
}

pub fn draw_mesh(surface: &mut luminance_sdl2::SDL2Surface, gl: &glow::Context, camera: &Camera, shader: &Shader, mesh: &RenderMesh) {
    let back_buffer = surface.back_buffer().expect("Couldn't get the backbuffer!");

    let projection = camera.get_proj(surface.width(), surface.height());
    let view = camera.get_view();

    let render_state = RenderState::default();

    surface.pipeline_builder().pipeline(
        &back_buffer,
        &PipelineState::default(),
        |_, mut shd_gate| {
            shd_gate.shade(shader.program(), |iface, mut rdr_gate| {
                let handle = shader.program().deref().handle();

                camera.upload_fields(&gl, handle);
                iface.projection.update(projection.to_cols_array_2d());
                iface.view.update(view.to_cols_array_2d());

                rdr_gate.render(&render_state, |mut tess_gate| {
                    // iface.model.update();
                    tess_gate.render(mesh.tess.slice(..))
                })
            });
        }
    );
}

pub fn create_texture(resolution: (i32, i32)) -> u32 {
    let mut texture = 0;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32, resolution.0, resolution.1, 0, gl::RGBA, gl::FLOAT, std::ptr::null()); //null ptr so texture is empty
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); //possibly make the filtering linear?
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0); //Unbind the texture
    }

    texture
}

pub fn create_frame_buffer(resolution: (i32, i32)) -> u32 {
    let mut fb = 0;

    unsafe {
        gl::GenFramebuffers(1, &mut fb);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fb);

        let mut depth_tex = 0;
        gl::GenRenderbuffers(1, &mut depth_tex);
        gl::BindRenderbuffer(gl::RENDERBUFFER, depth_tex);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, resolution.0, resolution.1);

        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_tex);
        gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, create_texture(resolution), 0);

        let draw_buffers = [gl::COLOR_ATTACHMENT0];
        gl::DrawBuffers(1, draw_buffers.as_ptr());

        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            panic!("aaaa framebuffer broken uwu");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }

    fb
}

static quad_vertices: [f32; 12] = [
     1.0,  1.0, 0.0,
     1.0, -1.0, 0.0,
    -1.0, -1.0, 0.0,
    -1.0,  1.0, 0.0,
];

static quad_indices: [u32; 6] = [
    0,1,3,
    1,2,3,
];

pub fn create_render_quad() -> u32 {
    let mut quad_va = 0;
    let mut quad_vb = 0;
    let mut quad_ebo = 0;

    unsafe {
        //vertex array
        gl::GenVertexArrays(1, &mut quad_va);
        gl::BindVertexArray(quad_va);

        //vertex buffer
        gl::GenBuffers(1, &mut quad_vb);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vb);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of::<[f32; 12]>() as isize, quad_vertices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        //indices
        gl::GenBuffers(1, &mut quad_ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad_ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, std::mem::size_of::<[u32; 6]>() as isize, quad_indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);

        //vertex attribs
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        //unbind
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    quad_va
}

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

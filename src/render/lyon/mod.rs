pub use lyon::*;

use lyon::{lyon_tessellation::{VertexBuffers, BuffersBuilder, geometry_builder::Positions, StrokeTessellator, StrokeOptions}, algorithms::rounded_polygon::Point, path::{Path, Position}};
use super::{Renderer, mesh::{Mesh, Vertex}};

impl Renderer {
    pub fn add_path(&mut self, path: &Path, width: f32, color: [f32; 3]) {
        let mut geometry: VertexBuffers<Point, u32> = VertexBuffers::new();
        let mut geometry_builder =
            BuffersBuilder::new(&mut geometry, Positions).with_inverted_winding();
        let mut tessellator = StrokeTessellator::new();

        tessellator
            .tessellate(
                path,
                &StrokeOptions::default().with_line_width(width),
                &mut geometry_builder,
            )
            .unwrap();

        let mesh = Mesh::new(
            &self.device,
            &geometry
                .vertices
                .iter()
                .map(|x| Vertex {
                    position: [x.position().x, x.position().y, 0.0],
                    color,
                })
                .collect::<Vec<_>>(),
            &geometry.indices,
        );

        self.add_mesh(mesh);
    }
}

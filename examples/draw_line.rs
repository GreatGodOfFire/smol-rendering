use smol_rendering::{
    render::mesh::{Mesh, Vertex},
    App,
};

use lyon::{
    geom::point,
    lyon_tessellation::{
        geometry_builder::Positions, BuffersBuilder, StrokeOptions, StrokeTessellator,
        VertexBuffers,
    },
    math::Point,
    path::{Path, Position},
};

fn main() {
    let mut app = App::new("smol_rendering::draw_line");

    let mut geometry: VertexBuffers<Point, u32> = VertexBuffers::new();
    let mut geometry_builder =
        BuffersBuilder::new(&mut geometry, Positions).with_inverted_winding();
    let mut tessellator = StrokeTessellator::new();

    let mut path_builder = Path::builder();
    path_builder.begin(point(-100.0, 0.0));
    path_builder.line_to(point(100.0, 100.0));
    path_builder.end(false);
    let path = path_builder.build();

    tessellator
        .tessellate(
            &path,
            &StrokeOptions::default().with_line_width(1.0),
            &mut geometry_builder,
        )
        .unwrap();

    let mesh = Mesh::new(
        app.renderer().device(),
        &geometry
            .vertices
            .iter()
            .map(|x| Vertex {
                position: [x.position().x, x.position().y, 0.0],
                color: [1.0; 3],
            })
            .collect::<Vec<_>>(),
        &geometry.indices,
    );
    app.renderer_mut().add_mesh(mesh);

    app.run();
}

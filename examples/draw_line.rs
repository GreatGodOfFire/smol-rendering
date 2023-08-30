use smol_rendering::{
    render::lyon::{path::Path, geom::point},
    App,
};

fn main() {
    let mut app = App::new("smol_rendering::draw_line");

    let mut path_builder = Path::builder();
    path_builder.begin(point(-100.0, 0.0));
    path_builder.line_to(point(100.0, 100.0));
    path_builder.end(false);
    let path = path_builder.build();

    app.renderer_mut().add_path(&path, 1.0, [1.0; 3]);

    app.run();
}

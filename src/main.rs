pub mod pipe;
pub mod coordinate;

use cursive::Cursive;
use cursive::Vec2;
use cursive::views::Canvas;

const X: usize = 200;
const Y: usize = 100;

pub struct Model;

pub fn create_canvas(model: Model) -> Canvas<Model> {
    Canvas::new(model)
        .with_draw(move |_model, _printer| {})
        // The required size will be set by the window layout, not by the printer!
        .with_required_size(move |_model, _req_size| Vec2::new(X, Y))
}

pub fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    // Build the UI from the model
    siv.add_layer(create_canvas(Model));

    siv.run();
}

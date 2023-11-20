#![allow(dead_code)]
extern crate env_logger;
extern crate log;

mod my_random;

fn main() {
    draw::draw();
}

mod draw {
    use log::LevelFilter;
    use nannou::prelude::*;
    use crate::my_random::MyRandom;

    struct Model {
        grid_count_x: usize,
        grid_count_y: usize,
        base_scale: f32,
        pct_circles: f32,
        rng_seed: u64,
    }

    impl Default for Model {
        fn default() -> Self {
            Self{
                grid_count_x: 20,
                grid_count_y: 20,
                base_scale: 0.7,
                pct_circles: 0.5,
                rng_seed: 100,
            }
        }
    }

    pub fn draw() {
        env_logger::builder()
            .format_level(false)
            .format_target(false)
            .format_timestamp(None)
            .filter(Some("nannou_00::draw"), LevelFilter::Trace)
            .init();

        nannou::app(model)
            .event(event)
            .simple_window(view)
            .run();
    }

    fn model(_app: &App) -> Model {
        Model::default()
    }

    fn event(_app: &App, _model: &mut Model, _event: Event) {}

    fn view(app: &App, model: &Model, frame: Frame) {
        // For consistent results from frame to frame
        MyRandom::seed_from_u64(model.rng_seed);

        // Prepare to draw.
        let draw = app.draw();
        // Clear the background.
        draw.background().color(BLACK);

        // Get boundary of the window (to constrain the movements of our circle)
        let boundary = app.window_rect();
        let cell_width: f32 = (boundary.right() - boundary.left()) / model.grid_count_x as f32;
        let cell_height: f32 = (boundary.top() - boundary.bottom()) / model.grid_count_y as f32;
        let cell_size = f32::min(cell_width, cell_height);
        let shape_size = cell_size * model.base_scale;
        let mut pt_center = boundary.bottom_left() + Point2::new(-cell_width / 2.0, -cell_height / 2.0);

        for _ in 0..model.grid_count_x {
            pt_center.x += cell_width;
            pt_center.y = boundary.bottom() - cell_height / 2.0;
            for _ in 0..model.grid_count_y {
                pt_center.y += cell_height;
                if MyRandom::get_float() < model.pct_circles
                {
                    draw_circle_from_size_ctr(&draw, pt_center, shape_size);
                }
                else
                {
                    draw_quad_from_size_ctr(&draw, pt_center, shape_size);
                }
            }
        }

        draw.to_frame(app, &frame).unwrap();
    }

    fn draw_quad_from_size_ctr(draw: &Draw, center: Point2, size: f32) {
        let size_d2 = size / 2.0;
        let v1 = center + Point2::new(size_d2, size_d2);
        let v2 = center + Point2::new(size_d2, -size_d2);
        let v3 = center + Point2::new(-size_d2, -size_d2);
        let v4 = center + Point2::new(-size_d2, size_d2);
        draw.quad()
            .color(BLUE)
            .points(v1, v2, v3, v4);
    }
    fn draw_circle_from_size_ctr(draw: &Draw, center: Point2, size: f32) {
        draw.ellipse()
            .color(RED)
            .w(size)
            .h(size)
            .xy(center);
    }
}
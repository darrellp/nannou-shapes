#![allow(dead_code)]
extern crate env_logger;
extern crate log;

mod my_random;

fn main() {
    draw::main();
}

mod draw {
    use log::LevelFilter;
    use nannou::prelude::*;
    use crate::my_random::MyRandom;
    use nannou_egui::{self, egui, Egui};

    pub fn main() {
        env_logger::builder()
            .format_level(false)
            .format_target(false)
            .format_timestamp(None)
            .filter(Some("nannou_00::draw"), LevelFilter::Trace)
            .init();

        nannou::app(model).update(update).run();
    }


    struct Settings {
        grid_count_x: usize,
        grid_count_y: usize,
        base_scale: f32,
        pct_circles: f32,
        rng_seed: u64,
    }

    impl Default for Settings {
        fn default() -> Self {
            Self{
                grid_count_x: 20,
                grid_count_y: 20,
                base_scale: 0.7,
                pct_circles: 0.5,
                rng_seed: MyRandom::from_range(1u64,u64::MAX) as u64,
            }
        }
    }

    struct Model {
        settings: Settings,
        egui: Egui,
    }

    impl Model {
        pub fn new(app: &App) -> Self {
            let window_id = app
                .new_window()
                .mouse_pressed(mouse_pressed)
                .view(view)
                .raw_event(raw_window_event)
                .build()
                .unwrap();
            let window = app.window(window_id).unwrap();
            let egui = Egui::from_window(&window);
            Self {
                settings: Settings::default(),
                egui
            }
        }
    }

    fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
        model.egui.handle_raw_event(event);
    }


    fn mouse_pressed(app: &App, model_window: &mut Model, button: MouseButton) {
        if button == MouseButton::Left {
            model_window.settings.rng_seed = app.elapsed_frames();
        }
    }

    fn update(_app: &App, model: &mut Model, update: Update) {
        let egui = &mut model.egui;
        let settings = &mut model.settings;

        egui.set_elapsed_time(update.since_start);
        let ctx = egui.begin_frame();
        egui::Window::new("Shapes Settings").show(&ctx, |ui| {
            ui.add(egui::Slider::new(&mut settings.grid_count_x, 1..=100).text("X Count"));
            ui.add(egui::Slider::new(&mut settings.grid_count_y, 1..=100).text("Y Count"));
        });
    }


    fn model(app: &App) -> Model {
        Model::new(app)
    }

    fn view(app: &App, model: &Model, frame: Frame) {
        let shapes_info = &model.settings;

        // For consistent results from frame to frame
        MyRandom::seed_from_u64(shapes_info.rng_seed);

        // Prepare to draw.
        let draw = app.draw();
        // Clear the background.
        draw.background().color(BLACK);

        // Get boundary of the window (to constrain the movements of our circle)
        let boundary = app.window_rect();
        let cell_width: f32 = (boundary.right() - boundary.left()) / shapes_info.grid_count_x as f32;
        let cell_height: f32 = (boundary.top() - boundary.bottom()) / shapes_info.grid_count_y as f32;
        let cell_size = f32::min(cell_width, cell_height);
        let shape_size = cell_size * shapes_info.base_scale;
        let mut pt_center = boundary.bottom_left() + Point2::new(-cell_width / 2.0, -cell_height / 2.0);

        for _ in 0..shapes_info.grid_count_x {
            pt_center.x += cell_width;
            pt_center.y = boundary.bottom() - cell_height / 2.0;
            for _ in 0..shapes_info.grid_count_y {
                pt_center.y += cell_height;
                if MyRandom::get_float() < shapes_info.pct_circles
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
        model.egui.draw_to_frame(&frame).unwrap();
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
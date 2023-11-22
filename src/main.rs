#![allow(dead_code)]
extern crate env_logger;
extern crate log;

mod my_random;

fn main() {
    draw::main();
}

mod draw {
    use env_logger::fmt::Color;
    use log::LevelFilter;
    use nannou::image::pnm::ArbitraryTuplType::RGB;
    use nannou::prelude::*;
    use crate::my_random::MyRandom;
    use nannou_egui::{self, egui, Egui};
    use nannou_egui::egui::Color32;

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
        base_size: f32,
        max_scale: f32,
        pct_circles: f32,
        handle_mouse: bool,
        alpha: u8,
        rng_seed: u64,
        circle_color: [u8; 3],
        square_color: [u8; 3],
    }

    impl Default for Settings {
        fn default() -> Self {
            Self{
                grid_count_x: 20,
                grid_count_y: 20,
                base_size: 0.7,
                max_scale: 1.0,
                pct_circles: 0.5,
                handle_mouse: true,
                alpha: 255u8,
                circle_color: [255_u8, 0_u8, 0_u8],
                square_color: [0_u8, 0_u8, 255_u8],

                rng_seed: MyRandom::from_range(1u64,u64::MAX) as u64,
            }
        }
    }

    struct Model {
        settings: Settings,
        egui: Egui,
        show_ui: bool
    }

    impl Model {
        pub fn new(app: &App) -> Self {
            let window_id = app
                .new_window()
                .mouse_pressed(mouse_pressed)
                .key_released(key_pressed)
                .view(view)
                .raw_event(raw_window_event)
                .build()
                .unwrap();
            let window = app.window(window_id).unwrap();
            let egui = Egui::from_window(&window);
            Self {
                settings: Settings::default(),
                show_ui: true,
                egui
            }
        }
    }

    fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
        model.egui.handle_raw_event(event);
    }

    fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
        if button == MouseButton::Left && model.settings.handle_mouse {
            reseed(app, &mut model.settings);
        }
    }

    fn key_pressed(_app: &App, model: &mut Model, key: Key) {
        if key == Key::U {
            model.show_ui = !model.show_ui;
        }
    }

    fn reseed(app: &App, settings: &mut Settings) {
        settings.rng_seed = app.elapsed_frames();
    }

    fn update(_app: &App, model: &mut Model, update: Update) {
        let egui = &mut model.egui;
        let settings = &mut model.settings;

        egui.set_elapsed_time(update.since_start);
        let ctx = egui.begin_frame();

        // Only use mouse to reset seed if we're not over the ui
        settings.handle_mouse = !ctx.is_pointer_over_area();

        if !model.show_ui {
            return;
        }

        egui::Window::new("Shapes Settings").show(&ctx, |ui| {
            egui::CollapsingHeader::new("Colors")
                .show(ui, |ui| {
                    ui.color_edit_button_srgb(&mut settings.square_color);
                    ui.color_edit_button_srgb(&mut settings.circle_color);
                });
            ui.add(egui::Slider::new(&mut settings.grid_count_x, 1..=100)
                .text("X Count"));
            ui.add(egui::Slider::new(&mut settings.grid_count_y, 1..=100)
                .text("Y Count"));
            ui.add(egui::Slider::new(&mut settings.base_size, 0.0..=2.0)
                .text("Base Size"));
            ui.add(egui::Slider::new(&mut settings.max_scale, 1.0..=5.0)
                .text("Max Scale"));
            ui.add(egui::Slider::new(&mut settings.pct_circles, 0.0..=1.0)
                .text("Circles"));
            ui.add(egui::Slider::new(&mut settings.alpha, 0u8..=255u8)
                .text("Alpha"));
        });
    }

    fn model(app: &App) -> Model {
        Model::new(app)
    }

    fn view(app: &App, model: &Model, frame: Frame) {
        let settings = &model.settings;

        // For consistent results from frame to frame
        MyRandom::seed_from_u64(settings.rng_seed);

        // Prepare to draw.
        let draw = app.draw();
        // Clear the background.
        draw.background().color(BLACK);

        // Get boundary of the window to size everything correctly
        let boundary = app.window_rect();
        let cell_width: f32 = (boundary.right() - boundary.left()) / settings.grid_count_x as f32;
        let cell_height: f32 = (boundary.top() - boundary.bottom()) / settings.grid_count_y as f32;
        let cell_size = f32::min(cell_width, cell_height);
        let shape_size = cell_size * settings.base_size;
        let mut pt_center = boundary.bottom_left() + Point2::new(-cell_width / 2.0, -cell_height / 2.0);

        let mut positions: Vec<Point2> = Vec::with_capacity(
            settings.grid_count_x * settings.grid_count_y);
        for _ in 0..settings.grid_count_x {
            pt_center.x += cell_width;
            pt_center.y = boundary.bottom() - cell_height / 2.0;
            for _ in 0..settings.grid_count_y {
                pt_center.y += cell_height;
                positions.push(pt_center);
            }
        }

        for i in 0..positions.len() - 1 {
            let swap_index = MyRandom::from_range(i, positions.len() - 1);
            (positions[i], positions[swap_index]) = (positions[swap_index], positions[i]);
        }
        for pt_center in positions {
            let scale = MyRandom::get_float() * (settings.max_scale - 1.0) + 1.0;
            let cur_size = shape_size * scale;
            if MyRandom::get_float() < settings.pct_circles
            {
                draw_circle_from_size_ctr(&draw, pt_center, cur_size, settings.circle_color, settings.alpha);
            }
            else
            {
                draw_quad_from_size_ctr(&draw, pt_center, cur_size, settings.square_color, settings.alpha);
            }
        }

        draw.to_frame(app, &frame).unwrap();
        if model.show_ui {
            model.egui.draw_to_frame(&frame).unwrap();}
    }

    fn draw_quad_from_size_ctr(draw: &Draw, center: Point2, size: f32, clr: [u8; 3], alpha: u8) {
        let size_d2 = size / 2.0;
        let v1 = center + Point2::new(size_d2, size_d2);
        let v2 = center + Point2::new(size_d2, -size_d2);
        let v3 = center + Point2::new(-size_d2, -size_d2);
        let v4 = center + Point2::new(-size_d2, size_d2);
        let (r, g, b) = clr.into();
        draw.quad()
            .rgba8(r, g, b, alpha)
            .points(v1, v2, v3, v4);
    }
    fn draw_circle_from_size_ctr(draw: &Draw, center: Point2, size: f32, clr: [u8; 3], alpha: u8) {
        let (r, g, b) = clr.into();
        draw.ellipse()
            .rgba8(r, g, b, alpha)
            .w(size)
            .h(size)
            .xy(center);
    }
}
#![allow(dead_code)]
extern crate env_logger;
extern crate log;

mod my_random;

fn main() {
    draw::main();
}

mod draw {
    use log::LevelFilter;
    use nannou::color::IntoColor;
    use nannou::prelude::*;
    use crate::my_random::MyRandom;
    use nannou_egui::{self, egui, Egui};
    use colourado::{ColorPalette, PaletteType};

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
        inset_count: u8,
        fixed_inset_count: bool,
        inversed_radii: bool,

        circle_info: ColorInfo,
        square_info: ColorInfo,
    }

    impl Default for Settings {
        fn default() -> Self {
            let blue = Rgb::new(0.0, 0.0, 1.0);
            let black = Rgb::new(0.0, 0.0, 0.0);
            let square_color_info = ColorInfo {
                colors: [blue, black, black, black],
                ..Default::default()
            };
            Self{
                grid_count_x: 27,
                grid_count_y: 20,
                base_size: 0.7,
                max_scale: 1.0,
                pct_circles: 0.5,
                handle_mouse: true,
                alpha: 255u8,
                inset_count: 1,
                fixed_inset_count: false,
                inversed_radii: true,
                circle_info: ColorInfo::default(),
                square_info: square_color_info,
                rng_seed: MyRandom::from_range(1u64,u64::MAX) as u64,
            }
        }
    }

    struct ColorInfo {
        colors: [nannou::color::Rgb<f32>; 4],
        enabled: [bool; 4],
        h_rnd: f32,
        s_rnd: f32,
        v_rnd: f32,
    }

    impl ColorInfo {
        pub fn pick_color(&self) -> (u8, u8, u8) {
            let mut enabled_colors = Vec::<Rgb<f32>>::with_capacity(4);
            for i in 0..4 {
                if self.enabled[i] {
                    enabled_colors.push(self.colors[i]);
                }
            }
            if enabled_colors.len() == 0 {
                return (0_u8, 0_u8, 0_u8);
            }
            let base = enabled_colors[MyRandom::from_range(0, enabled_colors.len())];
            if self.h_rnd <= 0.0 && self.s_rnd <= 0.0 && self.v_rnd <= 0.0 {
                let (r, g, b) = base.into_components();
                return ((r * 255.9) as u8, (g * 255.9) as u8, (b * 255.9) as u8);
            }

            let hsv: Hsv = base.into_hsv();
            let (mut h, mut s, mut v) = hsv.into();
            if self.h_rnd > 0.0 {
                h += (MyRandom::get_float() - 0.5) * self.h_rnd * 360.0;
                let degrees = h.to_positive_degrees();
                if degrees > 360.0 {
                    h -= 360.0;
                }
            }
            if self.s_rnd > 0.0 {
                s += MyRandom::get_float() * self.s_rnd - self.s_rnd / 2.0;
                if s < 0.0 {
                    s = 0.0;
                } else if s > 1.0 {
                    s = 1.0;
                }
            }
            if self.v_rnd > 0.0 {
                v += MyRandom::get_float() * self.v_rnd - self.v_rnd / 2.0;
                if v < 0.0 {
                    v = 0.0;
                } else if v > 1.0 {
                    v = 1.0;
                }
            }
            let hsv = Hsv::new(h, s, v);
            let rgb = hsv.into_rgb::<nannou::color::encoding::Srgb>();
            let (r, g, b) = rgb.into_components();
            ((r * 255.9) as u8, (g * 255.9) as u8, (b * 255.9) as u8)
        }
    }

    impl Default for ColorInfo {
        fn default() -> Self {
            let red = Rgb::new(1.0, 0.0, 0.0);
            let black = Rgb::new(0.0, 0.0, 0.0);
            Self {
                colors: [red, black, black, black],
                enabled: [true, false, false, false],
                h_rnd: 0.0,
                s_rnd: 0.0,
                v_rnd: 0.0,
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

    #[inline]
    fn rgb_to_array(rgb: Rgb<f32>) -> [f32; 3] {
        [rgb.red, rgb.green, rgb.blue]
    }

    #[inline]
    fn array_to_rgb(array: [f32; 3]) -> Rgb<f32> {
        Rgb::new(array[0], array[1], array[2])
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
                    ui.add(egui::Label::new("Square Colors"));
                    ui.horizontal(|ui| {
                        let mut a0 = rgb_to_array(settings.square_info.colors[0]);
                        let mut a1 = rgb_to_array(settings.square_info.colors[1]);
                        let mut a2 = rgb_to_array(settings.square_info.colors[2]);
                        let mut a3 = rgb_to_array(settings.square_info.colors[3]);
                        ui.color_edit_button_rgb(&mut a0);
                        ui.checkbox(&mut settings.square_info.enabled[0],"");
                        ui.color_edit_button_rgb(&mut a1);
                        ui.checkbox(&mut settings.square_info.enabled[1],"");
                        ui.color_edit_button_rgb(&mut a2);
                        ui.checkbox(&mut settings.square_info.enabled[2],"");
                        ui.color_edit_button_rgb(&mut a3);
                        ui.checkbox(&mut settings.square_info.enabled[3],"");
                        settings.square_info.colors[0] = array_to_rgb(a0);
                        settings.square_info.colors[1] = array_to_rgb(a1);
                        settings.square_info.colors[2] = array_to_rgb(a2);
                        settings.square_info.colors[3] = array_to_rgb(a3);
                    });
                    ui.end_row();
                    if ui.add(egui::Button::new("Randomize")).clicked() {
                        let palette = ColorPalette::new(4, PaletteType::Random, false);
                        settings.square_info.colors[0] = array_to_rgb(palette.colors[0].to_array());
                        settings.square_info.colors[1] = array_to_rgb(palette.colors[1].to_array());
                        settings.square_info.colors[2] = array_to_rgb(palette.colors[2].to_array());
                        settings.square_info.colors[3] = array_to_rgb(palette.colors[3].to_array());
                        settings.square_info.enabled[0] = true;
                        settings.square_info.enabled[1] = true;
                        settings.square_info.enabled[2] = true;
                        settings.square_info.enabled[3] = true;
                    }

                    ui.add(egui::Slider::new(&mut settings.square_info.h_rnd, 0.0..=1.0)
                        .text("Hue Variation"));
                    ui.add(egui::Slider::new(&mut settings.square_info.s_rnd, 0.0..=1.0)
                        .text("Sat Variation"));
                    ui.add(egui::Slider::new(&mut settings.square_info.v_rnd, 0.0..=1.0)
                        .text("Value Variation"));

                    ui.add(egui::Label::new("Circle Colors"));
                    ui.horizontal(|ui| {
                        let mut a0 = rgb_to_array(settings.circle_info.colors[0]);
                        let mut a1 = rgb_to_array(settings.circle_info.colors[1]);
                        let mut a2 = rgb_to_array(settings.circle_info.colors[2]);
                        let mut a3 = rgb_to_array(settings.circle_info.colors[3]);
                        ui.color_edit_button_rgb(&mut a0);
                        ui.checkbox(&mut settings.circle_info.enabled[0],"");
                        ui.color_edit_button_rgb(&mut a1);
                        ui.checkbox(&mut settings.circle_info.enabled[1],"");
                        ui.color_edit_button_rgb(&mut a2);
                        ui.checkbox(&mut settings.circle_info.enabled[2],"");
                        ui.color_edit_button_rgb(&mut a3);
                        ui.checkbox(&mut settings.circle_info.enabled[3],"");
                        settings.circle_info.colors[0] = array_to_rgb(a0);
                        settings.circle_info.colors[1] = array_to_rgb(a1);
                        settings.circle_info.colors[2] = array_to_rgb(a2);
                        settings.circle_info.colors[3] = array_to_rgb(a3);
                    });
                    ui.end_row();
                    if ui.add(egui::Button::new("Randomize")).clicked() {
                        let palette = ColorPalette::new(4, PaletteType::Random, false);
                        settings.circle_info.colors[0] = array_to_rgb(palette.colors[0].to_array());
                        settings.circle_info.colors[1] = array_to_rgb(palette.colors[1].to_array());
                        settings.circle_info.colors[2] = array_to_rgb(palette.colors[2].to_array());
                        settings.circle_info.colors[3] = array_to_rgb(palette.colors[3].to_array());
                        settings.circle_info.enabled[0] = true;
                        settings.circle_info.enabled[1] = true;
                        settings.circle_info.enabled[2] = true;
                        settings.circle_info.enabled[3] = true;
                    }

                    ui.add(egui::Slider::new(&mut settings.circle_info.h_rnd, 0.0..=1.0)
                        .text("Hue Variation"));
                    ui.add(egui::Slider::new(&mut settings.circle_info.s_rnd, 0.0..=1.0)
                        .text("Sat Variation"));
                    ui.add(egui::Slider::new(&mut settings.circle_info.v_rnd, 0.0..=1.0)
                        .text("Value Variation"));
                    ui.separator();
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
            ui.add(egui::Slider::new(&mut settings.inset_count, 1u8..=20u8)
                .text("Inset Count"));
            ui.horizontal(|ui| {
                ui.checkbox(&mut settings.fixed_inset_count,"Fixed Inset Count");
                ui.checkbox(&mut settings.inversed_radii,"Inverse Radii");
            });
            ui.end_row();

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
            let mut cur_size = shape_size * scale;

            let insets = if settings.fixed_inset_count {
                settings.inset_count
            } else {
                MyRandom::from_range(1, settings.inset_count + 1)
            };

            let delta = cur_size / (insets as f32);
            if MyRandom::get_float() < settings.pct_circles
            {
                for i in 1u8..=insets {
                    let color: [u8; 3] = settings.circle_info.pick_color().into();
                    draw_circle_from_size_ctr( & draw,
                        pt_center,
                        if settings.inversed_radii {
                            cur_size / (i as f32)
                        }
                        else {
                            cur_size
                        },
                        color,
                        settings.alpha);
                    if ! settings.inversed_radii {
                        cur_size -= delta;
                    }
                }
            }
            else
            {
                for i in 1u8..=insets {
                    let color: [u8; 3] = settings.square_info.pick_color().into();
                    draw_quad_from_size_ctr(&draw,
                        pt_center,
                        if settings.inversed_radii {
                            cur_size / (i as f32)
                        }
                        else {
                            cur_size
                        },
                        color,
                        settings.alpha);
                    if !settings.inversed_radii {
                        cur_size -= delta;
                    }
                }
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
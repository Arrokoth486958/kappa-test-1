use eframe::NativeOptions;
use egui::Vec2;
// use winit::{window::WindowBuilder, dpi::LogicalSize, event_loop::EventLoop, event::{WindowEvent, Event}};

fn main() {
    let options = NativeOptions {
        follow_system_theme: false,
        centered: true,
        initial_window_size: Some(Vec2::new(600.0, 400.0)),
        min_window_size: Some(Vec2::new(600.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native("Kappa", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::<MyApp>::default()
    })).unwrap();

    // let window_builder = WindowBuilder::new()
    //     .with_inner_size(LogicalSize::new(600, 400));
    // println!("{:?}", window_builder);

    // let event_loop = EventLoop::new().unwrap();
    // let window = window_builder.build(&event_loop).unwrap();

    // // let egui_ctx = egui::Context::

    // event_loop.run(move |event, elwt| {
    //     match event {
    //         Event::WindowEvent{
    //             ref event,
    //             window_id,
    //         } => {
    //             if window.id() == window_id {
    //                 match event {
    //                     WindowEvent::CloseRequested => {
    //                         elwt.exit();
    //                     }
    //                     WindowEvent::Resized(size) => {
    //                         window.focus_window();
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         },
    //         _ => {}
    //     }
    // }).unwrap();
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!(
                "../assets/fufu.png"
            ));
        });
    }
}
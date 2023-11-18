use std::sync::Arc;

use eframe::{NativeOptions, CreationContext};
use egui::{Vec2, FontDefinitions, FontFamily, FontData};
use font_kit::family_name::FamilyName;
use fontdb::Family;
// use winit::{window::WindowBuilder, dpi::LogicalSize, event_loop::EventLoop, event::{WindowEvent, Event}};

fn main() {
    let options = NativeOptions {
        follow_system_theme: false,
        multisampling: 2,
        centered: true,
        initial_window_size: Some(Vec2::new(600.0, 400.0)),
        min_window_size: Some(Vec2::new(600.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native("Kappa", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Box::<MyApp>::new(MyApp::new(cc))
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

impl MyApp {
    fn new(cc: &CreationContext) -> Self {
        let ctx = &cc.egui_ctx;
        
        let mut fonts = FontDefinitions::default();
        // 笑死，我超勇的
        fonts.families.clear();
        fonts.font_data.clear();

        let mut database = fontdb::Database::new();
        let target_family = database.family_name(&Family::Serif);
        let target_font_family = FontFamily::Name(target_family.into());
        fonts.families.insert(target_font_family.clone(), vec![]);
        database.load_system_fonts();
        // println!("{:?}", database);
        for i in database.faces() {
            let family_name = i.families.get(0).unwrap();
            if family_name.0 == target_family {
                let mut data: Vec<u8> = Vec::new();
                match &i.source {
                    fontdb::Source::Binary(bin) => {
                        let as_ref: &dyn AsRef<[u8]> =  bin.as_ref();
                        let slice: &[u8] = as_ref.as_ref();
                        data = slice.to_vec()
                    }
                    fontdb::Source::File(path) => {
                        data = std::fs::read(path).unwrap();
                    }
                    fontdb::Source::SharedFile(path, bin) => {
                        todo!("Font by SharedFile")
                    }
                }
                // 加载字体到内存
                let x = fonts.families.get_mut(&target_font_family).unwrap().push(family_name.0.to_owned());
                fonts.font_data.insert(family_name.0.to_owned(), FontData::from_owned(data));
            }
        }
        // let sys_fonts = font_kit::source::SystemSource::new().all_fonts().unwrap();
        // println!("{:?}", sys_fonts);

        // fonts.families.append(FontFamily::Name("Default"));

        println!("{:?}", fonts.families);

        ctx.set_fonts(fonts);

        MyApp {
            name: "Arrokoth".to_owned(),
            age: 24,
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
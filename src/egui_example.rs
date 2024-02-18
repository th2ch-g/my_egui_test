
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    Reactive,
    Continuous,
}
// #[derive(Default, Clone, Debug, Copy)]
#[derive(Debug, Clone)]
pub struct GUI {
    pub show_confirmation_dialog: bool,
    pub allowed_to_close: bool,
    pub show_immediate_viewport: bool,
    pub show_cursor_test: bool,
    // pub show_deferred_viewport: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub frame_times: egui::util::History<f32>,
    run_mode: RunMode,
    pub counter: usize,
}

impl Default for GUI {
    fn default() -> Self {
        // let tmp = std::sync::atomic::AtomicBool::new(false);
        Self {
            show_confirmation_dialog: false,
            allowed_to_close: false,
            show_immediate_viewport: false,
            show_cursor_test: false,
            // show_deferred_viewport: std::sync::Arc<tmp>,
            frame_times: egui::util::History::new(0..300, 1.0),
            run_mode: RunMode::Reactive,
            counter: 0,
        }
    }
}

// impl Default for GUI {
//     fn default() -> Self {
//         Self {
//             show_confirmation_dialog: false,
//             allowed_to_close: false,
//         }
//     }
// }

impl GUI {
    pub fn run_gui(&mut self, ctx: &egui::Context) {
        // egui::CentralPanel::default().show(ctx, |ui| {

        // if let Some(latest) = self.frame_times.latest_mut() {
        //     let mut system = sysinfo::System::new_all();
        //     *latest = system.get_global_processor_info().get_cpu_usage();
        // }
        // self.frame_times.add(now(), system.get_global_processor_info().get_cpu_usage());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // ui.menu_button("File", |ui| {
                //     if let Some(path) = rfd::FileDialog::new()
                //         .add_filter("pdb", &["pdb"])
                //         .pick_file() {
                //         let ppath = Some(path.display().to_string());
                //         dbg!(&ppath);
                //     }
                // });
                if ui.button("File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("pdb", &["pdb"])
                        .pick_file() {
                        let ppath = Some(path.display().to_string());
                        dbg!(&ppath);
                    }
                }

                ui.separator();

                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // ui.menu_button("File", |ui| {
                //     if let Some(path) = rfd::FileDialog::new()
                //         .add_filter("pdb", &["pdb"])
                //         .pick_file() {
                //         let ppath = Some(path.display().to_string());
                //         dbg!(&ppath);
                //     }
                // });
                if ui.button("File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("pdb", &["pdb"])
                        .pick_file() {
                        let ppath = Some(path.display().to_string());
                        dbg!(&ppath);
                    }
                }

                ui.separator();

                ui.add(egui::github_link_file!(
                    "https://github.com/th2ch-g",
                    "(source code)"
                    // egui::RichText::new("(source code)")
                ));

                ui.separator();

                egui::widgets::global_dark_light_mode_switch(ui);

                ui.separator();
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Hello world from side panel");
            });
            ui.separator();
            use egui::special_emojis::GITHUB;
            ui.hyperlink_to(
                format!("{GITHUB} go to my Github"),
                "https://github.com/th2ch-g",
            );
            ui.separator();

            // let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
            ui.collapsing("Theme", |ui| {
            //     ui.group(|ui| {
            //         theme.ui(ui);
            //         theme.clone().store_in_memory(ui.ctx());
            //     });
                ui.label("git");
            });

            ui.checkbox(
                &mut self.show_cursor_test,
                "Open Cursor"
            );

            if self.show_cursor_test {
                egui::Window::new("Cursor Test")
                    .default_open(true)
                    .max_width(1000.0)
                    .max_height(1000.0)
                    .resizable(true)
                    .show(ctx, |ui| {
                        for &ci in &egui::CursorIcon::ALL {
                            let _ = ui.button(format!("{ci:?}")).on_hover_cursor(ci);
                    }
                });
            }

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.run_mode, RunMode::Reactive, "Reactive");
                ui.radio_value(&mut self.run_mode, RunMode::Continuous, "Continuous");
            });
            // self.frame_timesVj.add(ctx.input(|i| i.time), );
            // ui.label(
            //     format!(
            //         "Mean CPU usage: {} ms / frame",
            //         1e3 * self.frame_times.average().unwrap_or_default()
            //     )
            // );
            // dbg!(&self.frame_times.average().unwrap_or_default());
            // ).on_hover_text("includes all app logic, egui layout, tesselation...");

            // ui.menu_button("Cursor Test", |ui| {
            //     ui.set_style(ui.ctx().style());
            //     // egui::SidePanel::left("test").show(ctx, |ui| {
            //     //     ui.label("test");
            //     // });
            //     // ui.label("test");
            //     egui::Window::new("Cursor Test")
            //         .default_open(true)
            //         .max_width(1000.0)
            //         .max_height(1000.0)
            //         .resizable(true)
            //         .show(ctx, |ui| {
            //         for &ci in &egui::CursorIcon::ALL {
            //             let _ = ui.button(format!("{ci:?}")).on_hover_cursor(ci);
            //         }
            //     });
            // });

            ui.toggle_value(&mut self.show_cursor_test, "Cursor test");

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.label("                             ");
            });


            ui.add(egui::Slider::new(&mut self.counter, 0..=120).text("age"));

            // ui.vertical_centered(|ui| {
            //     egui::reset_button(ui, self);
            // });
        });

        egui::Window::new("streamline cfd")
            .open(&mut true)
            .default_size([1000.0, 1000.0])
            .vscroll(true)
            .hscroll(true)
            .resizable(true)
            .title_bar(true)
            .collapsible(true)
            .show(ctx, |ui| {
            ui.heading("Try to close the window");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            ui.heading("Top Heading");
            ui.add_space(5.0);
            ui.label("test test test test");
            ui.add_space(15.0);
            ui.label("Sub heading");
            ui.add_space(5.0,);
            ui.label("test test test test");

            ui.add_space(20.0);
            if ui.button("Open File: ").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("pdb", &["pdb"])
                    .pick_file() {
                    let ppath = Some(path.display().to_string());
                    dbg!(&ppath);
                }
            }

            ui.checkbox(
                &mut self.show_immediate_viewport,
                "show immediate child viewport",
            );

            // let mut show_deferred_viewport = self.show_deferred_viewport.load(std::sync::atomic::Ordering::Relaxed);
            // ui.checkbox(&mut show_deferred_viewport, "show deferred child viewport");
            // self.show_deferred_viewport.store(show_deferred_viewport, std::sync::atomic::Ordering::Relaxed);
        });

        if self.show_immediate_viewport {
            // egui::Window::new("streamline cfd")
            //     .default_open(true)
            //     .max_width(1000.0)
            //     .max_height(1000.0)
            //     .default_width(800.0)
            //     .resizable(true)
            //     .show(ctx, |ui| {
            //         ui.label("Hello from immediate viewport");
            //     }
            // );

            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Immediate Viewport")
                    .with_inner_size([200.0, 100.0]),
                |ctx, _class| {
                    // assert!(
                    //     class == egui::ViewportClass::Immediate,
                    //     "This egui backend does not support multiple viewports"
                    // );
                    //
                    egui::Window::new("immediate_viewport")
                        .default_open(false)
                        .max_width(1000.0)
                        .max_height(1000.0)
                        .default_width(800.0)
                        .resizable(true)
                        .show(ctx, |ui| {
                            ui.label("Hello from immediate viewport");
                        }
                    );

                    // egui::CentralPanel::default().show(ctx, |ui| {
                    //     ui.label("Hello from immediate viewport");
                    // });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_immediate_viewport = false;
                    }
                },
            );
        }

        // if self.show_deferred_viewport.load(std::sync::atomic::Ordering::Relaxed) {
        //     let show_deferred_viewport = self.show_deferred_viewport.clone();
        //     ctx.show_viewport_deferred(
        //         egui::ViewportId::from_hash_of("deferred_viewport"),
        //         egui::ViewportBuilder::default()
        //             .with_title("Deferred viewport")
        //             .with_inner_size([200.0, 100.0]),
        //         move |ctx, class| {
        //             // assert!(class == egui::ViewportClass::Deferred,
        //             //         "This egui backend does not support multiple viewports"
        //             // );
        //
        //             // egui::CentralPanel::default().show(ctx, |ui| {
        //             //     ui.label("Hello from deferred viewport");
        //             // });
        //             egui::Window::new("deferred_viewport")
        //                 .default_open(false)
        //                 .max_width(1000.0)
        //                 .max_height(1000.0)
        //                 .default_width(800.0)
        //                 .resizable(true)
        //                 .show(ctx, |ui| {
        //                     ui.label("Hello from deferred viewport");
        //                 }
        //             );
        //
        //             if ctx.input(|i| i.viewport().close_requested()) {
        //                 show_deferred_viewport.store(false, std::sync::atomic::Ordering::Relaxed);
        //             }
        //         },
        //     );
        // }


        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
            dbg!("close request");
        }

        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}

// pub fn GUI(ui: &egui::Context) {
//     // egui::Window::new("Streamline CFD")
//     //     .default_open(true)
//     //     .max_width(1000.0)
//     //     .max_height(1000.0)
//     //     .default_width(800.0)
//     //     .resizable(true)
//     //     .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
//     //     .show(&ui, |mut ui| {
//     //         if ui.add(egui::Button::new("click me")).clicked() {
//     //             println!("ok");
//     //         }
//     //         ui.label("Slider");
//     //         ui.end_row();
//     //     });
//     egui::CentralPanel::default().show(ctx, |ui| {
//         ui.heading("Try to close the window");
//     });
//
//     if ctx.input(|i| i.viewport().close_requested()) {
//         if self.allowed_to_close {
//             // do nothing - we will close
//         } else {
//             ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
//             self.show_confirmation_dialog = true;
//         }
//     }
//
//     if self.show_confirmation_dialog {
//         egui::Window::new("Do you want to quit?")
//             .collapsible(false)
//             .resizable(false)
//             .show(ctx, |ui| {
//                 ui.horizontal(|ui| {
//                     if ui.button("No").clicked() {
//                         self.show_confirmation_dialog = false;
//                         self.allowed_to_close = false;
//                     }
//
//                     if ui.button("Yes").clicked() {
//                         self.show_confirmation_dialog = false;
//                         self.allowed_to_close = true;
//                         ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
//                     }
//                 });
//             });
//     }
// }

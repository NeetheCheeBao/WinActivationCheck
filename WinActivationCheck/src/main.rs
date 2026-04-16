#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use encoding_rs::GBK;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1520.0, 560.0])
            .with_title("WinActivationCheck_1.0.0"),
        ..Default::default()
    };
    eframe::run_native(
        "WinActivationCheck_1.0.0",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    let font_data = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc")
        .unwrap_or_else(|_| std::fs::read("C:\\Windows\\Fonts\\simhei.ttf").unwrap_or_default());

    if !font_data.is_empty() {
        fonts.font_data.insert(
            "cjk_font".to_owned(),
            egui::FontData::from_owned(font_data),
        );

        if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            vec.insert(0, "cjk_font".to_owned());
        }
        if let Some(vec) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            vec.insert(0, "cjk_font".to_owned());
        }

        ctx.set_fonts(fonts);
    }
}

struct MyApp {
    out_dlv: String,
    out_dli: String,
    out_xpr: String,
    tx: Sender<(String, String)>,
    rx: Receiver<(String, String)>,
    show_confirm_dialog: bool,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let (tx, rx) = channel();
        let app = Self {
            out_dlv: String::new(),
            out_dli: String::new(),
            out_xpr: String::new(),
            tx,
            rx,
            show_confirm_dialog: false,
        };
        app.trigger_refresh(&cc.egui_ctx);
        app
    }

    fn trigger_refresh(&self, ctx: &egui::Context) {
        let _ = self.tx.send(("clear_dlv".to_string(), "正在获取 dlv 信息...\n".to_string()));
        let _ = self.tx.send(("clear_dli".to_string(), "正在获取 dli 信息...\n".to_string()));
        let _ = self.tx.send(("clear_xpr".to_string(), "正在获取 xpr 信息...\n".to_string()));

        let tx = self.tx.clone();
        let ctx = ctx.clone();

        thread::spawn(move || {
            Self::run_cmd_sync("-dlv", "dlv", &tx, &ctx);
            Self::run_cmd_sync("-dli", "dli", &tx, &ctx);
            Self::run_cmd_sync("-xpr", "xpr", &tx, &ctx);
        });
    }

    fn run_cmd_sync(arg: &str, target: &str, tx: &Sender<(String, String)>, ctx: &egui::Context) {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        let mut cmd = Command::new("cmd");
        cmd.arg("/c")
           .arg(format!("cscript //nologo C:\\Windows\\System32\\slmgr.vbs {}", arg));
        cmd.creation_flags(CREATE_NO_WINDOW);

        match cmd.output() {
            Ok(output) => {
                let stdout = GBK.decode(&output.stdout).0.into_owned();
                let stderr = GBK.decode(&output.stderr).0.into_owned();
                
                if !stdout.trim().is_empty() {
                    let _ = tx.send((target.to_string(), format!("{}\n", stdout.trim())));
                }
                if !stderr.trim().is_empty() {
                    let _ = tx.send((target.to_string(), format!("[错误]: {}\n", stderr.trim())));
                }
            }
            Err(e) => {
                let _ = tx.send((target.to_string(), format!("[执行失败]: {}\n", e)));
            }
        }
        ctx.request_repaint();
    }

    fn run_clear_and_refresh(&self, ctx: egui::Context) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            
            let mut cmd1 = Command::new("cmd");
            cmd1.arg("/c").arg("cscript //nologo C:\\Windows\\System32\\slmgr.vbs /upk");
            cmd1.creation_flags(CREATE_NO_WINDOW);
            let _ = cmd1.output();

            let mut cmd2 = Command::new("cmd");
            cmd2.arg("/c").arg("cscript //nologo C:\\Windows\\System32\\slmgr.vbs /rearm");
            cmd2.creation_flags(CREATE_NO_WINDOW);
            let _ = cmd2.output();

            let _ = tx.send(("trigger".to_string(), "refresh".to_string()));
            ctx.request_repaint();
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok((target, msg)) = self.rx.try_recv() {
            match target.as_str() {
                "dlv" => {
                    self.out_dlv.clear();
                    self.out_dlv.push_str(&msg);
                }
                "dli" => {
                    self.out_dli.clear();
                    self.out_dli.push_str(&msg);
                }
                "xpr" => {
                    self.out_xpr.clear();
                    self.out_xpr.push_str(&msg);
                }
                "clear_dlv" => self.out_dlv = msg,
                "clear_dli" => self.out_dli = msg,
                "clear_xpr" => self.out_xpr = msg,
                "trigger" => {
                    if msg == "refresh" {
                        self.trigger_refresh(ctx);
                    }
                }
                _ => {}
            }
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("项目地址").clicked() {
                    ui.ctx().output_mut(|o| o.open_url = Some(egui::output::OpenUrl::new_tab("https://github.com/NeetheCheeBao/WinActivationCheck")));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Windows激活状态检查");
            ui.add_space(10.0);

            ui.columns(3, |cols| {
                cols[0].label("-dlv 信息");
                egui::ScrollArea::vertical().id_source("dlv_scroll").show(&mut cols[0], |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.out_dlv)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .interactive(false),
                    );
                });

                cols[1].label("-dli 信息");
                egui::ScrollArea::vertical().id_source("dli_scroll").show(&mut cols[1], |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.out_dli)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .interactive(false),
                    );
                });

                cols[2].label("-xpr 信息");
                egui::ScrollArea::vertical().id_source("xpr_scroll").show(&mut cols[2], |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.out_xpr)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .interactive(false),
                    );
                });
            });

            ui.add_space(20.0);
            
            ui.vertical_centered(|ui| {
                if ui.button("清除激活状态").clicked() {
                    self.show_confirm_dialog = true;
                }
            });
        });

        if self.show_confirm_dialog {
            egui::Window::new("警告")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("确认清除当前激活状态吗？");
                    ui.add_space(15.0);
                    ui.horizontal(|ui| {
                        if ui.button("确认").clicked() {
                            self.show_confirm_dialog = false;
                            self.out_dlv = "正在清除激活状态...\n".to_string();
                            self.out_dli = "正在清除激活状态...\n".to_string();
                            self.out_xpr = "正在清除激活状态...\n".to_string();
                            self.run_clear_and_refresh(ctx.clone());
                        }
                        if ui.button("取消").clicked() {
                            self.show_confirm_dialog = false;
                        }
                    });
                });
        }
    }
}
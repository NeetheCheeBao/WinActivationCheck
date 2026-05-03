#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use native_windows_derive::NwgUi;
use native_windows_gui as nwg;
use native_windows_gui::NativeUi;
use encoding_rs::GBK;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::cell::RefCell;

#[derive(Default, NwgUi)]
pub struct MyApp {
    #[nwg_control(size: (1520, 560), position: (300, 300), title: "WinActivationCheck_1.0.0")]
    #[nwg_events( OnWindowClose: [MyApp::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1, margin: [10, 10, 10, 10])]
    grid: nwg::GridLayout,

    #[nwg_control]
    #[nwg_events( OnNotice: [MyApp::update_ui] )]
    notice: nwg::Notice,

    #[nwg_control(text: "项目地址")]
    menu_url_root: nwg::Menu,

    #[nwg_control(text: "在浏览器打开...", parent: menu_url_root)]
    #[nwg_events( OnMenuItemSelected: [MyApp::open_url] )]
    menu_item_url: nwg::MenuItem,

    #[nwg_control(text: "-dlv 信息")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 1)]
    lbl_dlv: nwg::Label,
    
    #[nwg_control(text: "", readonly: true, flags: "VISIBLE|VSCROLL|AUTOVSCROLL")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 25)]
    out_dlv: nwg::TextBox,

    #[nwg_control(text: "-dli 信息")]
    #[nwg_layout_item(layout: grid, col: 1, row: 0, row_span: 1)]
    lbl_dli: nwg::Label,
    
    #[nwg_control(text: "", readonly: true, flags: "VISIBLE|VSCROLL|AUTOVSCROLL")]
    #[nwg_layout_item(layout: grid, col: 1, row: 1, row_span: 25)]
    out_dli: nwg::TextBox,

    #[nwg_control(text: "-xpr 信息")]
    #[nwg_layout_item(layout: grid, col: 2, row: 0, row_span: 1)]
    lbl_xpr: nwg::Label,
    
    #[nwg_control(text: "", readonly: true, flags: "VISIBLE|VSCROLL|AUTOVSCROLL")]
    #[nwg_layout_item(layout: grid, col: 2, row: 1, row_span: 25)]
    out_xpr: nwg::TextBox,

    #[nwg_control(text: "清除激活状态")]
    #[nwg_layout_item(layout: grid, col: 1, row: 27, row_span: 3)]
    #[nwg_events( OnButtonClick: [MyApp::clear_status] )]
    btn_clear: nwg::Button,

    tx: RefCell<Option<Sender<(String, String)>>>,
    rx: RefCell<Option<Receiver<(String, String)>>>,
}

impl MyApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn open_url(&self) {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c").arg("start https://github.com/NeetheCheeBao/WinActivationCheck");
        cmd.creation_flags(0x08000000);
        let _ = cmd.spawn();
    }

    fn clear_status(&self) {
        let params = nwg::MessageParams {
            title: "警告",
            content: "确认清除当前激活状态吗？",
            buttons: nwg::MessageButtons::OkCancel,
            icons: nwg::MessageIcons::Warning,
        };
        if nwg::modal_message(&self.window, &params) == nwg::MessageChoice::Ok {
            self.out_dlv.set_text("正在清除激活状态...\r\n");
            self.out_dli.set_text("正在清除激活状态...\r\n");
            self.out_xpr.set_text("正在清除激活状态...\r\n");
            self.run_clear_and_refresh();
        }
    }

    fn update_ui(&self) {
        if let Some(rx) = self.rx.borrow().as_ref() {
            while let Ok((target, msg)) = rx.try_recv() {
                match target.as_str() {
                    "dlv" => self.out_dlv.set_text(&format!("{}{}", self.out_dlv.text(), msg)),
                    "dli" => self.out_dli.set_text(&format!("{}{}", self.out_dli.text(), msg)),
                    "xpr" => self.out_xpr.set_text(&format!("{}{}", self.out_xpr.text(), msg)),
                    "clear_dlv" => self.out_dlv.set_text(&msg),
                    "clear_dli" => self.out_dli.set_text(&msg),
                    "clear_xpr" => self.out_xpr.set_text(&msg),
                    "trigger" => {
                        if msg == "refresh" {
                            self.trigger_refresh();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn trigger_refresh(&self) {
        if let Some(tx) = self.tx.borrow().as_ref() {
            let _ = tx.send(("clear_dlv".to_string(), "正在获取 dlv 信息...\r\n".to_string()));
            let _ = tx.send(("clear_dli".to_string(), "正在获取 dli 信息...\r\n".to_string()));
            let _ = tx.send(("clear_xpr".to_string(), "正在获取 xpr 信息...\r\n".to_string()));
            self.notice.sender().notice();

            let tx_clone = tx.clone();
            let notice_sender = self.notice.sender();
            thread::spawn(move || {
                Self::run_cmd_sync("-dlv", "dlv", &tx_clone, &notice_sender);
                Self::run_cmd_sync("-dli", "dli", &tx_clone, &notice_sender);
                Self::run_cmd_sync("-xpr", "xpr", &tx_clone, &notice_sender);
            });
        }
    }

    fn run_cmd_sync(arg: &str, target: &str, tx: &Sender<(String, String)>, notice: &nwg::NoticeSender) {
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
                    let _ = tx.send((target.to_string(), format!("{}\r\n", stdout.trim())));
                }
                if !stderr.trim().is_empty() {
                    let _ = tx.send((target.to_string(), format!("[错误]: {}\r\n", stderr.trim())));
                }
            }
            Err(e) => {
                let _ = tx.send((target.to_string(), format!("[执行失败]: {}\r\n", e)));
            }
        }
        notice.notice();
    }

    fn run_clear_and_refresh(&self) {
        if let Some(tx) = self.tx.borrow().as_ref() {
            let tx_clone = tx.clone();
            let notice_sender = self.notice.sender();
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

                let _ = tx_clone.send(("trigger".to_string(), "refresh".to_string()));
                notice_sender.notice();
            });
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Microsoft YaHei")
        .size(18)
        .weight(400)
        .build(&mut font)
        .ok();
    nwg::Font::set_global_default(Some(font));

    let (tx, rx) = channel();
    
    let app = MyApp {
        tx: RefCell::new(Some(tx)),
        rx: RefCell::new(Some(rx)),
        ..Default::default()
    };
    
    let ui = MyApp::build_ui(app).expect("Failed to build UI");
    
    ui.trigger_refresh();
    
    nwg::dispatch_thread_events();
}
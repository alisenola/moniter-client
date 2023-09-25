use winsafe::{prelude::*, gui};
use winsafe::co::ES;
use std::fs;
use regex::Regex;
use std::path::PathBuf;

use crate::utils::*;

#[derive(Clone)]
pub struct DocumentWindow {
    pub wnd:       gui::WindowMain,
    txt_content: gui::Edit,
    txt_password: gui::Edit,
    btn_load: gui::Button,
}

impl DocumentWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Document".to_owned(),
                size: (800, 600),
                ..Default::default()
            },
        );

        let txt_content = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 20),
				width: 760,
                height: 500,
                edit_style: ES::MULTILINE,
				..Default::default()
			},
		);

        let txt_password = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (500, 550),
				width: 120,
                edit_style: ES::PASSWORD,
				..Default::default()
			},
		);

        let btn_load = gui::Button::new(
            &wnd,
            gui::ButtonOpts {
                text: "&Load".to_owned(),
                position: (700, 550),
                ..Default::default()
            },
        );

        let new_self = Self { wnd, btn_load, txt_content, txt_password };
        new_self.events();
        new_self
    }

    fn events(&self) {
        let self2 = self.clone();
        self.btn_load.on().bn_clicked(move || {
            let password = self2.txt_password.text();
            let mut path = PathBuf::from("D:\\");
            path.push("_documents");
            path.push("logs");
            let paths = fs::read_dir(path).unwrap();
            for path in paths {
                let p = path.unwrap().path().display().to_string();
                let rf = Regex::new(r"\d{4}-\d{2}-\d{2}.zip").unwrap();
                if rf.is_match(&p) {
                    let logs: String = read_logs_with_password(&p, "log.txt", password.as_bytes());
                    self2.txt_content.set_text(&logs)
                }
            }
            Ok(())
        });
    }
}
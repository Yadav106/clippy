use core::time;
use std::process;
use std::thread;

use clipboard::{ClipboardContext, ClipboardProvider};

pub struct ClipboardHistory {
    pub contents: Vec<String>,
}

impl ClipboardHistory {
    pub fn new() -> ClipboardHistory {
        ClipboardHistory {
            contents: Vec::new(),
        }
    }

    pub fn get_clipboard_data(&mut self) {
        loop {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            let current_content = ctx.get_contents().unwrap_or_else(|_err| {
                eprint!("Error while getting the clipboard contents");
                process::exit(1);
            });

            let last_copied_element = self.contents.last().cloned();
            // println!("{}", current_content);
            if let Some(last_element) = last_copied_element {
                if last_element != current_content {
                    self.contents.push(current_content);
                }
            } else {
                self.contents.push(current_content);
            }

            println!("{:?}", self.contents);
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

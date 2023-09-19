use std::{collections::vec_deque::VecDeque, io::Write};

use egui::{ScrollArea, Ui, Color32, RichText, TextEdit};
use std::sync::mpsc::{channel, Receiver, Sender};

const MAX_MESSAGES: usize = 256;

struct RichMsg{
    pub msg: String,
    pub color: Color32
}

pub struct MyLogger {
    messages: VecDeque<RichMsg>,
    rx: Receiver<String>,
}

#[derive(Clone)]
pub struct LogWriter {
    tx: Sender<String>,
    curr_msg: String,
}


impl Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.curr_msg
            .push_str(String::from_utf8_lossy(buf).as_ref());
        if buf.contains(&b'\n') {
            self.flush().ok();
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut msg = self.curr_msg.clone();
        if msg.ends_with('\n') {
            msg.pop();
        }
        self.tx.send(msg).ok();
        self.curr_msg.clear();
        Ok(())
    }
}

pub fn new_logger() -> (MyLogger, LogWriter) {
    let (tx, rx) = channel();
    (
        MyLogger {
            messages: Default::default(),
            rx,
        },
        LogWriter {
            tx,
            curr_msg: String::new(),
        },
    )
}

impl MyLogger {
    pub fn show_log(&mut self, ui: &mut Ui) {
        while let Ok(msg) = self.rx.try_recv() {
            let (prefix, msg) = msg.split_at(msg.find(']').map_or(0, |i|i+2));
            let mut color = ui.visuals().text_color();
            if prefix.contains("[ERROR]"){
                color = ui.visuals().error_fg_color;
            }
            else if prefix.contains("[WARN]"){
                color = ui.visuals().warn_fg_color;
            }
            else if prefix.contains("[DEBUG]"){
                color = ui.visuals().weak_text_color();
            }
            self.messages.push_back(RichMsg{ msg: msg.to_owned(), color});
        }
        while self.messages.len() > MAX_MESSAGES {
            self.messages.pop_front();
        }
        let rh = ui.text_style_height(&egui::TextStyle::Body);

        ScrollArea::vertical().show_rows(ui, rh, self.messages.len(), |ui, range| {
            for msg in &mut self.messages.range(range) {
                ui.add(TextEdit::multiline(&mut msg.msg.as_str()).desired_rows(1).text_color(msg.color));
            }
        });
    }
}

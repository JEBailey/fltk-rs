use fltk::{app, text::*, window::*};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
struct Term {
    pub term: TextDisplay,
    current_dir: String,
    cmd: String,
}

impl Term {
    pub fn new(buf: TextBuffer) -> Term {
        let mut current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();

        current_dir.push_str("$ ");

        let mut t = Term {
            term: TextDisplay::new(5, 5, 630, 470, ""),
            current_dir,
            cmd: String::from(""),
        };

        t.set_buffer(Some(buf));
        t
    }

    pub fn style(&mut self) {
        self.term.set_color(Color::Black);
        self.term.set_text_color(Color::Green);
        self.term.set_text_font(Font::Courier);
        self.term.set_cursor_color(Color::Green);
        self.term.set_cursor_style(TextCursor::Block);
        self.term.show_cursor(true);
    }

    fn append(&mut self, txt: &str) {
        self.term.buffer().unwrap().append(txt);
        self.term
            .set_insert_position(self.term.buffer().unwrap().length());
        self.term.scroll(
            self.term
                .count_lines(0, self.term.buffer().unwrap().length(), true),
            0,
        );
    }

    fn run_command(&mut self) -> String {
        let args = self.cmd.clone();
        let args: Vec<&str> = args.split_whitespace().collect();

        if !args.is_empty() {
            let mut cmd = Command::new(args[0]);
            if args.len() > 1 {
                if args[0] == "cd" {
                    let path = args[1];
                    return self.change_dir(&PathBuf::from(path));
                } else {
                    cmd.args(&args[1..]);
                }
            }
            let out = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            if out.is_err() {
                let msg = format!("{}: command not found!\n", self.cmd);
                msg
            } else {
                let stdout = out.unwrap().stdout;
                let stdout = String::from_utf8_lossy(&stdout).to_string();
                stdout
            }
        } else {
            String::from("")
        }
    }

    pub fn change_dir(&mut self, path: &Path) -> String {
        if path.exists() && path.is_dir() {
            std::env::set_current_dir(path).unwrap();
            let mut current_dir = std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
            current_dir.push_str("$ ");
            self.current_dir = current_dir;
            String::from("")
        } else {
            String::from("Path does not exist!\n")
        }
    }
}

impl Deref for Term {
    type Target = TextDisplay;

    fn deref(&self) -> &Self::Target {
        &self.term
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.term
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Plastic);
    let mut wind = Window::new(100, 100, 640, 480, "Rusty Terminal");
    let buf = TextBuffer::default();

    let mut term = Term::new(buf);
    term.style();

    let dir = term.current_dir.clone();
    term.append(&dir);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let mut term_c = term.clone();
    term_c.handle(Box::new(move |ev| {
        // println!("{:?}", app::event());
        // println!("{:?}", app::event_key());
        // println!("{:?}", app::event_text());
        match ev {
            Event::KeyDown => match app::event_key() {
                Key::Enter => {
                    term.append("\n");
                    let out = term.run_command();
                    term.append(&out);
                    let current_dir = term.current_dir.clone();
                    term.append(&current_dir);
                    term.cmd.clear();
                    true
                }
                Key::BackSpace => {
                    if !term.cmd.is_empty() {
                        let text_len = term.buffer().unwrap().text().len() as u32;
                        term.term
                            .buffer()
                            .unwrap()
                            .remove(text_len - 1, text_len as u32);
                        term.cmd.pop().unwrap();
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    let temp = app::event_text();
                    term.cmd.push_str(&temp);
                    term.append(&temp);
                    true
                }
            },
            _ => false,
        }
    }));

    app.run().unwrap();
}

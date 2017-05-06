use bindgen::config::{Config, Braces};
use std::io::Write;

pub struct Writer<'a, 'f, F: 'f + Write> {
    out: &'f mut F,
    config: &'a Config,
    spaces: Vec<usize>,
    line_started: bool,
    line_length: usize,
}

impl<'a, 'f, F: Write> Writer<'a, 'f, F> {
    pub fn new(out: &'f mut F, config: &'a Config) -> Writer<'a, 'f, F> {
        Writer {
            out: out,
            config: config,
            spaces: vec![0],
            line_started: false,
            line_length: 0,
        }
    }

    fn spaces(&mut self) -> usize {
        *self.spaces.last().unwrap()
    }


    fn push_set_spaces(&mut self, spaces: usize) {
        self.spaces.push(spaces);
    }

    pub fn push_tab(&mut self) {
        let spaces = self.spaces() -
                     (self.spaces() % self.config.tab_width) + 
                     self.config.tab_width;
        self.spaces.push(spaces);
    }

    pub fn pop_tab(&mut self) {
        assert!(!self.spaces.is_empty());
        self.spaces.pop();
    }

    pub fn new_line(&mut self) {
        write!(self.out, "\n").unwrap();
        self.line_started = false;
        self.line_length = 0;
    }

    pub fn open_brace(&mut self) {
        match self.config.braces {
            Braces::SameLine => {
                self.write(" {");
                self.push_tab();
                self.new_line();
            }
            Braces::NextLine => {
                self.new_line();
                self.write("{");
                self.push_tab();
                self.new_line();
            }
        }
    }

    pub fn close_brace(&mut self, semicolon: bool) {
        self.pop_tab();
        self.new_line();
        if semicolon {
            self.write("};");
        } else {
            self.write("}");
        }
    }

    pub fn write(&mut self, text: &str) {
        if !self.line_started {
            for _ in 0..self.spaces() {
                write!(self.out, " ").unwrap();
            }
            self.line_started = true;
            self.line_length += self.spaces();
        }

        write!(self.out, "{}", text).unwrap();
        self.line_length += text.len();
    }

    pub fn write_aligned_list(&mut self, items: Vec<String>, join: String) {
        let align_length = self.line_length;
        self.push_set_spaces(align_length);
        for (i, item) in items.iter().enumerate() {
            self.write(item);
            if i != items.len() - 1 {
                self.write(&join);
                self.new_line();
            }
        }
        self.pop_tab();
    }
}

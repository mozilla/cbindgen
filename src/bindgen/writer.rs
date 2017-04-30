use bindgen::config::{Config, Braces};
use std::io::Write;

pub struct Writer<'a, 'f, F: 'f + Write> {
    out: &'f mut F,
    config: &'a Config,
    tabs: Vec<u32>,
    pending_spaces: u32,
    has_written: bool,
}

impl<'a, 'f, F: Write> Writer<'a, 'f, F> {
    pub fn new(out: &'f mut F, config: &'a Config) -> Writer<'a, 'f, F> {
        Writer {
            out: out,
            config: config,
            tabs: vec![0],
            pending_spaces: 0,
            has_written: false,
        }
    }

    fn tabs(&mut self) -> u32 {
        *self.tabs.last().unwrap()
    }

    pub fn push_tab(&mut self) {
        let tabs = self.tabs() + 1;
        self.tabs.push(tabs);

        if !self.has_written {
            self.pending_spaces = self.tabs() * self.config.tab_width;
        }
    }

    pub fn pop_tab(&mut self) {
        assert!(!self.tabs.is_empty());
        self.tabs.pop();

        if !self.has_written {
            self.pending_spaces = self.tabs() * self.config.tab_width;
        }
    }

    pub fn new_line(&mut self) {
        write!(self.out, "\n").unwrap();
        self.pending_spaces = self.tabs() * self.config.tab_width;
        self.has_written = false;
    }

    pub fn open_brace(&mut self) {
        match self.config.braces {
            Braces::SameLine => {
                write!(self.out, " {{").unwrap();
                self.push_tab();
                self.new_line();
            }
            Braces::NextLine => {
                self.new_line();
                write!(self.out, "{{").unwrap();
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
        for _ in 0..self.pending_spaces {
            write!(self.out, " ").unwrap();
        }
        self.pending_spaces = 0;
        self.has_written = true;

        write!(self.out, "{}", text).unwrap()
    }
}

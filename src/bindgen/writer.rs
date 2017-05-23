use std::cmp;
use std::io;
use std::io::Write;

use bindgen::config::{Config, Braces};

pub enum ListType<'a> {
    Join(&'a str),
    Cap(&'a str),
}

pub struct NullFile;
impl Write for NullFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A utility writer for generating code easier.
pub struct SourceWriter<'a, F: Write> {
    out: F,
    config: &'a Config,
    spaces: Vec<usize>,
    line_started: bool,
    line_length: usize,
    line_number: usize,
    max_line_length: usize,
}
pub type MeasureWriter<'a> = SourceWriter<'a, NullFile>;

impl<'a, F: Write> SourceWriter<'a, F> {
    pub fn new(out: F, config: &'a Config) -> SourceWriter<'a, F> {
        SourceWriter {
            out: out,
            config: config,
            spaces: vec![0],
            line_started: false,
            line_length: 0,
            line_number: 1,
            max_line_length: 0,
        }
    }

    pub fn measure<T>(&self, func: T) -> usize
        where T: Fn(&mut MeasureWriter)
    {
        let mut measurer = SourceWriter {
            out: NullFile,
            config: self.config,
            spaces: self.spaces.clone(),
            line_started: self.line_started,
            line_length: self.line_length,
            line_number: self.line_number,
            max_line_length: self.line_length,
        };

        func(&mut measurer);

        measurer.max_line_length
    }

    fn spaces(&self) -> usize {
        *self.spaces.last().unwrap()
    }

    fn push_set_spaces(&mut self, spaces: usize) {
        self.spaces.push(spaces);
    }

    fn line_length_for_align(&self) -> usize {
        if self.line_started {
            self.line_length
        } else {
            self.line_length + self.spaces()
        }
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
        self.line_number += 1;
    }

    pub fn new_line_if_not_start(&mut self) {
        if self.line_number != 1 {
            self.new_line();
        }
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
        self.max_line_length = cmp::max(self.max_line_length, self.line_length);
    }

    pub fn write_vertical_list<'b>(&mut self, items: &Vec<String>, list_type: ListType<'b>) {
        let align_length = self.line_length_for_align();
        self.push_set_spaces(align_length);
        for (i, item) in items.iter().enumerate() {
            self.write(&item);

            match list_type {
                ListType::Join(text) => {
                    if i != items.len() - 1 {
                        self.write(&text);
                    }
                }
                ListType::Cap(text) => {
                    self.write(&text);
                }
            }

            if i != items.len() - 1 {
                self.new_line();
            }
        }
        self.pop_tab();
    }

    pub fn write_vertical_source_list<'b, S: Source>(&mut self, items: &Vec<S>, list_type: ListType<'b>) {
        let align_length = self.line_length_for_align();
        self.push_set_spaces(align_length);
        for (i, ref item) in items.iter().enumerate() {
            item.write(self.config, self);

            match list_type {
                ListType::Join(text) => {
                    if i != items.len() - 1 {
                        self.write(&text);
                    }
                }
                ListType::Cap(text) => {
                    self.write(&text);
                }
            }

            if i != items.len() - 1 {
                self.new_line();
            }
        }
        self.pop_tab();
    }
}

pub trait Source {
    fn write<F: Write>(&self, config: &Config, &mut SourceWriter<F>);
}

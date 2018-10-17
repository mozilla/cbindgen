/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cmp;
use std::io;
use std::io::Write;

use bindgen::config::{Braces, Config};

/// A type of way to format a list.
pub enum ListType<'a> {
    /// Join each adjacent item with a str.
    Join(&'a str),
    /// End each item with a str.
    Cap(&'a str),
}

/// An empty file used for creating a null source writer and measuring line
/// metrics for various code layouts.
pub struct NullFile;
impl Write for NullFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A utility wrapper to write unbuffered data and correctly adjust positions.
struct InnerWriter<'a, 'b: 'a, F: 'a + Write>(&'a mut SourceWriter<'b, F>);

impl<'a, 'b, F: Write> Write for InnerWriter<'a, 'b, F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let writer = &mut self.0;

        if !writer.line_started {
            for _ in 0..writer.spaces() {
                write!(writer.out, " ").unwrap();
            }
            writer.line_started = true;
            writer.line_length += writer.spaces();
        }

        let written = writer.out.write(buf)?;
        writer.line_length += written;
        writer.max_line_length = cmp::max(writer.max_line_length, writer.line_length);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.out.flush()
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

    /// Takes a function that writes source and returns the maximum line length
    /// written.
    pub fn measure<T>(&self, func: T) -> usize
    where
        T: Fn(&mut MeasureWriter),
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

    pub fn push_set_spaces(&mut self, spaces: usize) {
        self.spaces.push(spaces);
    }

    pub fn line_length_for_align(&self) -> usize {
        if self.line_started {
            self.line_length
        } else {
            self.line_length + self.spaces()
        }
    }

    pub fn push_tab(&mut self) {
        let spaces =
            self.spaces() - (self.spaces() % self.config.tab_width) + self.config.tab_width;
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

    pub fn write(&mut self, text: &'static str) {
        write!(self, "{}", text);
    }

    pub fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) {
        InnerWriter(self).write_fmt(fmt).unwrap();
    }

    pub fn write_horizontal_source_list<'b, S: Source>(
        &mut self,
        items: &[S],
        list_type: ListType<'b>,
    ) {
        for (i, ref item) in items.iter().enumerate() {
            item.write(self.config, self);

            match list_type {
                ListType::Join(text) => {
                    if i != items.len() - 1 {
                        write!(self, "{}", text);
                    }
                }
                ListType::Cap(text) => {
                    write!(self, "{}", text);
                }
            }
        }
    }

    pub fn write_vertical_source_list<'b, S: Source>(
        &mut self,
        items: &[S],
        list_type: ListType<'b>,
    ) {
        let align_length = self.line_length_for_align();
        self.push_set_spaces(align_length);
        for (i, ref item) in items.iter().enumerate() {
            item.write(self.config, self);

            match list_type {
                ListType::Join(text) => {
                    if i != items.len() - 1 {
                        write!(self, "{}", text);
                    }
                }
                ListType::Cap(text) => {
                    write!(self, "{}", text);
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

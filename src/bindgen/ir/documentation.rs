/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use crate::bindgen::config::{Config, DocumentationLength, DocumentationStyle, Language};
use crate::bindgen::utilities::SynAttributeHelpers;
use crate::bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Documentation {
    pub doc_comment: Vec<String>,
}

impl Documentation {
    pub fn load(attrs: &[syn::Attribute]) -> Self {
        let attrs_lines = attrs.get_comment_lines();
        let annotation_lines: Vec<usize> = attrs_lines
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                let line = line.trim_start();
                if !line.starts_with("cbindgen:") {
                    return None;
                }

                Some(i)
            })
            .collect();

        let mut skip_lines = annotation_lines.clone();

        annotation_lines.iter().for_each(|line_index| {
            for i in *line_index..attrs_lines.len() - 1 {
                if !attrs_lines[i].ends_with('\\') {
                    break;
                }
                skip_lines.push(i);
            }
        });

        let doc = attrs
            .get_comment_lines()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !skip_lines.contains(i))
            .map(|(_, l)| l)
            .collect();

        Documentation { doc_comment: doc }
    }

    pub fn simple(line: &str) -> Self {
        Documentation {
            doc_comment: vec![line.to_owned()],
        }
    }

    pub fn none() -> Self {
        Documentation {
            doc_comment: Vec::new(),
        }
    }
}

impl Source for Documentation {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if self.doc_comment.is_empty() || !config.documentation {
            return;
        }

        let end = match config.documentation_length {
            DocumentationLength::Short => 1,
            DocumentationLength::Full => self.doc_comment.len(),
        };

        // Cython uses Python-style comments, so `documentation_style` is not relevant.
        if config.language == Language::Cython {
            for line in &self.doc_comment[..end] {
                write!(out, "#{}", line);
                out.new_line();
            }
            return;
        }

        let style = match config.documentation_style {
            DocumentationStyle::Auto if config.language == Language::C => DocumentationStyle::Doxy,
            DocumentationStyle::Auto if config.language == Language::Cxx => DocumentationStyle::Cxx,
            DocumentationStyle::Auto => DocumentationStyle::C, // Fallback if `Language` gets extended.
            other => other,
        };

        // Following these documents for style conventions:
        // https://en.wikibooks.org/wiki/C++_Programming/Code/Style_Conventions/Comments
        // https://www.cs.cmu.edu/~410/doc/doxygen.html
        match style {
            DocumentationStyle::C => {
                out.write("/*");
                out.new_line();
            }

            DocumentationStyle::Doxy => {
                out.write("/**");
                out.new_line();
            }

            _ => (),
        }

        for line in &self.doc_comment[..end] {
            match style {
                DocumentationStyle::C => out.write(""),
                DocumentationStyle::Doxy => out.write(" *"),
                DocumentationStyle::C99 => out.write("//"),
                DocumentationStyle::Cxx => out.write("///"),
                DocumentationStyle::Auto => unreachable!(), // Auto case should always be covered
            }

            write!(out, "{}", line);
            out.new_line();
        }

        match style {
            DocumentationStyle::C => {
                out.write(" */");
                out.new_line();
            }

            DocumentationStyle::Doxy => {
                out.write(" */");
                out.new_line();
            }

            _ => (),
        }
    }
}

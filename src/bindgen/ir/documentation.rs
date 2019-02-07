/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::utilities::SynAttributeHelpers;
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Documentation {
    pub doc_comment: Vec<String>,
}

impl Documentation {
    pub fn load(attrs: &[syn::Attribute]) -> Self {
        let doc = attrs
            .get_comment_lines()
            .into_iter()
            .filter(|x| !x.is_empty() && !x.starts_with("cbindgen:"))
            .collect();

        Documentation { doc_comment: doc }
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

        if config.language == Language::C {
            out.write("/**");
            out.new_line();
        }
        for line in &self.doc_comment {
            if config.language != Language::C {
                out.write("///");
            } else {
                out.write(" *");
            }
            if line.len() != 0 {
                out.write(" ");
            }
            write!(out, "{}", line);
            out.new_line();
        }
        if config.language == Language::C {
            out.write(" */");
            out.new_line();
        }
    }
}

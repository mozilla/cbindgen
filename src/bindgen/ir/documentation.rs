/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Documentation {
    pub doc_comment: Vec<String>
}

impl Documentation {
    pub fn load(attrs: &Vec<syn::Attribute>) -> Self {
        let mut doc = Vec::new();

        for attr in attrs {
            if attr.style == syn::AttrStyle::Outer &&
               attr.is_sugared_doc {
                if let syn::MetaItem::NameValue(_, syn::Lit::Str(ref comment, _)) = attr.value {
                    let line = comment.trim_left_matches("///").trim();

                    if !line.starts_with("cbindgen:") {
                        doc.push(line.to_owned());
                    }
                }
            }
        }

        Documentation {
            doc_comment: doc,
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

        if config.language == Language::C {
            out.write("/*");
            out.new_line();
        }
        for line in &self.doc_comment {
            if config.language != Language::C {
                out.write("// ");
            } else {
                out.write(" * ");
            }
            out.write(line);
            out.new_line();
        }
        if config.language == Language::C {
            out.write(" */");
            out.new_line();
        }
    }
}

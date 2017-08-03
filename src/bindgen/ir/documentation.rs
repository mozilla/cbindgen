/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use bindgen::writer::{Source, SourceWriter};
use bindgen::config::Config;

#[derive(Debug, Clone)]
pub struct Documentation {
    pub doc_comment: Vec<String>
}

impl Documentation {
    pub fn load(doc: String) -> Self {
        let doc = doc.lines().filter_map(|x|{
            let x = x.trim_left_matches("///");
            if x.trim().starts_with("cbindgen:") {
                None
            } else {
                Some(x.into())
            }
        }).collect();
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
    fn write<F: Write>(&self,config: &Config, out: &mut SourceWriter<F>) {
        if self.doc_comment.is_empty() || !config.documentation {
            return;
        }
        for line in &self.doc_comment {
            out.write("//");
            out.write(line);
            out.new_line();
        }
    }
}

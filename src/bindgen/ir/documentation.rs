/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::utilities::SynAttributeHelpers;

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

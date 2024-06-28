/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashSet;

use crate::bindgen::ir::{Function, FunctionAbi};

#[derive(Debug, Clone, Default)]
/// Loosely-defined set of things that may need to be defined at the top of a file.
/// Motivation is whether to emit compiler-specific calling conventions based on
/// whether they are used.
pub struct Predefines {
    calling_conventions: HashSet<FunctionAbi>,
}

impl Predefines {
    pub(crate) fn new(functions: &[Function]) -> Self {
        Self {
            calling_conventions: functions
                .iter()
                .map(|f| &f.abi)
                .collect::<HashSet<_>>()
                .into_iter()
                .cloned()
                .collect(),
        }
    }

    pub(crate) fn calling_conventions(&self) -> &HashSet<FunctionAbi> {
        &self.calling_conventions
    }
}

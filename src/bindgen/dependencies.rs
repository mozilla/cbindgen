/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use std::fmt::{self, Display};
use std::collections::HashMap;

use petgraph::{Graph, Direction};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

use bindgen::ir::{Function, OpaqueItem, Item, ItemKind};
use bindgen::library::Library;
use bindgen::config::Config;

const ITEM_ORDER: [ItemKind; 6] = [ItemKind::Enum,
                                   ItemKind::OpaqueItem,
                                   ItemKind::Struct,
                                   ItemKind::Typedef,
                                   ItemKind::Function,
                                   ItemKind::Specialization];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DependencyKind {
    Ptr,
    Normal,
}

impl Display for DependencyKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DependencyKind::Ptr => write!(f, "Ptr"),
            DependencyKind::Normal => write!(f, "Normal"),
        }
    }
}

/// A dependency list is used for gathering what order to output the types.
pub struct DependencyList {
    graph: Graph<Item, DependencyKind>,
    lookup: HashMap<Item, NodeIndex>,
}

impl DependencyList {
    pub fn new(functions: &[Function], library: &Library, config: &Config) -> Self {
        let mut d = DependencyList {
            graph: Graph::new(),
            lookup: HashMap::new(),
        };
        for f in functions {
            d.add_dep(Item::Function(f.clone()), library, config);
        }
        d
    }

    fn add_dep(&mut self, mut item: Item, library: &Library, config: &Config) {
        if !self.lookup.contains_key(&item) {
            item.apply_transformation(config);
            let idx = self.graph.add_node(item.clone());
            self.lookup.insert(item.clone(), idx);
            let deps = item.get_deps(library);
            for &(ref d, _) in &deps {
                self.add_dep(d.clone(), library, config);
            }
            for (d, k) in deps {
                if let Some(to_id) = self.lookup.get(&d) {
                    match d {
                        Item::Specialization(ref s) if !s.generic_values.is_empty() => {
                            self.graph.add_edge(*to_id, idx, k);
                        }
                        _ => {
                            self.graph.add_edge(idx, *to_id, k);
                        }
                    }
                } else {
                    println!("Did not found {:?}", d);
                    panic!();
                }
            }
        }
    }


    // It's there for debugging
    #[allow(dead_code)]
    pub fn print(&self) {
        use petgraph::dot::Dot;
        println!("{}", Dot::new(&self.graph));
    }

    fn generate_opaque_item(
        &self,
        id: NodeIndex,
        o: OpaqueItem,
        ret: &mut Vec<Item>,
    ) -> Option<NodeIndex> {
        // It is possible to have multiple edges with different
        // dependencies between nodes, so we need to group the edges by
        // theire source
        let mut edges = HashMap::new();
        for e in self.graph.edges_directed(id, Direction::Incoming) {
            edges.entry(e.source()).or_insert_with(Vec::new).push(e);
        }
        // We would only remove edges with a ptr dependency between nodes
        // by injecting a opaque wrapper
        let edges = edges
            .values()
            .filter(|edges| {
                edges.iter().all(|e| e.weight() == &DependencyKind::Ptr)
            })
            .collect::<Vec<_>>();
        // If there is node ptr dependency we are done here
        if edges.is_empty() {
            None
        } else {
            ret.push(Item::OpaqueItem(o));
            Some(id)
        }
    }

    fn remove_cycle(&mut self, id: NodeIndex, ret: &mut Vec<Item>) {
        let nid = {
            let node = self.graph.node_weight(id).expect("Got id from graph above");
            match *node {
                Item::Struct(ref s) => self.generate_opaque_item(id, s.as_opaque(), ret),
                _ => return,
            }
        };
        if let Some(nid) = nid {
            // We could not simply remove all edges in a given list here
            // because the edge indices may change on removal
            // Because of the borrow checker we could also not use
            // a while let loop here…
            let mut skip_counter = 0;
            loop {
                let id = if let Some(e) = self.graph
                    .edges_directed(nid, Direction::Incoming)
                    .skip(skip_counter)
                    .next()
                {
                    if *e.weight() == DependencyKind::Ptr {
                        e.id()
                    } else {
                        // Ignore edges with DependencyKind::Normal
                        skip_counter += 1;
                        continue;
                    }
                } else {
                    break;
                };
                self.graph.remove_edge(id);
            }
        }
    }

    pub fn calculate_order(mut self) -> Vec<Item> {
        let mut ret = Vec::new();
        let mut cycle_counter = 0;
        while self.graph.node_count() > 0 {
            let mut all_empty = true;
            let mut pos = 0;
            while pos < ITEM_ORDER.len() {
                let current_item_kind = ITEM_ORDER[pos];
                // find structs without any dependency
                let externals = self.graph
                    .externals(Direction::Outgoing)
                    .filter(|idx| {
                        self.graph.node_weight(*idx)
                            .expect("Node is there because we got the id from the graph above")
                            == current_item_kind
                    })
                    .collect::<Vec<_>>();
                if externals.is_empty() {
                    pos += 1;
                } else {
                    pos = 0;
                    all_empty = false;
                    cycle_counter = 0;
                    // Iterate over all nodes without dependency
                    // 1. Remove them from the graph
                    // 2. Push them to the orderd struct list
                    for idx in externals {
                        if let Some(mut s) = self.graph.remove_node(idx) {
                            s.mangle_paths();
                            ret.push(s);
                        }
                    }
                }
            }
            if all_empty {
                if cycle_counter >= self.graph.node_count() {
                    self.print();
                    panic!("Could not remove cycle");
                }
                // there is a cyclic graph left, so we add a struct as opaque
                // item and remove some edge from the dependceny graph
                let id = self.graph
                    .node_indices()
                    .skip(cycle_counter)
                    .next()
                    .expect("Graph is not empty");
                self.remove_cycle(id, &mut ret);
                cycle_counter += 1;
            }
        }
        ret
    }
}

//! Substrate to SCIR conversion.

use std::collections::{HashMap, HashSet};

use opacity::Opacity;
use scir::{Cell, CellId as ScirCellId, CellInner, Instance, Library};

use crate::io::Node;

use super::{CellId, RawCell};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum ExportAsTestbench {
    No,
    Yes,
}

impl ExportAsTestbench {
    pub fn as_bool(&self) -> bool {
        match *self {
            Self::No => false,
            Self::Yes => true,
        }
    }
}

impl From<bool> for ExportAsTestbench {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}

impl RawCell {
    /// Export this cell and all subcells as a SCIR library.
    pub(crate) fn to_scir_lib(&self, testbench: ExportAsTestbench) -> scir::Library {
        let mut lib = Library::new(self.name.clone());
        let mut cells = HashMap::new();
        let id = self.to_scir_cell(&mut lib, &mut cells);
        lib.set_top(id, testbench.as_bool());
        lib
    }

    fn to_scir_cell(
        &self,
        lib: &mut Library,
        cells: &mut HashMap<CellId, ScirCellId>,
    ) -> ScirCellId {
        // Create the SCIR cell as a whitebox for now.
        // If this Substrate cell is actually a blackbox,
        // the contents of this SCIR cell will be made into a blackbox
        // by calling `cell.set_contents`.
        let mut cell = Cell::new_whitebox(self.name.clone());

        let mut nodes = HashMap::new();
        let mut roots_added = HashSet::new();

        for (&src, &root) in self.roots.iter() {
            let s = if !roots_added.contains(&root) {
                let s = cell.add_node(self.node_name(root));
                roots_added.insert(root);
                nodes.insert(root, s);
                s
            } else {
                nodes[&root]
            };
            nodes.insert(src, s);
        }

        for port in self.ports.iter() {
            cell.expose_port(nodes[&port.node()]);
        }

        let contents = match self.contents.as_ref() {
            Opacity::Opaque(s) => Opacity::Opaque(s.clone()),
            Opacity::Clear(contents) => {
                let mut inner = CellInner::new();
                for (i, instance) in contents.instances.iter().enumerate() {
                    if !cells.contains_key(&instance.child.id) {
                        instance.child.to_scir_cell(lib, cells);
                    }
                    let child: ScirCellId = *cells.get(&instance.child.id).unwrap();

                    let mut sinst = Instance::new(arcstr::format!("xinst{i}"), child);
                    assert_eq!(instance.child.ports.len(), instance.connections.len());
                    for (port, &conn) in instance.child.ports.iter().zip(&instance.connections) {
                        let scir_port_name = instance.child.node_name(port.node());
                        sinst.connect(scir_port_name, nodes[&conn]);
                    }
                    inner.add_instance(sinst);
                }

                for p in contents.primitives.iter() {
                    let sp = match p {
                        super::PrimitiveDevice::Res2 { pos, neg, value } => {
                            scir::PrimitiveDevice::Res2 {
                                pos: nodes[pos],
                                neg: nodes[neg],
                                value: scir::Expr::NumericLiteral(*value),
                            }
                        }
                        super::PrimitiveDevice::RawInstance {
                            ports,
                            cell,
                            params,
                        } => scir::PrimitiveDevice::RawInstance {
                            ports: ports.iter().map(|p| nodes[p]).collect(),
                            cell: cell.clone(),
                            params: params.clone(),
                        },
                    };
                    inner.add_primitive(sp);
                }
                Opacity::Clear(inner)
            }
        };

        cell.set_contents(contents);

        let id = lib.add_cell(cell);
        cells.insert(self.id, id);

        id
    }

    /// The name associated with the given node.
    ///
    /// # Panics
    ///
    /// Panics if the node does not exist within this cell.
    fn node_name(&self, node: Node) -> String {
        let node = self.roots[&node];
        self.node_names[&node].to_string()
    }
}

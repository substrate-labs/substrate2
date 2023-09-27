//! Schematic primitives.

use indexmap::IndexMap;
use rust_decimal::Decimal;
use scir::Expr;
use serde::{Deserialize, Serialize};
use spice::Spice;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::arcstr;
use crate::arcstr::ArcStr;
use crate::block::{Block, SchemaPrimitive};
use crate::io::{Array, InOut, Signal, TwoTerminalIo};
use crate::schematic::schema::{Schema, SchemaPrimitiveWrapper};

/// An instance with a pre-defined cell.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawInstance {
    /// The name of the underlying cell.
    pub cell: ArcStr,
    /// The name of the ports of the underlying cell.
    pub ports: Vec<ArcStr>,
    /// The parameters to pass to the instance.
    pub params: IndexMap<ArcStr, Expr>,
}

impl Hash for RawInstance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell.hash(state);
        self.ports.hash(state);
        self.params.iter().collect::<Vec<_>>().hash(state);
    }
}
impl RawInstance {
    /// Create a new raw instance with the given parameters.
    #[inline]
    pub fn from_params(
        cell: ArcStr,
        ports: Vec<ArcStr>,
        params: impl Into<IndexMap<ArcStr, Expr>>,
    ) -> Self {
        Self {
            cell,
            ports,
            params: params.into(),
        }
    }
    /// Create a new raw instance with no parameters.
    #[inline]
    pub fn new(cell: ArcStr, ports: Vec<ArcStr>) -> Self {
        Self {
            cell,
            ports,
            params: IndexMap::new(),
        }
    }
}
impl Block for RawInstance {
    type Kind = SchemaPrimitive;
    type Io = InOut<Array<Signal>>;

    fn id() -> ArcStr {
        arcstr::literal!("raw_instance")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("raw_instance_{}", self.cell)
    }

    fn io(&self) -> Self::Io {
        InOut(Array::new(self.ports.len(), Default::default()))
    }
}

/// An ideal 2-terminal resistor.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resistor {
    /// The resistor value.
    value: Decimal,
}
impl Resistor {
    /// Create a new resistor with the given value.
    #[inline]
    pub fn new(value: impl Into<Decimal>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// The value of the resistor.
    #[inline]
    pub fn value(&self) -> Decimal {
        self.value
    }
}
impl Block for Resistor {
    type Kind = SchemaPrimitive;
    type Io = TwoTerminalIo;

    fn id() -> ArcStr {
        arcstr::literal!("resistor")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("resistor_{}", self.value)
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

impl SchemaPrimitiveWrapper<Spice> for Resistor {
    fn primitive(&self) -> <Spice as Schema>::Primitive {
        spice::Primitive::Res2 {
            value: self.value.into(),
        }
    }
}

/// An ideal 2-terminal capacitor.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Capacitor {
    /// The resistor value.
    value: Decimal,
}
impl Capacitor {
    /// Create a new capacitor with the given value.
    #[inline]
    pub fn new(value: impl Into<Decimal>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// The value of the capacitor.
    #[inline]
    pub fn value(&self) -> Decimal {
        self.value
    }
}
impl Block for Capacitor {
    type Kind = SchemaPrimitive;
    type Io = TwoTerminalIo;

    fn id() -> ArcStr {
        arcstr::literal!("capacitor")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("capacitor_{}", self.value)
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

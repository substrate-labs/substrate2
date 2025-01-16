//! MOS devices and parameters.

use std::fmt::Display;

use crate::layers::Sky130Layer;
use crate::Sky130Pdk;
use arcstr::ArcStr;
use layir::Shape;
use serde::{Deserialize, Serialize};
use substrate::block::Block;
use substrate::geometry::bbox::Bbox;
use substrate::geometry::dir::Dir;
use substrate::geometry::rect::Rect;
use substrate::geometry::span::Span;
use substrate::layout::tracks::{RoundingMode, Tracks, UniformTracks};
use substrate::layout::Layout;
use substrate::schematic::CellBuilder;
use substrate::types::layout::{PortGeometry, PortGeometryBuilder};
use substrate::types::{
    Array, ArrayBundle, FlatLen, HasBundleKind, InOut, Input, Io, MosIo, MosIoView, Signal,
};

/// MOSFET sizing parameters.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MosParams {
    /// Device width, in nm.
    pub w: i64,
    /// Device channel length, in nm.
    pub l: i64,
    /// Number of fingers.
    pub nf: i64,
}

impl From<(i64, i64, i64)> for MosParams {
    fn from(value: (i64, i64, i64)) -> Self {
        Self {
            w: value.0,
            l: value.1,
            nf: value.2,
        }
    }
}

impl From<(i64, i64)> for MosParams {
    fn from(value: (i64, i64)) -> Self {
        Self {
            w: value.0,
            l: value.1,
            nf: 1,
        }
    }
}

impl Display for MosParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}x{}", self.w, self.l, self.nf)
    }
}

macro_rules! define_mosfets {
    ($({$typ:ident, $name:ident, $doc:literal, $opensubckt:ident, $comsubckt:ident}),*) => {
        /// An enumeration of Sky 130 MOSFET varieties.
        #[derive(Clone, Copy, Debug)]
        pub enum MosKind {
            $(
                #[doc = $doc]
                #[doc = ""]
                #[doc = concat!("In the open-source PDK, produces an instance of `", stringify!($opensubckt), "`.")]
                #[doc = concat!("In the commercial PDK, produces an instance of `", stringify!($comsubckt), "`.")]
                $typ,
            )*
        }

        impl MosKind {
            pub(crate) fn open_subckt(&self) -> arcstr::ArcStr {
                match self {
                    $(
                        MosKind::$typ => arcstr::literal!(stringify!($opensubckt))
                    ),*
                }
            }
            pub(crate) fn commercial_subckt(&self) -> arcstr::ArcStr {
                match self {
                    $(
                        MosKind::$typ => arcstr::literal!(stringify!($comsubckt))
                    ),*
                }
            }

            pub(crate) fn try_from_str(kind: &str) -> Option<Self> {
                match kind {
                    $(
                        stringify!($opensubckt) | stringify!($comsubckt) => Some(MosKind::$typ),
                    )*
                    _ => None,
                }
            }
        }
        $(
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
        #[doc = $doc]
        #[doc = ""]
        #[doc = concat!("In the open-source PDK, produces an instance of `", stringify!($opensubckt), "`.")]
        #[doc = concat!("In the commercial PDK, produces an instance of `", stringify!($comsubckt), "`.")]
        pub struct $typ {
            params: MosParams,
        }

        impl $typ {
            /// Creates a new [`$typ`].
            #[inline]
            pub fn new(params: impl Into<MosParams>) -> Self {
                Self {
                    params: params.into(),
                }
            }
        }

        impl Block for $typ {
            type Io = MosIo;

            fn name(&self) -> substrate::arcstr::ArcStr {
                arcstr::format!(concat!(stringify!($name), "_{}"), self.params)
            }
            fn io(&self) -> Self::Io {
                Default::default()
            }
        }

        impl substrate::schematic::Schematic for $typ {
            type Schema = crate::Sky130Pdk;
            type NestedData = ();
            fn schematic(
                    &self,
                    io: &substrate::types::schematic::IoNodeBundle<Self>,
                    cell: &mut CellBuilder<<Self as substrate::schematic::Schematic>::Schema>,
                ) -> substrate::error::Result<Self::NestedData> {
                let mut prim = substrate::schematic::PrimitiveBinding::new(crate::Primitive::Mos {
                    kind: MosKind::$typ,
                    params: self.params.clone(),
                });
                prim.connect("D", io.d);
                prim.connect("G", io.g);
                prim.connect("S", io.s);
                prim.connect("B", io.b);
                cell.set_primitive(prim);
                Ok(())
            }
        }
        )*
    };
}

define_mosfets!(
    {
        Nfet01v8,
        nfet_01v8,
        "A core NMOS device.",
        sky130_fd_pr__nfet_01v8,
        nshort
    },
    {
        Nfet01v8Lvt,
        nfet_01v8_lvt,
        "A core low-threshold NMOS device.",
        sky130_fd_pr__nfet_01v8_lvt,
        nlowvt
    },
    {
        Nfet03v3Nvt,
        nfet_03v3_nvt,
        "A 3.3V native-threshold NMOS device.",
        sky130_fd_pr__nfet_03v3_nvt,
        ntvnative
    },
    {
        Nfet05v0Nvt,
        nfet_05v0_nvt,
        "A 5.0V native-threshold NMOS device.",
        sky130_fd_pr__nfet_05v0_nvt,
        nhvnative
    },
    {
        Nfet20v0,
        nfet_20v0,
        "A 20.0V NMOS device.",
        sky130_fd_pr__nfet_20v0,
        nvhv
    },
    {
        SpecialNfetLatch,
        special_nfet_latch,
        "A special latch NMOS, used as the pull down device in SRAM cells.",
        sky130_fd_pr__special_nfet_latch,
        npd
    },
    {
        SpecialNfetPass,
        special_nfet_pass,
        "A special pass NMOS, used as the access device in SRAM cells.",
        sky130_fd_pr__special_nfet_pass,
        npass
    },
    {
        SpecialPfetPass,
        special_pfet_pass,
        "A special pass PMOS, used as the pull-up device in SRAM cells.",
        sky130_fd_pr__special_pfet_pass,
        ppu
    },
    {
        Pfet01v8,
        pfet_01v8,
        "A core PMOS device.",
        sky130_fd_pr__pfet_01v8,
        pshort
    },
    {
        Pfet01v8Hvt,
        pfet_01v8_hvt,
        "A core high-threshold PMOS device.",
        sky130_fd_pr__pfet_01v8_hvt,
        phighvt
    },
    {
        Pfet01v8Lvt,
        pfet_01v8_lvt,
        "A core low-threshold PMOS device.",
        sky130_fd_pr__pfet_01v8_lvt,
        plowvt
    },
    {
        Pfet20v0,
        pfet_20v0,
        "A 20.0V PMOS device.",
        sky130_fd_pr__pfet_20v0,
        pvhv
    }
);

/// Determines the connection direction of a transistor gate.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum GateDir {
    /// Connects the gate towards the right.
    #[default]
    Right,
    /// Connects the gate towards the left.
    Left,
}

/// The IO of an NMOS or PMOS tile.
#[derive(Debug, Clone, Io)]
pub struct BareMosTileIo {
    /// `NF + 1` source/drain contacts on li1, where `NF` is the number of fingers.
    pub sd: InOut<Array<Signal>>,
    /// `NF` gate contacts on li1, where `NF` is the number of fingers.
    pub g: Input<Array<Signal>>,
}

/// The set of supported gate lengths.
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default, Serialize, Deserialize,
)]
pub enum MosLength {
    /// 150nm.
    ///
    /// This is the minimum length supported by the SKY130 technology.
    #[default]
    L150,
}

impl MosLength {
    /// The length in nanometers.
    fn nm(&self) -> i64 {
        match *self {
            Self::L150 => 150,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
struct MosTile {
    w: i64,
    len: MosLength,
    nf: i64,
    gate_dir: GateDir,
}

impl Block for MosTile {
    type Io = BareMosTileIo;

    fn name(&self) -> ArcStr {
        arcstr::format!("mos_tile_w{}_l{}_nf{}", self.w, self.len.nm(), self.nf)
    }

    fn io(&self) -> Self::Io {
        BareMosTileIo {
            sd: InOut(Array::new(self.nf as usize + 1, Signal::new())),
            g: Input(Array::new(
                match self.gate_dir {
                    GateDir::Left => self.nf / 2 + 1,
                    GateDir::Right => (self.nf - 1) / 2 + 1,
                } as usize,
                Signal::new(),
            )),
        }
    }
}

impl Layout for MosTile {
    type Schema = Sky130Pdk;
    type Bundle = BareMosTileIoView<substrate::types::codegen::PortGeometryBundle<Sky130Pdk>>;
    type Data = ();
    fn layout(
        &self,
        cell: &mut substrate::layout::CellBuilder<Self::Schema>,
    ) -> substrate::error::Result<(Self::Bundle, Self::Data)> {
        let m0tracks = UniformTracks::new(170, 260);
        let m1tracks = UniformTracks::new(400, 140);

        let top_m1 = m1tracks.to_track_idx(self.w + 10, RoundingMode::Up);
        let bot_m1 = m1tracks.to_track_idx(-10, RoundingMode::Down);
        let gate_top_m1 = bot_m1 - 1;
        let gate_vspan = m1tracks
            .track(gate_top_m1)
            .union(m1tracks.track(gate_top_m1 - 1))
            .shrink_all(45);

        let tracks = (1..self.nf + 2)
            .map(|i| {
                let span = m0tracks.track(i);
                Rect::from_spans(span, Span::new(-10, self.w + 10))
            })
            .collect::<Vec<_>>();

        let gate_spans = tracks
            .windows(2)
            .map(|tracks| {
                let (left, right) = (tracks[0], tracks[1]);
                Span::new(left.right(), right.left()).shrink_all(55)
            })
            .collect::<Vec<_>>();

        let mut sd = Vec::new();
        for rect in tracks.iter() {
            let sd_rect = rect.with_vspan(
                m1tracks
                    .track(bot_m1)
                    .union(m1tracks.track(top_m1))
                    .shrink_all(45),
            );
            sd.push(PortGeometry::new(Shape::new(Sky130Layer::Li1, sd_rect)));
            cell.draw(Shape::new(Sky130Layer::Li1, sd_rect))?;
            let num_cuts = (self.w + 20 - 160 + 170) / 340;
            for j in 0..num_cuts {
                let base = rect.bot() + 10 + 80 + 340 * j;
                let cut = Rect::from_spans(rect.hspan(), Span::with_start_and_length(base, 170));
                cell.draw(Shape::new(Sky130Layer::Licon1, cut))?;
            }
        }

        let diff = Rect::from_sides(
            tracks[0].left() - 130,
            0,
            tracks.last().unwrap().right() + 130,
            self.w,
        );
        cell.draw(Shape::new(Sky130Layer::Diff, diff))?;

        let mut g = vec![None; self.io().g.len()];
        for i in 0..self.nf as usize {
            let li_track = tracks[match (i % 2 == 0, self.gate_dir) {
                (true, GateDir::Left) | (false, GateDir::Right) => i,
                _ => i + 1,
            }];

            let gate_idx = |idx| match self.gate_dir {
                GateDir::Left => (idx + 1) / 2,
                GateDir::Right => idx / 2,
            };
            let poly_li = Rect::from_spans(li_track.hspan(), gate_vspan);
            if i == 0 || gate_idx(i) != gate_idx(i - 1) {
                cell.draw(Shape::new(Sky130Layer::Li1, poly_li))?;
                g[gate_idx(i)] = Some(PortGeometry::new(Shape::new(Sky130Layer::Li1, poly_li)));

                let cut = Rect::from_spans(
                    li_track.hspan(),
                    Span::new(poly_li.top() - 90, poly_li.top() - 260),
                );
                cell.draw(Shape::new(Sky130Layer::Licon1, cut))?;

                let npc = Rect::from_spans(
                    poly_li.hspan(),
                    Span::new(poly_li.top(), poly_li.top() - 350),
                )
                .expand_dir(Dir::Vert, 10)
                .expand_dir(Dir::Horiz, 100);
                cell.draw(Shape::new(Sky130Layer::Npc, npc))?;
            }
            let poly = Rect::from_spans(
                gate_spans[i].union(li_track.hspan()),
                Span::new(poly_li.top() - 350, poly_li.top()),
            );
            cell.draw(Shape::new(Sky130Layer::Poly, poly))?;
        }
        let g = g.into_iter().map(|x| x.unwrap()).collect();

        for &span in gate_spans.iter() {
            cell.draw(Shape::new(
                Sky130Layer::Poly,
                Rect::from_spans(span, Span::new(gate_vspan.stop() - 350, self.w + 130)),
            ))?;
        }

        let bbox = cell.bbox_rect();
        let lcm_bbox = bbox;
        cell.draw(Shape::new(Sky130Layer::Outline, lcm_bbox))?;

        Ok((
            BareMosTileIoView {
                g: ArrayBundle::new(Signal, g),
                sd: ArrayBundle::new(Signal, sd),
            },
            (),
        ))
    }
}

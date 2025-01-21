//! The set of PDK layers.
#![allow(missing_docs)]

use gdsconv::GdsLayer;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Sky130Layer {
    PrBoundary,
    Pwell,
    Nwell,
    Dnwell,
    Vhvi,
    Diff,
    Tap,
    Psdm,
    Nsdm,
    Poly,
    Ldntm,
    Lvtn,
    Hvtp,
    Hvtr,
    Tunm,
    Licon1,
    /// Nitride poly cut.
    Npc,
    Li1,
    Mcon,
    Met1,
    Via,
    Met2,
    Via2,
    Met3,
    Via3,
    Met4,
    Via4,
    Met5,
    Pad,
    Rpm,
    Urpm,
    Hvi,
    Ncm,
    CfomDrawing,
    CfomMask,
    CfomMaskAdd,
    CfomMaskDrop,
    Cli1mDrawing,
    Cli1mMask,
    Cli1mMaskAdd,
    Cli1mMaskDrop,
    AreaIdLowTapDensity,
    AreaIdSeal,
    AreaIdCore,
    AreaIdFrame,
    AreaIdEsd,
    AreaIdStandardc,
    AreaIdAnalog,
    Outline,
}

impl Sky130Layer {
    pub fn gds_layer(&self) -> GdsLayer {
        match self {
            Self::PrBoundary => GdsLayer(235, 4),
            Self::Pwell => GdsLayer(64, 44),
            Self::Nwell => GdsLayer(64, 20),
            Self::Dnwell => GdsLayer(64, 18),
            Self::Vhvi => GdsLayer(74, 21),
            Self::Diff => GdsLayer(65, 20),
            Self::Tap => GdsLayer(65, 44),
            Self::Psdm => GdsLayer(94, 20),
            Self::Nsdm => GdsLayer(93, 44),
            Self::Poly => GdsLayer(66, 20),
            Self::Ldntm => GdsLayer(11, 44),
            Self::Lvtn => GdsLayer(125, 44),
            Self::Hvtp => GdsLayer(78, 44),
            Self::Hvtr => GdsLayer(18, 20),
            Self::Tunm => GdsLayer(80, 20),
            Self::Licon1 => GdsLayer(66, 44),
            Self::Npc => GdsLayer(95, 20),
            Self::Li1 => GdsLayer(67, 20),
            Self::Mcon => GdsLayer(67, 44),
            Self::Met1 => GdsLayer(68, 20),
            Self::Via => GdsLayer(68, 44),
            Self::Met2 => GdsLayer(69, 20),
            Self::Via2 => GdsLayer(69, 44),
            Self::Met3 => GdsLayer(70, 20),
            Self::Via3 => GdsLayer(70, 44),
            Self::Met4 => GdsLayer(71, 20),
            Self::Via4 => GdsLayer(71, 44),
            Self::Met5 => GdsLayer(72, 20),
            Self::Pad => GdsLayer(76, 20),
            Self::Rpm => GdsLayer(86, 20),
            Self::Urpm => GdsLayer(79, 20),
            Self::Hvi => GdsLayer(75, 20),
            Self::Ncm => GdsLayer(92, 44),
            Self::CfomDrawing => GdsLayer(22, 20),
            Self::CfomMask => GdsLayer(23, 0),
            Self::CfomMaskAdd => GdsLayer(22, 21),
            Self::CfomMaskDrop => GdsLayer(22, 22),
            Self::Cli1mDrawing => GdsLayer(115, 44),
            Self::Cli1mMask => GdsLayer(56, 0),
            Self::Cli1mMaskAdd => GdsLayer(115, 43),
            Self::Cli1mMaskDrop => GdsLayer(115, 42),
            Self::AreaIdLowTapDensity => GdsLayer(81, 14),
            Self::AreaIdSeal => GdsLayer(81, 1),
            Self::AreaIdCore => GdsLayer(81, 2),
            Self::AreaIdFrame => GdsLayer(81, 3),
            Self::AreaIdEsd => GdsLayer(81, 19),
            Self::AreaIdStandardc => GdsLayer(81, 4),
            Self::AreaIdAnalog => GdsLayer(81, 79),
            Self::Outline => GdsLayer(236, 0),
        }
    }

    pub fn gds_pin_layer(&self) -> Option<GdsLayer> {
        let layer = match self {
            Self::Pwell => GdsLayer(122, 16),
            Self::Nwell => GdsLayer(64, 16),
            Self::Poly => GdsLayer(66, 16),
            Self::Licon1 => GdsLayer(66, 58),
            Self::Li1 => GdsLayer(67, 16),
            Self::Mcon => GdsLayer(67, 48),
            Self::Met1 => GdsLayer(68, 16),
            Self::Via => GdsLayer(68, 58),
            Self::Met2 => GdsLayer(69, 16),
            Self::Via2 => GdsLayer(69, 58),
            Self::Met3 => GdsLayer(70, 16),
            Self::Via3 => GdsLayer(70, 48),
            Self::Met4 => GdsLayer(71, 16),
            Self::Via4 => GdsLayer(71, 48),
            Self::Met5 => GdsLayer(72, 16),
            Self::Pad => GdsLayer(76, 16),
            _ => return None,
        };
        Some(layer)
    }

    pub fn gds_label_layer(&self) -> Option<GdsLayer> {
        let layer = match self {
            Self::Pwell => GdsLayer(64, 59),
            Self::Nwell => GdsLayer(64, 5),
            Self::Poly => GdsLayer(66, 5),
            Self::Licon1 => GdsLayer(66, 41),
            Self::Li1 => GdsLayer(67, 5),
            Self::Mcon => GdsLayer(67, 41),
            Self::Met1 => GdsLayer(68, 5),
            Self::Via => GdsLayer(68, 41),
            Self::Met2 => GdsLayer(69, 5),
            Self::Via2 => GdsLayer(69, 41),
            Self::Met3 => GdsLayer(70, 5),
            Self::Via3 => GdsLayer(70, 41),
            Self::Met4 => GdsLayer(71, 5),
            Self::Via4 => GdsLayer(71, 41),
            Self::Met5 => GdsLayer(72, 5),
            Self::Pad => GdsLayer(76, 5),
            _ => return None,
        };
        Some(layer)
    }
}

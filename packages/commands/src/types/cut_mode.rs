use facet::Facet;
use strum::{Display, EnumIter, EnumString, FromRepr};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, Display, FromRepr, Facet)]
pub enum CuttingShape {
    Full,
    Partial,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
pub enum CutMode {
    Cut(CuttingShape),
    FeedAndCut(u8, CuttingShape),
    SetCuttingPosition(u8, CuttingShape),
    FeedAndCutAndMoveToStart(u8, CuttingShape),
}

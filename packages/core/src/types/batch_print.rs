use facet::Facet;

#[derive(Clone, Copy, Facet)]
#[repr(C)]
pub enum BatchPrintMode {
    Disable,
    Enable,
}

#[derive(Clone, Copy, Facet)]
#[repr(C)]
pub enum BatchPrintDirection {
    Forward,
    Reverse,
}

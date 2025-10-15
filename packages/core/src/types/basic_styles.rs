use std::alloc::Layout;

use bitfield_struct::bitfield;
use facet::{ConstTypeId, Def, Facet, MarkerTraits, Shape, Type, ValueVTable, ValueVTableSized};
use winnow::{
    Parser, Partial,
    binary::u8,
    error::{ContextError, ErrMode},
};

use crate::commands::reader::error::ErrorCtx;

#[bitfield(u8)]
pub struct BasicStyles {
    #[bits(1)]
    pub font_index: u8,

    #[bits(2)]
    _reserved0: u8,

    pub emphasized: bool,
    pub double_height: bool,
    pub double_width: bool,

    #[bits(1)]
    _reserved1: u8,

    pub underline: bool,
}

unsafe impl<'f> Facet<'f> for BasicStyles {
    const VTABLE: &'static facet::ValueVTable = &ValueVTable::Sized(ValueVTableSized {
        type_name: |f, opts| f.write_str("BasicStyles"),
        clone_into: || None,
        debug: || None,
        default_in_place: || None,
        display: || None,
        drop_in_place: || None,
        hash: || None,
        invariants: || None,
        ord: || None,
        parse: || None,
        partial_eq: || None,
        partial_ord: || None,
        try_borrow_inner: || None,
        try_from: || None,
        try_into_inner: || None,
        marker_traits: || MarkerTraits::empty(),
    });

    const SHAPE: &'static facet::Shape = &Shape {
        id: ConstTypeId::of::<BasicStyles>(),
        attributes: &[],
        def: Def::Scalar,
        doc: &[],
        inner: None,
        layout: facet::ShapeLayout::Sized(unsafe { Layout::from_size_align_unchecked(1, 1) }),
        ty: Type::Primitive(facet::PrimitiveType::Numeric(facet::NumericType::Integer {
            signed: false,
        })),
        type_identifier: "BasicStyles",
        type_params: &[],
        type_tag: None,
        vtable: Self::VTABLE,
    };
}

impl BasicStyles {
    pub fn parser<'i>() -> impl Parser<Partial<&'i [u8]>, Self, ErrMode<ContextError<ErrorCtx>>> {
        u8.map(|v| BasicStyles::from_bits(v))
    }
}

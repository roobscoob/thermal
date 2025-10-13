pub mod reader;

use escpos_macros::escpos_commands;

escpos_commands! {
    SPEC_PATH = "../../vendor/spec/escpos-commands.json",
    enum_name = Command,

    derive = "std::clone::Clone, facet::Facet, strum::Display, strum::EnumCount, strum::EnumDiscriminants, strum::EnumMessage",
    strum_discriminants_derive = "strum::EnumCount, strum::EnumIter, strum::EnumMessage",

    category_enum_name = CommandCategory,
    category_derive = "strum::EnumCount, strum::EnumIter, strum::EnumMessage",
}

/// Usage:
/// simple_encoding!(PC437, PC437Character with PC437_TABLE, PC437_INV_TABLE);
///
/// Requirements:
/// - `PC437_TABLE: [char; N]`  (index -> char)
/// - `PC437_INV_TABLE: &[(char, u8); N]`  (sorted by char, char -> index)
/// - Traits `Encoding` and `Character` are in the same crate as this macro.
#[macro_export]
macro_rules! simple_encoding {
    ($enc:ident, $char_ty:ident with offset $offset:tt and table $table:ident, $inv:ident) => {
        /// Zero-sized encoding marker type.
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $enc;

        /// Newtype that only represents characters encodable by this table.
        /// Field is private to enforce construction via the `Character` trait.
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $char_ty(char);

        impl $char_ty {
            #[inline]
            fn as_char(self) -> char {
                self.0
            }
        }

        impl $crate::encoding::Character for $char_ty {
            #[inline]
            fn into_char(&self) -> Option<char> {
                // Always Some because instances are only constructed when valid.
                Some(self.0)
            }

            #[inline]
            fn from_char(c: char) -> Option<Self> {
                if (c as usize) < 127 {
                    return None;
                }

                // Membership check via binary search over the inverse table.
                match $inv.binary_search_by_key(&c, |&(ch, _)| ch) {
                    Ok(_) => Some(Self(c)),
                    Err(_) => None,
                }
            }
        }

        impl $enc {
            #[inline]
            fn index_of(c: char) -> u8 {
                // Safe because $char_ty::from_char only yields valid entries.
                let i = $inv
                    .binary_search_by_key(&c, |&(ch, _)| ch)
                    .expect("character must exist in inverse table");
                $inv[i].1
            }
        }

        impl $crate::encoding::Encoding for $enc {
            type Character = $char_ty;

            #[inline]
            fn with_encoding<R, F: for<'a> FnOnce(&'a [u8]) -> R>(
                &self,
                c: Self::Character,
                e: F,
            ) -> Option<R> {
                let idx = Self::index_of(c.as_char());
                let one = [idx + $offset];
                Some(e(&one))
            }

            #[inline]
            fn decode(&self, v: &mut &[u8]) -> Option<Self::Character> {
                if v.is_empty() {
                    return None;
                }

                let (first, rest) = v.split_first()?;
                let i = (*first as usize) + $offset;

                if i < $table.len() {
                    // Any index within table length maps to a valid char in the forward table,
                    // so wrapping in $char_ty is always valid.
                    *v = rest;
                    Some($char_ty($table[i]))
                } else {
                    None
                }
            }
        }
    };
}

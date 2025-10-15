use crate::encoding::PartialUnicodeEncoding;

pub mod encoding;
pub(crate) mod r#macro;
pub mod tables;

pub struct EncodeStr<
    'l,
    V,
    T: ?Sized + PartialUnicodeEncoding<V> + 'l,
    I: Iterator<Item = &'l T> + Clone,
    M: for<'a> FnMut(&'a [u8], &'l T) -> V,
> {
    string: &'l str,
    encodings: I,
    mapping: M,

    prev_encoding: Option<&'l T>,
}

impl<
    'l,
    V,
    T: ?Sized + PartialUnicodeEncoding<V> + 'l,
    I: Iterator<Item = &'l T> + Clone,
    M: for<'a> FnMut(&'a [u8], &'l T) -> V,
> Iterator for EncodeStr<'l, V, T, I, M>
{
    type Item = Result<V, char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.string.chars();
        let (char, str) = chars.next().map(|c| (c, chars.as_str()))?;
        self.string = str;

        for enc in self
            .prev_encoding
            .iter()
            .copied()
            .chain(self.encodings.clone())
        {
            if let Some(v) = enc.try_encode_char(char, &mut |bytes| (self.mapping)(bytes, enc)) {
                self.prev_encoding = Some(enc);

                return Some(Ok(v));
            }
        }

        Some(Err(char))
    }
}

pub fn encode_str<
    'b,
    V,
    T: ?Sized + PartialUnicodeEncoding<V> + 'b,
    I: Iterator<Item = &'b T> + Clone,
    M: for<'a> FnMut(&'a [u8], &'b T) -> V,
>(
    string: &'b str,
    encodings: I,
    mapping: M,
) -> EncodeStr<'b, V, T, I, M> {
    EncodeStr {
        encodings,
        mapping,
        string,
        prev_encoding: None,
    }
}

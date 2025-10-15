pub trait Character: Sized {
    fn into_char(&self) -> Option<char>;
    fn from_char(c: char) -> Option<Self>;
}

impl Character for char {
    fn from_char(c: char) -> Option<Self> {
        Some(c)
    }

    fn into_char(&self) -> Option<char> {
        Some(*self)
    }
}

pub trait Encoding {
    type Character: Character;

    fn with_encoding<R, F: for<'a> FnOnce(&'a [u8]) -> R>(
        &self,
        c: Self::Character,
        e: F,
    ) -> Option<R>;

    fn decode(&self, v: &mut &[u8]) -> Option<Self::Character>;
}

pub trait PartialUnicodeEncoding<T> {
    /// If `ch` is representable, invoke `with_bytes` exactly once and return its result.
    /// Otherwise return `None`. The &[u8] lives only for the duration of the call.
    fn try_encode_char(&self, ch: char, with_bytes: &mut dyn FnMut(&[u8]) -> T) -> Option<T>;

    fn try_decode_char(&self, v: &mut &[u8]) -> Option<char>;
}

impl<T> PartialUnicodeEncoding<T> for &dyn PartialUnicodeEncoding<T> {
    fn try_encode_char(&self, ch: char, with_bytes: &mut dyn FnMut(&[u8]) -> T) -> Option<T> {
        (**self).try_encode_char(ch, with_bytes)
    }

    fn try_decode_char(&self, v: &mut &[u8]) -> Option<char> {
        (**self).try_decode_char(v)
    }
}

impl<E, T> PartialUnicodeEncoding<T> for E
where
    E: Encoding,
{
    fn try_encode_char(&self, ch: char, with_bytes: &mut dyn FnMut(&[u8]) -> T) -> Option<T> {
        let c = <E::Character as Character>::from_char(ch)?;
        self.with_encoding(c, |bytes| with_bytes(bytes))
    }

    fn try_decode_char(&self, v: &mut &[u8]) -> Option<char> {
        self.decode(v).and_then(|c| c.into_char())
    }
}

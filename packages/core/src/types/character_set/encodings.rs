use thermal_encoding::{
    encoding::{Character, Encoding},
    tables::{
        Iso88597Character, KatakanaCharacter, Pc437Character, Pc737Character, Pc850Character,
        Pc851Character, Pc852Character, Pc853Character, Pc857Character, Pc858Character,
        Pc860Character, Pc863Character, Pc865Character, Pc866Character, Wpc1252Character,
    },
};

use crate::types::character_set::{AsciiVariant, Codepage};

impl AsciiVariant {
    #[inline]
    pub fn try_encode(&self, ch: char) -> Option<u8> {
        match (self, ch) {
            (Self::Uk, '£') => Some(0x23),
            (Self::Spain1, '₧') => Some(0x23),
            (Self::Vietnam, '₫') => Some(0x23),

            (Self::China, '¥') => Some(0x24),
            (Self::Sweden | Self::Norway, '¤') => Some(0x24),

            (Self::Arabia, '٪') => Some(0x25),
            (Self::Arabia, '٭') => Some(0x2A),

            (Self::France, 'à') => Some(0x40),
            (Self::Germany, '§') => Some(0x40),
            (Self::Sweden | Self::Norway | Self::Denmark2, 'É') => Some(0x40),
            (Self::Spain2 | Self::LatinAmerica, 'á') => Some(0x40),
            (Self::SloveniaCroatia, 'Ž') => Some(0x40),

            (Self::France | Self::Italy, '°') => Some(0x5B),
            (Self::Germany | Self::Sweden, 'Ä') => Some(0x5B),
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 'Æ') => Some(0x5B),
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, '¡') => Some(0x5B),
            (Self::SloveniaCroatia, 'Š') => Some(0x5B),

            (Self::France, 'ç') => Some(0x5C),
            (Self::Germany | Self::Sweden, 'Ö') => Some(0x5C),
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 'Ø') => Some(0x5C),
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, 'Ñ') => Some(0x5C),
            (Self::Japan, '¥') => Some(0x5C),
            (Self::Korea, '₩') => Some(0x5C),
            (Self::SloveniaCroatia, 'Đ') => Some(0x5C),

            (Self::France, '§') => Some(0x5D),
            (Self::Germany, 'Ü') => Some(0x5D),
            (Self::Denmark1 | Self::Sweden | Self::Norway | Self::Denmark2, 'Å') => Some(0x5D),
            (Self::Italy, 'é') => Some(0x5D),
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, '¿') => Some(0x5D),
            (Self::SloveniaCroatia, 'Ć') => Some(0x5D),

            (Self::Sweden | Self::Norway | Self::Denmark2, 'Ü') => Some(0x5E),
            (Self::Spain2 | Self::LatinAmerica, 'é') => Some(0x5E),
            (Self::SloveniaCroatia, 'Č') => Some(0x5E),

            (Self::Sweden | Self::Norway | Self::Denmark2, 'é') => Some(0x60),
            (Self::Italy, 'ù') => Some(0x60),
            (Self::LatinAmerica, 'ü') => Some(0x60),
            (Self::SloveniaCroatia, 'ž') => Some(0x60),

            (Self::France, 'é') => Some(0x7B),
            (Self::Germany | Self::Sweden, 'ä') => Some(0x7B),
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 'æ') => Some(0x7B),
            (Self::Italy, 'à') => Some(0x7B),
            (Self::Spain1, '¨') => Some(0x7B),
            (Self::Spain2 | Self::LatinAmerica, 'í') => Some(0x7B),
            (Self::SloveniaCroatia, 'š') => Some(0x7B),

            (Self::France, 'ù') => Some(0x7C),
            (Self::Germany | Self::Sweden, 'ö') => Some(0x7C),
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 'ø') => Some(0x7C),
            (Self::Italy, 'ò') => Some(0x7C),
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, 'ñ') => Some(0x7C),
            (Self::SloveniaCroatia, 'đ') => Some(0x7C),

            (Self::France | Self::Italy, 'è') => Some(0x7D),
            (Self::Germany, 'ü') => Some(0x7D),
            (Self::Denmark1 | Self::Sweden | Self::Norway | Self::Denmark2, 'å') => Some(0x7D),
            (Self::Spain2 | Self::LatinAmerica, 'ó') => Some(0x7D),
            (Self::SloveniaCroatia, 'ć') => Some(0x7D),

            (Self::France, '¨') => Some(0x7E),
            (Self::Germany, 'ß') => Some(0x7E),
            (Self::Sweden | Self::Norway | Self::Denmark2, 'ü') => Some(0x7E),
            (Self::Italy, 'ì') => Some(0x7E),
            (Self::Spain2 | Self::LatinAmerica, 'ú') => Some(0x7E),
            (Self::SloveniaCroatia, 'č') => Some(0x7E),

            _ => None,
        }
        .or_else(|| {
            if ch.is_ascii() {
                let b = ch as u8;
                if !Self::slot_overridden(*self, b) {
                    Some(b)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    #[inline]
    fn slot_overridden(table: Self, b: u8) -> bool {
        matches!(
            (table, b),
            // 0x23
            (Self::Uk | Self::Spain1 | Self::Vietnam, 0x23)
            // 0x24
            | (Self::China | Self::Sweden | Self::Norway, 0x24)
            // 0x25, 0x2A
            | (Self::Arabia, 0x25 | 0x2A)
            // 0x40
            | (Self::France | Self::Germany | Self::Sweden | Self::Norway | Self::Denmark2
                | Self::Spain2 | Self::LatinAmerica | Self::SloveniaCroatia, 0x40)
            // 0x5B
            | (Self::France | Self::Italy | Self::Germany | Self::Sweden
                | Self::Norway | Self::Denmark1 | Self::Denmark2
                | Self::Spain1 | Self::Spain2 | Self::LatinAmerica
                | Self::SloveniaCroatia, 0x5B)
            // 0x5C
            | (Self::France | Self::Germany | Self::Sweden | Self::Norway
                | Self::Denmark1 | Self::Denmark2 | Self::Spain1 | Self::Spain2
                | Self::LatinAmerica | Self::Japan | Self::Korea
                | Self::SloveniaCroatia, 0x5C)
            // 0x5D
            | (Self::France | Self::Germany | Self::Denmark1 | Self::Sweden
                | Self::Norway | Self::Denmark2 | Self::Italy
                | Self::Spain1 | Self::Spain2 | Self::LatinAmerica
                | Self::SloveniaCroatia, 0x5D)
            // 0x5E
            | (Self::Sweden | Self::Norway | Self::Denmark2
                | Self::Spain2 | Self::LatinAmerica | Self::SloveniaCroatia, 0x5E)
            // 0x60
            | (Self::Sweden | Self::Norway | Self::Denmark2
                | Self::Italy | Self::LatinAmerica | Self::SloveniaCroatia, 0x60)
            // 0x7B
            | (Self::France | Self::Germany | Self::Sweden | Self::Norway
                | Self::Denmark1 | Self::Denmark2 | Self::Italy
                | Self::Spain1 | Self::Spain2 | Self::LatinAmerica
                | Self::SloveniaCroatia, 0x7B)
            // 0x7C
            | (Self::France | Self::Germany | Self::Sweden | Self::Norway
                | Self::Denmark1 | Self::Denmark2 | Self::Italy
                | Self::Spain1 | Self::Spain2 | Self::LatinAmerica
                | Self::SloveniaCroatia, 0x7C)
            // 0x7D
            | (Self::France | Self::Italy | Self::Germany | Self::Denmark1
                | Self::Sweden | Self::Norway | Self::Denmark2
                | Self::Spain2 | Self::LatinAmerica | Self::SloveniaCroatia, 0x7D)
            // 0x7E
            | (Self::France | Self::Germany | Self::Sweden | Self::Norway
                | Self::Denmark2 | Self::Italy | Self::Spain2
                | Self::LatinAmerica | Self::SloveniaCroatia, 0x7E)
        )
    }
}

impl Encoding for AsciiVariant {
    type Character = char;

    fn decode(&self, v: &mut &[u8]) -> Option<Self::Character> {
        let (first, rest) = v.split_first()?;

        let x = Some(match (self, *first) {
            (_, 128..) => return None,

            (Self::Uk, 0x23) => '£',
            (Self::Spain1, 0x23) => '₧',
            (Self::Vietnam, 0x23) => '₫',
            (_, 0x23) => '#',

            (Self::China, 0x24) => '¥',
            (Self::Sweden | Self::Norway, 0x24) => '¤',
            (_, 0x24) => '$',

            (Self::Arabia, 0x25) => '٪',
            (_, 0x25) => '%',

            (Self::Arabia, 0x2A) => '٭',
            (_, 0x2A) => '*',

            (Self::France, 0x40) => 'à',
            (Self::Germany, 0x40) => '§',
            (Self::Sweden | Self::Norway | Self::Denmark2, 0x40) => 'É',
            (Self::Spain2 | Self::LatinAmerica, 0x40) => 'á',
            (Self::SloveniaCroatia, 0x40) => 'Ž',
            (_, 0x40) => '@',

            (Self::France | Self::Italy, 0x5B) => '°',
            (Self::Germany | Self::Sweden, 0x5B) => 'Ä',
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 0x5B) => 'Æ',
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, 0x5B) => '¡',
            (Self::SloveniaCroatia, 0x5B) => 'Š',
            (_, 0x5B) => '[',

            (Self::France, 0x5C) => 'ç',
            (Self::Germany | Self::Sweden, 0x5C) => 'Ö',
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 0x5C) => 'Ø',
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, 0x5C) => 'Ñ',
            (Self::Japan, 0x5C) => '¥',
            (Self::Korea, 0x5C) => '₩',
            (Self::SloveniaCroatia, 0x5C) => 'Đ',
            (_, 0x5C) => '\\',

            (Self::France, 0x5D) => '§',
            (Self::Germany, 0x5D) => 'Ü',
            (Self::Denmark1 | Self::Sweden | Self::Norway | Self::Denmark2, 0x5D) => 'Å',
            (Self::Italy, 0x5D) => 'é',
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, 0x5D) => '¿',
            (Self::SloveniaCroatia, 0x5D) => 'Ć',
            (_, 0x5D) => ']',

            (Self::Sweden | Self::Norway | Self::Denmark2, 0x5E) => 'Ü',
            (Self::Spain2 | Self::LatinAmerica, 0x5E) => 'é',
            (Self::SloveniaCroatia, 0x5E) => 'Č',
            (_, 0x5E) => '^',

            (Self::Sweden | Self::Norway | Self::Denmark2, 0x60) => 'é',
            (Self::Italy, 0x60) => 'ù',
            (Self::LatinAmerica, 0x60) => 'ü',
            (Self::SloveniaCroatia, 0x60) => 'ž',
            (_, 0x60) => '`',

            (Self::France, 0x7B) => 'é',
            (Self::Germany | Self::Sweden, 0x7B) => 'ä',
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 0x7B) => 'æ',
            (Self::Italy, 0x7B) => 'à',
            (Self::Spain1, 0x7B) => '¨',
            (Self::Spain2 | Self::LatinAmerica, 0x7B) => 'í',
            (Self::SloveniaCroatia, 0x7B) => 'š',
            (_, 0x7B) => '{',

            (Self::France, 0x7C) => 'ù',
            (Self::Germany | Self::Sweden, 0x7C) => 'ö',
            (Self::Norway | Self::Denmark1 | Self::Denmark2, 0x7C) => 'ø',
            (Self::Italy, 0x7C) => 'ò',
            (Self::Spain1 | Self::Spain2 | Self::LatinAmerica, 0x7C) => 'ñ',
            (Self::SloveniaCroatia, 0x7C) => 'đ',
            (_, 0x7C) => '|',

            (Self::France | Self::Italy, 0x7D) => 'è',
            (Self::Germany, 0x7D) => 'ü',
            (Self::Denmark1 | Self::Sweden | Self::Norway | Self::Denmark2, 0x7D) => 'å',
            (Self::Spain2 | Self::LatinAmerica, 0x7D) => 'ó',
            (Self::SloveniaCroatia, 0x7D) => 'ć',
            (_, 0x7D) => '}',

            (Self::France, 0x7E) => '¨',
            (Self::Germany, 0x7E) => 'ß',
            (Self::Sweden | Self::Norway | Self::Denmark2, 0x7E) => 'ü',
            (Self::Italy, 0x7E) => 'ì',
            (Self::Spain2 | Self::LatinAmerica, 0x7E) => 'ú',
            (Self::SloveniaCroatia, 0x7E) => 'č',
            (_, 0x7E) => '~',

            (_, v) => v as char,
        });

        *v = rest;

        x
    }

    fn with_encoding<R, F: for<'a> FnOnce(&'a [u8]) -> R>(
        &self,
        c: Self::Character,
        e: F,
    ) -> Option<R> {
        Some(e(&[self.try_encode(c)?]))
    }
}

pub enum CharacterCodeTableCharacter {
    Unicode(char),
}

impl CharacterCodeTableCharacter {
    pub fn try_from<T: Character>(v: T) -> Option<Self> {
        Self::try_from(v.into_char()?)
    }
}

impl Character for CharacterCodeTableCharacter {
    fn from_char(c: char) -> Option<Self> {
        Some(Self::Unicode(c))
    }

    fn into_char(&self) -> Option<char> {
        match self {
            Self::Unicode(u) => Some(*u),
        }
    }
}

impl Encoding for Codepage {
    type Character = CharacterCodeTableCharacter;

    fn decode(&self, v: &mut &[u8]) -> Option<Self::Character> {
        match self {
            Self::Page0_Pc437 => thermal_encoding::tables::PC437
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page1_Katakana => thermal_encoding::tables::KATAKANA
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page2_Pc850 => thermal_encoding::tables::PC850
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page3_Pc860 => thermal_encoding::tables::PC860
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page4_Pc863 => thermal_encoding::tables::PC863
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page5_Pc865 => thermal_encoding::tables::PC865
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page6_Hiragana => todo!(),
            Self::Page7_OnePassKanji => todo!(),
            Self::Page8_OnePassKanji => todo!(),

            Self::Page11_Pc851 => thermal_encoding::tables::PC851
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page12_Pc853 => thermal_encoding::tables::PC853
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page13_Pc857 => thermal_encoding::tables::PC857
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page14_Pc737 => thermal_encoding::tables::PC737
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page15_Iso8859_7 => thermal_encoding::tables::ISO8859_7
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page16_Wpc1252 => thermal_encoding::tables::WPC1252
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page17_Pc866 => thermal_encoding::tables::PC866
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page18_Pc852 => thermal_encoding::tables::PC852
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page19_Pc858 => thermal_encoding::tables::PC858
                .decode(v)
                .and_then(|v| CharacterCodeTableCharacter::try_from(v)),

            Self::Page20_Thai42 => todo!(),
            Self::Page21_Thai11 => todo!(),
            Self::Page22_Thai13 => todo!(),
            Self::Page23_Thai14 => todo!(),
            Self::Page24_Thai16 => todo!(),
            Self::Page25_Thai17 => todo!(),
            Self::Page26_Thai18 => todo!(),
            Self::Page30_Tcvn3 => todo!(),
            Self::Page31_Tcvn3 => todo!(),
            Self::Page32_Pc720 => todo!(),
            Self::Page33_Wpc775 => todo!(),
            Self::Page34_Pc855 => todo!(),
            Self::Page35_Pc861 => todo!(),
            Self::Page36_Pc862 => todo!(),
            Self::Page37_Pc864 => todo!(),
            Self::Page38_Pc869 => todo!(),
            Self::Page39_Iso8859_2 => todo!(),
            Self::Page40_Iso8859_15 => todo!(),
            Self::Page41_Pc1098 => todo!(),
            Self::Page42_Pc1118 => todo!(),
            Self::Page43_Pc1119 => todo!(),
            Self::Page44_Pc1125 => todo!(),
            Self::Page45_Wpc1250 => todo!(),
            Self::Page46_Wpc1251 => todo!(),
            Self::Page47_Wpc1253 => todo!(),
            Self::Page48_Wpc1254 => todo!(),
            Self::Page49_Wpc1255 => todo!(),
            Self::Page50_Wpc1256 => todo!(),
            Self::Page51_Wpc1257 => todo!(),
            Self::Page52_Wpc1258 => todo!(),
            Self::Page53_Kz1048 => todo!(),
            Self::Page66_Devanagari => todo!(),
            Self::Page67_Bengali => todo!(),
            Self::Page68_Tamil => todo!(),
            Self::Page69_Telugu => todo!(),
            Self::Page70_Assamese => todo!(),
            Self::Page71_Oriya => todo!(),
            Self::Page72_Kannada => todo!(),
            Self::Page73_Malayalam => todo!(),
            Self::Page74_Gujarati => todo!(),
            Self::Page75_Punjabi => todo!(),
            Self::Page82_Marathi => todo!(),
            Self::Page254 => todo!(),
            Self::Page255 => todo!(),
        }
    }

    fn with_encoding<R, F: for<'a> FnOnce(&'a [u8]) -> R>(
        &self,
        c: Self::Character,
        e: F,
    ) -> Option<R> {
        match self {
            Self::Page0_Pc437 => thermal_encoding::tables::PC437
                .with_encoding(Pc437Character::from_char(c.into_char()?)?, e),

            Self::Page1_Katakana => thermal_encoding::tables::KATAKANA
                .with_encoding(KatakanaCharacter::from_char(c.into_char()?)?, e),

            Self::Page2_Pc850 => thermal_encoding::tables::PC850
                .with_encoding(Pc850Character::from_char(c.into_char()?)?, e),

            Self::Page3_Pc860 => thermal_encoding::tables::PC860
                .with_encoding(Pc860Character::from_char(c.into_char()?)?, e),

            Self::Page4_Pc863 => thermal_encoding::tables::PC863
                .with_encoding(Pc863Character::from_char(c.into_char()?)?, e),

            Self::Page5_Pc865 => thermal_encoding::tables::PC865
                .with_encoding(Pc865Character::from_char(c.into_char()?)?, e),

            Self::Page6_Hiragana => todo!(),
            Self::Page7_OnePassKanji => todo!(),
            Self::Page8_OnePassKanji => todo!(),

            Self::Page11_Pc851 => thermal_encoding::tables::PC851
                .with_encoding(Pc851Character::from_char(c.into_char()?)?, e),

            Self::Page12_Pc853 => thermal_encoding::tables::PC853
                .with_encoding(Pc853Character::from_char(c.into_char()?)?, e),

            Self::Page13_Pc857 => thermal_encoding::tables::PC857
                .with_encoding(Pc857Character::from_char(c.into_char()?)?, e),

            Self::Page14_Pc737 => thermal_encoding::tables::PC737
                .with_encoding(Pc737Character::from_char(c.into_char()?)?, e),

            Self::Page15_Iso8859_7 => thermal_encoding::tables::ISO8859_7
                .with_encoding(Iso88597Character::from_char(c.into_char()?)?, e),

            Self::Page16_Wpc1252 => thermal_encoding::tables::WPC1252
                .with_encoding(Wpc1252Character::from_char(c.into_char()?)?, e),

            Self::Page17_Pc866 => thermal_encoding::tables::PC866
                .with_encoding(Pc866Character::from_char(c.into_char()?)?, e),

            Self::Page18_Pc852 => thermal_encoding::tables::PC852
                .with_encoding(Pc852Character::from_char(c.into_char()?)?, e),

            Self::Page19_Pc858 => thermal_encoding::tables::PC858
                .with_encoding(Pc858Character::from_char(c.into_char()?)?, e),

            Self::Page20_Thai42 => todo!(),
            Self::Page21_Thai11 => todo!(),
            Self::Page22_Thai13 => todo!(),
            Self::Page23_Thai14 => todo!(),
            Self::Page24_Thai16 => todo!(),
            Self::Page25_Thai17 => todo!(),
            Self::Page26_Thai18 => todo!(),
            Self::Page30_Tcvn3 => todo!(),
            Self::Page31_Tcvn3 => todo!(),
            Self::Page32_Pc720 => todo!(),
            Self::Page33_Wpc775 => todo!(),
            Self::Page34_Pc855 => todo!(),
            Self::Page35_Pc861 => todo!(),
            Self::Page36_Pc862 => todo!(),
            Self::Page37_Pc864 => todo!(),
            Self::Page38_Pc869 => todo!(),
            Self::Page39_Iso8859_2 => todo!(),
            Self::Page40_Iso8859_15 => todo!(),
            Self::Page41_Pc1098 => todo!(),
            Self::Page42_Pc1118 => todo!(),
            Self::Page43_Pc1119 => todo!(),
            Self::Page44_Pc1125 => todo!(),
            Self::Page45_Wpc1250 => todo!(),
            Self::Page46_Wpc1251 => todo!(),
            Self::Page47_Wpc1253 => todo!(),
            Self::Page48_Wpc1254 => todo!(),
            Self::Page49_Wpc1255 => todo!(),
            Self::Page50_Wpc1256 => todo!(),
            Self::Page51_Wpc1257 => todo!(),
            Self::Page52_Wpc1258 => todo!(),
            Self::Page53_Kz1048 => todo!(),
            Self::Page66_Devanagari => todo!(),
            Self::Page67_Bengali => todo!(),
            Self::Page68_Tamil => todo!(),
            Self::Page69_Telugu => todo!(),
            Self::Page70_Assamese => todo!(),
            Self::Page71_Oriya => todo!(),
            Self::Page72_Kannada => todo!(),
            Self::Page73_Malayalam => todo!(),
            Self::Page74_Gujarati => todo!(),
            Self::Page75_Punjabi => todo!(),
            Self::Page82_Marathi => todo!(),
            Self::Page254 => todo!(),
            Self::Page255 => todo!(),
        }
    }
}

macro_rules! assert_xml_parse {
    ($xml:expr, $expected:expr) => {
        {
            use ::std::{str::FromStr, fmt::Debug};

            use crate::util::Element;

            // Workaround to help type inference "see" the generic type.

            fn assert_element_parses<'a, T>(element: &'a Element, expected: T)
            where
                T: Debug + Eq + TryFrom<&'a Element>,
                T::Error: Debug
            {
                assert_eq!(
                    T::try_from(&element).unwrap(),
                    expected
                )
            }

            let element = Element::from_str($xml).unwrap();
            assert_element_parses(&element, $expected)
        }
    };
}

macro_rules! assert_xml_format {
    ($actual:expr, $xml:expr) => {
        {
            use ::std::str::FromStr;

            use crate::util::Element;

            assert_eq!(
                Element::try_from($actual).unwrap(),
                Element::from_str($xml).unwrap()
            )
        }
    };
}

pub(crate) use assert_xml_parse;
pub(crate) use assert_xml_format;

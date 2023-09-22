macro_rules! assert_xml_parse {
    ($xml:expr, $expected:expr) => {
        {
            use ::std::{str::FromStr, fmt::Debug};

            use crate::util::{Element, Error};

            /// Workaround to help type inference "see" the generic type.
            fn _impl<T>(xml: &str, expected: T)
            where
                for<'a> T: Debug + Eq + TryFrom<&'a Element, Error = Error>,
            {
                let element = Element::from_str(xml).unwrap();
                ::pretty_assertions::assert_eq!(
                    T::try_from(&element).unwrap(),
                    expected
                )
            }

            _impl($xml, $expected)
        }
    };
}

macro_rules! assert_xml_format {
    ($actual:expr, $xml:expr) => {
        {
            use ::std::str::FromStr;

            use crate::util::Element;

            ::pretty_assertions::assert_eq!(
                Element::try_from($actual).unwrap(),
                Element::from_str($xml).unwrap()
            )
        }
    };
}

macro_rules! assert_xml_roundtrip {
    ($value:expr) => {
        {
            use ::std::fmt::Debug;

            use crate::util::{Element, Error};

            /// Workaround to help type inference "see" the generic type.
            fn _impl<T>(value: T)
            where
                for<'a> T: Clone + Debug + Eq + Into<Element> + TryFrom<&'a Element, Error = Error>,
            {
                let element = value.clone().into();
                ::pretty_assertions::assert_eq!(
                    T::try_from(&element).unwrap(),
                    value
                )
            }

            _impl($value)
        }
    };
}

pub(crate) use assert_xml_parse;
pub(crate) use assert_xml_format;
pub(crate) use assert_xml_roundtrip;

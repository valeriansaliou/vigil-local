// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::fmt;
use std::ops::Deref;

use regex;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug)]
pub struct Regex(regex::Regex);

impl Deref for Regex {
    type Target = regex::Regex;

    fn deref(&self) -> &regex::Regex {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Regex {
    fn deserialize<D>(de: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RegexVisitor;

        impl<'de> Visitor<'de> for RegexVisitor {
            type Value = Regex;

            fn expecting(&self, format: &mut fmt::Formatter) -> fmt::Result {
                format.write_str("a regular expression pattern")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Regex, E> {
                regex::Regex::new(value)
                    .map(Regex)
                    .map_err(|err| E::custom(err.to_string()))
            }
        }

        de.deserialize_str(RegexVisitor)
    }
}

// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

#[derive(Debug, PartialEq)]
pub enum Status {
    Healthy,
    Sick,
    Dead,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            &Status::Healthy => "healthy",
            &Status::Sick => "sick",
            &Status::Dead => "dead",
        }
    }
}

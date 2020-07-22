// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

#[derive(Deserialize)]
pub enum Mode {
    #[serde(rename = "poll")]
    Poll,

    #[serde(rename = "script")]
    Script,
}

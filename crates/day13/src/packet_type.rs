use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PacketType {
    Number(isize),
    List(Vec<PacketType>),
}

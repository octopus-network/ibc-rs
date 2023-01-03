//! Types for the IBC events emitted from Tendermint Websocket by the client module.

use derive_more::From;
use ibc_proto::google::protobuf::Any;
use subtle_encoding::hex;
use tendermint::abci;

use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::height::Height;
use crate::core::ics24_host::identifier::ClientId;
use crate::events::IbcEventType;
use crate::prelude::*;
use serde_derive::{Deserialize, Serialize};

/// The content of the `key` field for the attribute containing the client identifier.
pub const CLIENT_ID_ATTRIBUTE_KEY: &str = "client_id";

/// The content of the `key` field for the attribute containing the client type.
pub const CLIENT_TYPE_ATTRIBUTE_KEY: &str = "client_type";

/// The content of the `key` field for the attribute containing the height.
pub const CONSENSUS_HEIGHT_ATTRIBUTE_KEY: &str = "consensus_height";

pub const CONSENSUS_HEIGHTS_ATTRIBUTE_KEY: &str = "consensus_heights";

/// The content of the `key` field for the header in update client event.
pub const HEADER_ATTRIBUTE_KEY: &str = "header";

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, From, Serialize, Deserialize)]
struct ClientIdAttribute {
    client_id: ClientId,
}

impl From<ClientIdAttribute> for abci::EventAttribute {
    fn from(attr: ClientIdAttribute) -> Self {
        (CLIENT_ID_ATTRIBUTE_KEY, attr.client_id.as_str()).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, From, Serialize, Deserialize, PartialEq, Eq)]
struct ClientTypeAttribute {
    client_type: ClientType,
}

impl From<ClientTypeAttribute> for abci::EventAttribute {
    fn from(attr: ClientTypeAttribute) -> Self {
        (CLIENT_TYPE_ATTRIBUTE_KEY, attr.client_type.as_str()).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, From, Serialize, Deserialize)]
struct ConsensusHeightAttribute {
    consensus_height: Height,
}

impl From<ConsensusHeightAttribute> for abci::EventAttribute {
    fn from(attr: ConsensusHeightAttribute) -> Self {
        (CONSENSUS_HEIGHT_ATTRIBUTE_KEY, attr.consensus_height).into()
    }
}

#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, From, Serialize, Deserialize)]
struct ConsensusHeightsAttribute {
    consensus_heights: Vec<Height>,
}

impl From<ConsensusHeightsAttribute> for abci::EventAttribute {
    fn from(attr: ConsensusHeightsAttribute) -> Self {
        let consensus_heights: Vec<String> = attr
            .consensus_heights
            .into_iter()
            .map(|consensus_height| consensus_height.to_string())
            .collect();
        (CONSENSUS_HEIGHTS_ATTRIBUTE_KEY, consensus_heights.join(",")).into()
    }
}

#[derive(Debug, From, Serialize, Deserialize)]
struct HeaderAttribute {
    header: Any,
}

mod sealed {
    use super::*;

    #[cfg_attr(
        feature = "parity-scale-codec",
        derive(
            parity_scale_codec::Encode,
            parity_scale_codec::Decode,
            scale_info::TypeInfo
        )
    )]
    #[cfg_attr(
        feature = "borsh",
        derive(borsh::BorshSerialize, borsh::BorshDeserialize)
    )]
    pub struct InnerAny {
        pub type_url: String,
        pub value: Vec<u8>,
    }

    #[cfg(feature = "borsh")]
    impl borsh::BorshSerialize for HeaderAttribute {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> borsh::maybestd::io::Result<()> {
            let inner_any = InnerAny {
                type_url: self.header.type_url.clone(),
                value: self.header.value.clone(),
            };

            borsh::BorshSerialize::serialize(&inner_any, writer)
        }
    }

    #[cfg(feature = "borsh")]
    impl borsh::BorshDeserialize for HeaderAttribute {
        fn deserialize(buf: &mut &[u8]) -> borsh::maybestd::io::Result<Self> {
            let inner_any = InnerAny::deserialize(buf)?;

            Ok(HeaderAttribute {
                header: Any {
                    type_url: inner_any.type_url,
                    value: inner_any.value,
                },
            })
        }
    }

    #[cfg(feature = "parity-scale-codec")]
    impl parity_scale_codec::Encode for HeaderAttribute {
        fn encode_to<T: parity_scale_codec::Output + ?Sized>(&self, writer: &mut T) {
            let inner_any = InnerAny {
                type_url: self.header.type_url.clone(),
                value: self.header.value.clone(),
            };
            inner_any.encode_to(writer);
        }
    }
    #[cfg(feature = "parity-scale-codec")]
    impl parity_scale_codec::Decode for HeaderAttribute {
        fn decode<I: parity_scale_codec::Input>(
            input: &mut I,
        ) -> Result<Self, parity_scale_codec::Error> {
            let inner_any = InnerAny::decode(input)?;
            let header = Any {
                type_url: inner_any.type_url.clone(),
                value: inner_any.value.clone(),
            };

            Ok(HeaderAttribute { header })
        }
    }

    #[cfg(feature = "parity-scale-codec")]
    impl scale_info::TypeInfo for HeaderAttribute {
        type Identity = Self;

        fn type_info() -> scale_info::Type {
            scale_info::Type::builder()
                .path(scale_info::Path::new("HeaderAttribute", module_path!()))
                // i128 is chosen before we represent the timestamp is nanoseconds, which is represented as a i128 by Time
                .composite(
                    scale_info::build::Fields::named()
                        .field(|f| f.ty::<String>().name("type_url").type_name("String"))
                        .field(|f| f.ty::<Vec<u8>>().name("value").type_name("Vec<u8>")),
                )
        }
    }
}
impl From<HeaderAttribute> for abci::EventAttribute {
    fn from(attr: HeaderAttribute) -> Self {
        (
            HEADER_ATTRIBUTE_KEY,
            String::from_utf8(hex::encode(attr.header.value)).unwrap(),
        )
            .into()
    }
}

/// CreateClient event signals the creation of a new on-chain client (IBC client).
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClient {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
    consensus_height: ConsensusHeightAttribute,
}

impl CreateClient {
    pub fn new(client_id: ClientId, client_type: ClientType, consensus_height: Height) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
            consensus_height: ConsensusHeightAttribute::from(consensus_height),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn consensus_height(&self) -> &Height {
        &self.consensus_height.consensus_height
    }
}

impl From<CreateClient> for abci::Event {
    fn from(c: CreateClient) -> Self {
        Self {
            kind: IbcEventType::CreateClient.as_str().to_owned(),
            attributes: vec![
                c.client_id.into(),
                c.client_type.into(),
                c.consensus_height.into(),
            ],
        }
    }
}

/// UpdateClient event signals a recent update of an on-chain client (IBC Client).
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClient {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
    // Deprecated: consensus_height is deprecated and will be removed in a future release.
    // Please use consensus_heights instead.
    consensus_height: ConsensusHeightAttribute,
    consensus_heights: ConsensusHeightsAttribute,
    header: HeaderAttribute,
}

impl UpdateClient {
    pub fn new(
        client_id: ClientId,
        client_type: ClientType,
        consensus_height: Height,
        consensus_heights: Vec<Height>,
        header: Any,
    ) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
            consensus_height: ConsensusHeightAttribute::from(consensus_height),
            consensus_heights: ConsensusHeightsAttribute::from(consensus_heights),
            header: HeaderAttribute::from(header),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn consensus_height(&self) -> &Height {
        &self.consensus_height.consensus_height
    }

    pub fn consensus_heights(&self) -> &[Height] {
        self.consensus_heights.consensus_heights.as_ref()
    }

    pub fn header(&self) -> &Any {
        &self.header.header
    }
}

impl From<UpdateClient> for abci::Event {
    fn from(u: UpdateClient) -> Self {
        Self {
            kind: IbcEventType::UpdateClient.as_str().to_owned(),
            attributes: vec![
                u.client_id.into(),
                u.client_type.into(),
                u.consensus_height.into(),
                u.consensus_heights.into(),
                u.header.into(),
            ],
        }
    }
}

/// ClientMisbehaviour event signals the update of an on-chain client (IBC Client) with evidence of
/// misbehaviour.
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMisbehaviour {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
}

impl ClientMisbehaviour {
    pub fn new(client_id: ClientId, client_type: ClientType) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }
}

impl From<ClientMisbehaviour> for abci::Event {
    fn from(c: ClientMisbehaviour) -> Self {
        Self {
            kind: IbcEventType::ClientMisbehaviour.as_str().to_owned(),
            attributes: vec![c.client_id.into(), c.client_type.into()],
        }
    }
}

/// Signals a recent upgrade of an on-chain client (IBC Client).
#[cfg_attr(
    feature = "parity-scale-codec",
    derive(
        parity_scale_codec::Encode,
        parity_scale_codec::Decode,
        scale_info::TypeInfo
    )
)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeClient {
    client_id: ClientIdAttribute,
    client_type: ClientTypeAttribute,
    consensus_height: ConsensusHeightAttribute,
}

impl UpgradeClient {
    pub fn new(client_id: ClientId, client_type: ClientType, consensus_height: Height) -> Self {
        Self {
            client_id: ClientIdAttribute::from(client_id),
            client_type: ClientTypeAttribute::from(client_type),
            consensus_height: ConsensusHeightAttribute::from(consensus_height),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id.client_id
    }

    pub fn client_type(&self) -> &ClientType {
        &self.client_type.client_type
    }

    pub fn consensus_height(&self) -> &Height {
        &self.consensus_height.consensus_height
    }
}

impl From<UpgradeClient> for abci::Event {
    fn from(u: UpgradeClient) -> Self {
        Self {
            kind: IbcEventType::UpgradeClient.as_str().to_owned(),
            attributes: vec![
                u.client_id.into(),
                u.client_type.into(),
                u.consensus_height.into(),
            ],
        }
    }
}

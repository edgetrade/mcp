#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), ListEncryptedWalletsResponse> = Route {
    procedure: "agent.listEncryptedWallets",
    route_type: RouteType::Query,
    input_schema: PhantomData,
    output_schema: PhantomData,
};
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///A JSON-serializable record of chain types to wallet info
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A JSON-serializable record of chain types to wallet info",
///  "type": "object",
///  "required": [
///    "EVM",
///    "SVM"
///  ],
///  "additionalProperties": {
///    "anyOf": [
///      {
///        "description": "Information about a wallet",
///        "type": "object",
///        "required": [
///          "address",
///          "name"
///        ],
///        "properties": {
///          "address": {
///            "description": "The blockchain address of the wallet",
///            "type": "string",
///            "name": "Wallet Address"
///          },
///          "name": {
///            "description": "The human-readable name of the wallet",
///            "type": "string",
///            "name": "Wallet Name"
///          }
///        },
///        "additionalProperties": false,
///        "name": "Wallet Info"
///      },
///      {
///        "type": "null"
///      }
///    ]
///  },
///  "propertyNames": {
///    "description": "The blockchain type (EVM or SVM)",
///    "type": "string",
///    "enum": [
///      "EVM",
///      "SVM"
///    ],
///    "name": "Chain Type"
///  },
///  "name": "List Encrypted Wallets Output"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ListEncryptedWalletsResponse {
    #[serde(rename = "EVM")]
    pub evm: ::serde_json::Value,
    #[serde(rename = "SVM")]
    pub svm: ::serde_json::Value,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<
        ListEncryptedWalletsResponseExtraKey,
        ::std::option::Option<ListEncryptedWalletsResponseExtraValue>,
    >,
}
///The blockchain type (EVM or SVM)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The blockchain type (EVM or SVM)",
///  "type": "string",
///  "enum": [
///    "EVM",
///    "SVM"
///  ],
///  "name": "Chain Type"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ListEncryptedWalletsResponseExtraKey {
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "SVM")]
    Svm,
}
impl ::std::fmt::Display for ListEncryptedWalletsResponseExtraKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Evm => f.write_str("EVM"),
            Self::Svm => f.write_str("SVM"),
        }
    }
}
impl ::std::str::FromStr for ListEncryptedWalletsResponseExtraKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EVM" => Ok(Self::Evm),
            "SVM" => Ok(Self::Svm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListEncryptedWalletsResponseExtraKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ListEncryptedWalletsResponseExtraKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ListEncryptedWalletsResponseExtraKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Information about a wallet
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information about a wallet",
///  "type": "object",
///  "required": [
///    "address",
///    "name"
///  ],
///  "properties": {
///    "address": {
///      "description": "The blockchain address of the wallet",
///      "type": "string",
///      "name": "Wallet Address"
///    },
///    "name": {
///      "description": "The human-readable name of the wallet",
///      "type": "string",
///      "name": "Wallet Name"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Wallet Info"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListEncryptedWalletsResponseExtraValue {
    ///The blockchain address of the wallet
    pub address: ::std::string::String,
    ///The human-readable name of the wallet
    pub name: ::std::string::String,
}

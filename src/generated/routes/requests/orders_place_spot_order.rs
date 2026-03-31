#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PlaceSpotOrderRequest, PlaceSpotOrderResponse> = Route {
    procedure: "orders.placeSpotOrder",
    route_type: RouteType::Mutation,
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
///`PlaceSpotOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "envelope",
///    "order"
///  ],
///  "properties": {
///    "envelope": {
///      "description": "The encrypted envelope for the edge vault",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64",
///      "name": "Envelope"
///    },
///    "order": {
///      "description": "The order to place",
///      "type": "object",
///      "required": [
///        "amount",
///        "exitStrategyId",
///        "pairId",
///        "side",
///        "tokenId",
///        "txPreset",
///        "type",
///        "wallets"
///      ],
///      "properties": {
///        "amount": {
///          "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///          "oneOf": [
///            {
///              "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "native"
///                },
///                "value": {
///                  "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Native Amount"
///            },
///            {
///              "description": "The amount of the order in tokens",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "token"
///                },
///                "value": {
///                  "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Token Amount"
///            },
///            {
///              "description": "The amount of the order as a percentage; must be between 0 and 100",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "percentage"
///                },
///                "value": {
///                  "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Percentage Amount"
///            }
///          ],
///          "name": "Amount"
///        },
///        "exitStrategyId": {
///          "description": "The ID of the exit strategy to use for the order (optional)",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Exit Strategy ID"
///        },
///        "pairId": {
///          "description": "The ID of the pair to trade the token on",
///          "type": "object",
///          "required": [
///            "chainType",
///            "pairChainId",
///            "pairContractAddress"
///          ],
///          "properties": {
///            "chainType": {
///              "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///              "type": "string",
///              "enum": [
///                "EVM",
///                "SVM"
///              ],
///              "name": "Chain Type"
///            },
///            "pairChainId": {
///              "description": "The chain ID of the pair; stringified",
///              "type": "string",
///              "name": "Pair Chain ID"
///            },
///            "pairContractAddress": {
///              "description": "The contract address of the pair",
///              "type": "string",
///              "name": "Pair Contract Address"
///            }
///          },
///          "name": "Pair ID"
///        },
///        "side": {
///          "description": "The side of the order; must be \"buy\" or \"sell\"",
///          "type": "string",
///          "enum": [
///            "buy",
///            "sell"
///          ],
///          "name": "Side"
///        },
///        "tokenId": {
///          "description": "The ID of the token to trade",
///          "type": "object",
///          "required": [
///            "chainType",
///            "tokenChainId",
///            "tokenContractAddress"
///          ],
///          "properties": {
///            "chainType": {
///              "description": "The chain type of the token; must be \"EVM\" or \"SVM\"",
///              "type": "string",
///              "enum": [
///                "EVM",
///                "SVM"
///              ],
///              "name": "Chain Type"
///            },
///            "tokenChainId": {
///              "description": "The chain ID of the token; stringified",
///              "type": "string",
///              "name": "Token Chain ID"
///            },
///            "tokenContractAddress": {
///              "description": "The contract address of the token",
///              "type": "string",
///              "name": "Token Contract Address"
///            }
///          },
///          "name": "Token ID"
///        },
///        "txPreset": {
///          "type": "object",
///          "required": [
///            "bribe",
///            "maxBaseGas",
///            "maxPriceImpact",
///            "method",
///            "presetKey",
///            "priorityGas",
///            "slippage"
///          ],
///          "properties": {
///            "bribe": {
///              "description": "The bribe for the order",
///              "type": "string",
///              "name": "Bribe"
///            },
///            "maxBaseGas": {
///              "description": "The maximum base gas for the order",
///              "type": "string",
///              "name": "Max Base Gas"
///            },
///            "maxPriceImpact": {
///              "description": "The max price impact for the order",
///              "type": "string",
///              "name": "Max Price Impact"
///            },
///            "method": {
///              "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///              "type": "string",
///              "enum": [
///                "flashbot",
///                "normal"
///              ],
///              "name": "Method"
///            },
///            "presetKey": {
///              "description": "The key for the transaction preset",
///              "type": "string",
///              "name": "Preset Key"
///            },
///            "priorityGas": {
///              "description": "The priority gas for the order",
///              "type": "string",
///              "name": "Priority Gas"
///            },
///            "slippage": {
///              "description": "The slippage for the order",
///              "type": "string",
///              "name": "Slippage"
///            }
///          }
///        },
///        "type": {
///          "description": "The type of the order; must be \"spot\"",
///          "type": "string",
///          "const": "spot",
///          "name": "Type"
///        },
///        "wallets": {
///          "description": "The wallets that will be used to trade the token; must be an array of 1 wallet for agents",
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "address"
///            ],
///            "properties": {
///              "address": {
///                "type": "string"
///              }
///            }
///          },
///          "maxItems": 1,
///          "minItems": 1,
///          "name": "Wallets"
///        }
///      },
///      "name": "Order"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequest {
    ///The encrypted envelope for the edge vault
    pub envelope: ::std::string::String,
    pub order: PlaceSpotOrderRequestOrder,
}
///The order to place
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The order to place",
///  "type": "object",
///  "required": [
///    "amount",
///    "exitStrategyId",
///    "pairId",
///    "side",
///    "tokenId",
///    "txPreset",
///    "type",
///    "wallets"
///  ],
///  "properties": {
///    "amount": {
///      "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///      "oneOf": [
///        {
///          "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "native"
///            },
///            "value": {
///              "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Native Amount"
///        },
///        {
///          "description": "The amount of the order in tokens",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "token"
///            },
///            "value": {
///              "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Token Amount"
///        },
///        {
///          "description": "The amount of the order as a percentage; must be between 0 and 100",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "percentage"
///            },
///            "value": {
///              "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Percentage Amount"
///        }
///      ],
///      "name": "Amount"
///    },
///    "exitStrategyId": {
///      "description": "The ID of the exit strategy to use for the order (optional)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Exit Strategy ID"
///    },
///    "pairId": {
///      "description": "The ID of the pair to trade the token on",
///      "type": "object",
///      "required": [
///        "chainType",
///        "pairChainId",
///        "pairContractAddress"
///      ],
///      "properties": {
///        "chainType": {
///          "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///          "type": "string",
///          "enum": [
///            "EVM",
///            "SVM"
///          ],
///          "name": "Chain Type"
///        },
///        "pairChainId": {
///          "description": "The chain ID of the pair; stringified",
///          "type": "string",
///          "name": "Pair Chain ID"
///        },
///        "pairContractAddress": {
///          "description": "The contract address of the pair",
///          "type": "string",
///          "name": "Pair Contract Address"
///        }
///      },
///      "name": "Pair ID"
///    },
///    "side": {
///      "description": "The side of the order; must be \"buy\" or \"sell\"",
///      "type": "string",
///      "enum": [
///        "buy",
///        "sell"
///      ],
///      "name": "Side"
///    },
///    "tokenId": {
///      "description": "The ID of the token to trade",
///      "type": "object",
///      "required": [
///        "chainType",
///        "tokenChainId",
///        "tokenContractAddress"
///      ],
///      "properties": {
///        "chainType": {
///          "description": "The chain type of the token; must be \"EVM\" or \"SVM\"",
///          "type": "string",
///          "enum": [
///            "EVM",
///            "SVM"
///          ],
///          "name": "Chain Type"
///        },
///        "tokenChainId": {
///          "description": "The chain ID of the token; stringified",
///          "type": "string",
///          "name": "Token Chain ID"
///        },
///        "tokenContractAddress": {
///          "description": "The contract address of the token",
///          "type": "string",
///          "name": "Token Contract Address"
///        }
///      },
///      "name": "Token ID"
///    },
///    "txPreset": {
///      "type": "object",
///      "required": [
///        "bribe",
///        "maxBaseGas",
///        "maxPriceImpact",
///        "method",
///        "presetKey",
///        "priorityGas",
///        "slippage"
///      ],
///      "properties": {
///        "bribe": {
///          "description": "The bribe for the order",
///          "type": "string",
///          "name": "Bribe"
///        },
///        "maxBaseGas": {
///          "description": "The maximum base gas for the order",
///          "type": "string",
///          "name": "Max Base Gas"
///        },
///        "maxPriceImpact": {
///          "description": "The max price impact for the order",
///          "type": "string",
///          "name": "Max Price Impact"
///        },
///        "method": {
///          "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///          "type": "string",
///          "enum": [
///            "flashbot",
///            "normal"
///          ],
///          "name": "Method"
///        },
///        "presetKey": {
///          "description": "The key for the transaction preset",
///          "type": "string",
///          "name": "Preset Key"
///        },
///        "priorityGas": {
///          "description": "The priority gas for the order",
///          "type": "string",
///          "name": "Priority Gas"
///        },
///        "slippage": {
///          "description": "The slippage for the order",
///          "type": "string",
///          "name": "Slippage"
///        }
///      }
///    },
///    "type": {
///      "description": "The type of the order; must be \"spot\"",
///      "type": "string",
///      "const": "spot",
///      "name": "Type"
///    },
///    "wallets": {
///      "description": "The wallets that will be used to trade the token; must be an array of 1 wallet for agents",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "address"
///        ],
///        "properties": {
///          "address": {
///            "type": "string"
///          }
///        }
///      },
///      "maxItems": 1,
///      "minItems": 1,
///      "name": "Wallets"
///    }
///  },
///  "name": "Order"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrder {
    ///The amount of the order; a discriminated union of native, token, and percentage amounts
    pub amount: PlaceSpotOrderRequestOrderAmount,
    ///The ID of the exit strategy to use for the order (optional)
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: ::std::option::Option<f64>,
    #[serde(rename = "pairId")]
    pub pair_id: PlaceSpotOrderRequestOrderPairId,
    ///The side of the order; must be "buy" or "sell"
    pub side: PlaceSpotOrderRequestOrderSide,
    #[serde(rename = "tokenId")]
    pub token_id: PlaceSpotOrderRequestOrderTokenId,
    #[serde(rename = "txPreset")]
    pub tx_preset: PlaceSpotOrderRequestOrderTxPreset,
    ///The type of the order; must be "spot"
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    ///The wallets that will be used to trade the token; must be an array of 1 wallet for agents
    pub wallets: [PlaceSpotOrderRequestOrderWalletsItem; 1usize],
}
///The amount of the order; a discriminated union of native, token, and percentage amounts
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///  "oneOf": [
///    {
///      "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "native"
///        },
///        "value": {
///          "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Native Amount"
///    },
///    {
///      "description": "The amount of the order in tokens",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "token"
///        },
///        "value": {
///          "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Token Amount"
///    },
///    {
///      "description": "The amount of the order as a percentage; must be between 0 and 100",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "percentage"
///        },
///        "value": {
///          "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Percentage Amount"
///    }
///  ],
///  "name": "Amount"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum PlaceSpotOrderRequestOrderAmount {
    ///The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)
    #[serde(rename = "native")]
    Native(::std::string::String),
    ///The amount of the order in tokens
    #[serde(rename = "token")]
    Token(::std::string::String),
    ///The amount of the order as a percentage; must be between 0 and 100
    #[serde(rename = "percentage")]
    Percentage(::std::string::String),
}
///The ID of the pair to trade the token on
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The ID of the pair to trade the token on",
///  "type": "object",
///  "required": [
///    "chainType",
///    "pairChainId",
///    "pairContractAddress"
///  ],
///  "properties": {
///    "chainType": {
///      "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///      "type": "string",
///      "enum": [
///        "EVM",
///        "SVM"
///      ],
///      "name": "Chain Type"
///    },
///    "pairChainId": {
///      "description": "The chain ID of the pair; stringified",
///      "type": "string",
///      "name": "Pair Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the pair",
///      "type": "string",
///      "name": "Pair Contract Address"
///    }
///  },
///  "name": "Pair ID"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderPairId {
    ///The chain type of the pair; must be "EVM" or "SVM"
    #[serde(rename = "chainType")]
    pub chain_type: PlaceSpotOrderRequestOrderPairIdChainType,
    ///The chain ID of the pair; stringified
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    ///The contract address of the pair
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
}
///The chain type of the pair; must be "EVM" or "SVM"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
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
pub enum PlaceSpotOrderRequestOrderPairIdChainType {
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "SVM")]
    Svm,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderPairIdChainType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Evm => f.write_str("EVM"),
            Self::Svm => f.write_str("SVM"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderPairIdChainType {
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
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderPairIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderPairIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderPairIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The side of the order; must be "buy" or "sell"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The side of the order; must be \"buy\" or \"sell\"",
///  "type": "string",
///  "enum": [
///    "buy",
///    "sell"
///  ],
///  "name": "Side"
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
pub enum PlaceSpotOrderRequestOrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderSide {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The ID of the token to trade
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The ID of the token to trade",
///  "type": "object",
///  "required": [
///    "chainType",
///    "tokenChainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "chainType": {
///      "description": "The chain type of the token; must be \"EVM\" or \"SVM\"",
///      "type": "string",
///      "enum": [
///        "EVM",
///        "SVM"
///      ],
///      "name": "Chain Type"
///    },
///    "tokenChainId": {
///      "description": "The chain ID of the token; stringified",
///      "type": "string",
///      "name": "Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token",
///      "type": "string",
///      "name": "Token Contract Address"
///    }
///  },
///  "name": "Token ID"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderTokenId {
    ///The chain type of the token; must be "EVM" or "SVM"
    #[serde(rename = "chainType")]
    pub chain_type: PlaceSpotOrderRequestOrderTokenIdChainType,
    ///The chain ID of the token; stringified
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///The chain type of the token; must be "EVM" or "SVM"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The chain type of the token; must be \"EVM\" or \"SVM\"",
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
pub enum PlaceSpotOrderRequestOrderTokenIdChainType {
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "SVM")]
    Svm,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderTokenIdChainType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Evm => f.write_str("EVM"),
            Self::Svm => f.write_str("SVM"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderTokenIdChainType {
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
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderTokenIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderTokenIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderTokenIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderRequestOrderTxPreset`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bribe",
///    "maxBaseGas",
///    "maxPriceImpact",
///    "method",
///    "presetKey",
///    "priorityGas",
///    "slippage"
///  ],
///  "properties": {
///    "bribe": {
///      "description": "The bribe for the order",
///      "type": "string",
///      "name": "Bribe"
///    },
///    "maxBaseGas": {
///      "description": "The maximum base gas for the order",
///      "type": "string",
///      "name": "Max Base Gas"
///    },
///    "maxPriceImpact": {
///      "description": "The max price impact for the order",
///      "type": "string",
///      "name": "Max Price Impact"
///    },
///    "method": {
///      "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///      "type": "string",
///      "enum": [
///        "flashbot",
///        "normal"
///      ],
///      "name": "Method"
///    },
///    "presetKey": {
///      "description": "The key for the transaction preset",
///      "type": "string",
///      "name": "Preset Key"
///    },
///    "priorityGas": {
///      "description": "The priority gas for the order",
///      "type": "string",
///      "name": "Priority Gas"
///    },
///    "slippage": {
///      "description": "The slippage for the order",
///      "type": "string",
///      "name": "Slippage"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderTxPreset {
    ///The bribe for the order
    pub bribe: ::std::string::String,
    ///The maximum base gas for the order
    #[serde(rename = "maxBaseGas")]
    pub max_base_gas: ::std::string::String,
    ///The max price impact for the order
    #[serde(rename = "maxPriceImpact")]
    pub max_price_impact: ::std::string::String,
    ///The method for the order; must be "flashbot" or "normal". Only applies to EVM chains.
    pub method: PlaceSpotOrderRequestOrderTxPresetMethod,
    ///The key for the transaction preset
    #[serde(rename = "presetKey")]
    pub preset_key: ::std::string::String,
    ///The priority gas for the order
    #[serde(rename = "priorityGas")]
    pub priority_gas: ::std::string::String,
    ///The slippage for the order
    pub slippage: ::std::string::String,
}
///The method for the order; must be "flashbot" or "normal". Only applies to EVM chains.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///  "type": "string",
///  "enum": [
///    "flashbot",
///    "normal"
///  ],
///  "name": "Method"
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
pub enum PlaceSpotOrderRequestOrderTxPresetMethod {
    #[serde(rename = "flashbot")]
    Flashbot,
    #[serde(rename = "normal")]
    Normal,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderTxPresetMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Flashbot => f.write_str("flashbot"),
            Self::Normal => f.write_str("normal"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "flashbot" => Ok(Self::Flashbot),
            "normal" => Ok(Self::Normal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderRequestOrderWalletsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address"
///  ],
///  "properties": {
///    "address": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderWalletsItem {
    pub address: ::std::string::String,
}
///`PlaceSpotOrderResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "data",
///    "error",
///    "wallet"
///  ],
///  "properties": {
///    "data": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "error": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "wallet": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponse {
    pub data: ::std::option::Option<::std::string::String>,
    pub error: ::std::option::Option<::std::string::String>,
    pub wallet: ::std::string::String,
}

// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(message, repeated, tag="2")]
    pub withdraws: ::prost::alloc::vec::Vec<Withdraw>,
    #[prost(message, repeated, tag="3")]
    pub card_purchases: ::prost::alloc::vec::Vec<CardPurchase>,
    #[prost(message, repeated, tag="4")]
    pub generate_yields: ::prost::alloc::vec::Vec<GenerateYield>,
    #[prost(message, repeated, tag="5")]
    pub withdraw_yields: ::prost::alloc::vec::Vec<WithdrawYield>,
    #[prost(message, repeated, tag="6")]
    pub direct_card_purhcases: ::prost::alloc::vec::Vec<DirectCardPurchase>,
    #[prost(message, repeated, tag="7")]
    pub card_bot_user_account_inits: ::prost::alloc::vec::Vec<InitCardBotUserAccount>,
    #[prost(message, repeated, tag="8")]
    pub card_bot_direct_card_purchases: ::prost::alloc::vec::Vec<CardBotDirectCardPurchase>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub input_amount: u64,
    #[prost(uint64, tag="7")]
    pub output_amount: u64,
    #[prost(string, tag="8")]
    pub input_token: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub output_token: ::prost::alloc::string::String,
    #[prost(enumeration="DepositType", tag="10")]
    pub deposit_type: i32,
    #[prost(string, tag="11")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub user_vault: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub purchase_record: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Withdraw {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub amount: u64,
    #[prost(string, tag="7")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_vault: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub withdrawer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardPurchase {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub card_id: u64,
    #[prost(string, tag="7")]
    pub card_type: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub amount: u64,
    #[prost(string, tag="9")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub buyer_vault: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub purchase_record: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateYield {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub amount: u64,
    #[prost(string, tag="7")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub user_vault: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawYield {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub amount: u64,
    #[prost(bool, tag="7")]
    pub withdraw_all: bool,
    #[prost(string, tag="8")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub user_vault: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectCardPurchase {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub input_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub output_token: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub input_amount: u64,
    #[prost(uint64, tag="9")]
    pub output_amount: u64,
    #[prost(uint64, tag="10")]
    pub card_id: u64,
    #[prost(string, tag="11")]
    pub card_type: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub buyer_purchase: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub purchase_record: ::prost::alloc::string::String,
}
/// Card Bot Messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitCardBotUserAccount {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub user_custody: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub usdc_token: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub admin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardBotDirectCardPurchase {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(uint64, tag="2")]
    pub block_height: u64,
    #[prost(string, tag="3")]
    pub blockhash: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
    #[prost(string, tag="5")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub input_token: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub output_token: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub input_amount: u64,
    #[prost(uint64, tag="10")]
    pub output_amount: u64,
    #[prost(string, tag="11")]
    pub user_custody: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub admin: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DepositType {
    Unspecified = 0,
    Usdc = 1,
    Native = 2,
    NonNative = 3,
}
impl DepositType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DepositType::Unspecified => "DEPOSIT_TYPE_UNSPECIFIED",
            DepositType::Usdc => "DEPOSIT_TYPE_USDC",
            DepositType::Native => "DEPOSIT_TYPE_NATIVE",
            DepositType::NonNative => "DEPOSIT_TYPE_NON_NATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPOSIT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DEPOSIT_TYPE_USDC" => Some(Self::Usdc),
            "DEPOSIT_TYPE_NATIVE" => Some(Self::Native),
            "DEPOSIT_TYPE_NON_NATIVE" => Some(Self::NonNative),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)

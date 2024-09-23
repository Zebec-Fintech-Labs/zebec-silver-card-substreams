// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(string, tag="1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub destination: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub input_token: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub output_token: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub input_amount: u64,
    #[prost(uint64, tag="7")]
    pub output_amount: u64,
    #[prost(enumeration="DepositType", tag="8")]
    pub deposit_type: i32,
    #[prost(int64, tag="9")]
    pub timestamp: i64,
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

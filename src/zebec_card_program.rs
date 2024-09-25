// use std::*;
use crate::types::Pubkey;
use borsh;

pub mod typedefs {
    //! User-defined types.

    // use substreams_solana_program_instructions::pubkey::Pubkey;

    use super::*;
    #[derive(Debug, Default, Clone)]
    pub struct BuyPrepaidCardParams {
        pub index: u64,
        pub amount: u64,
        pub card_type: String,
    }

    // impl borsh::ser::BorshSerialize for BuyPrepaidCardParams
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     String: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.index, writer)?;
    //         borsh::BorshSerialize::serialize(&self.amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.card_type, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for BuyPrepaidCardParams
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                card_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Clone, Default, Copy)]
    pub struct DepositParams {
        pub amount: u64,
        pub token_type: TokenType,
    }

    // impl borsh::ser::BorshSerialize for DepositParams
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     TokenType: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.token_type, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for DepositParams
    where
        u64: borsh::BorshDeserialize,
        TokenType: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                token_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct InitCardConfigParams {
        pub native_fee: u64,
        pub non_native_fee: u64,
        pub revenue_fee: u64,
        pub card_vault: Pubkey,
        pub revenue_vault: Pubkey,
        pub commission_vault: Pubkey,
        pub min_card_amount: u64,
        pub max_card_amount: u64,
        pub daily_card_buy_limit: u64,
        pub fee_tier: Vec<FeeTier>,
    }

    // impl borsh::ser::BorshSerialize for InitCardConfigParams
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     Vec<FeeTier>: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.native_fee, writer)?;
    //         borsh::BorshSerialize::serialize(&self.non_native_fee, writer)?;
    //         borsh::BorshSerialize::serialize(&self.revenue_fee, writer)?;
    //         borsh::BorshSerialize::serialize(&self.card_vault, writer)?;
    //         borsh::BorshSerialize::serialize(&self.revenue_vault, writer)?;
    //         borsh::BorshSerialize::serialize(&self.commission_vault, writer)?;
    //         borsh::BorshSerialize::serialize(&self.min_card_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.max_card_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.daily_card_buy_limit, writer)?;
    //         borsh::BorshSerialize::serialize(&self.fee_tier, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for InitCardConfigParams
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Vec<FeeTier>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
                non_native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
                revenue_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
                card_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                revenue_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                commission_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                min_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                daily_card_buy_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_tier: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct SetCardConfigParams {
        pub native_fee: u64,
        pub non_native_fee: u64,
        pub revenue_fee: u64,
        pub card_vault: Pubkey,
        pub revenue_vault: Pubkey,
        pub commission_vault: Pubkey,
        pub zic_owner: Pubkey,
        pub min_card_amount: u64,
        pub max_card_amount: u64,
        pub daily_card_buy_limit: u64,
        pub fee_tier: Vec<FeeTier>,
    }

    // impl borsh::ser::BorshSerialize for SetCardConfigParams
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     Vec<FeeTier>: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.native_fee, writer)?;
    //         borsh::BorshSerialize::serialize(&self.non_native_fee, writer)?;
    //         borsh::BorshSerialize::serialize(&self.revenue_fee, writer)?;
    //         borsh::BorshSerialize::serialize(&self.card_vault, writer)?;
    //         borsh::BorshSerialize::serialize(&self.revenue_vault, writer)?;
    //         borsh::BorshSerialize::serialize(&self.commission_vault, writer)?;
    //         borsh::BorshSerialize::serialize(&self.zic_owner, writer)?;
    //         borsh::BorshSerialize::serialize(&self.min_card_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.max_card_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.daily_card_buy_limit, writer)?;
    //         borsh::BorshSerialize::serialize(&self.fee_tier, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for SetCardConfigParams
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Vec<FeeTier>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
                non_native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
                revenue_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
                card_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                revenue_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                commission_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                zic_owner: borsh::BorshDeserialize::deserialize_reader(reader)?,
                min_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                daily_card_buy_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_tier: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct ProviderConfig {
        pub min_card_amount: u64,
        pub max_card_amount: u64,
        pub fee_tiers: FeeMap,
    }

    // impl borsh::ser::BorshSerialize for ProviderConfig
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     FeeMap: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.min_card_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.max_card_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.fee_tiers, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for ProviderConfig
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        FeeMap: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                min_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_tiers: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    pub struct FeeTier {
        pub min_amount: u64,
        pub max_amount: u64,
        pub fee: u64,
    }

    // impl borsh::ser::BorshSerialize for FeeTier
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.min_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.max_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.fee, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for FeeTier
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                min_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct FeeMap {
        pub tiers: Vec<FeeTier>,
    }

    // impl borsh::ser::BorshSerialize for FeeMap
    // where
    //     Vec<FeeTier>: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.tiers, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for FeeMap
    where
        Vec<FeeTier>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                tiers: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TokenType {
        Native,
        NonNative,
        Usdc,
    }
    // #[automatically_derived]
    // impl core::marker::Copy for TokenType {}
    // impl borsh::ser::BorshSerialize for TokenType {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         let variant_idx: u8 = match self {
    //             TokenType::Native => 0u8,
    //             TokenType::NonNative => 1u8,
    //             TokenType::Usdc => 2u8,
    //         };
    //         writer.write_all(&variant_idx.to_le_bytes())?;
    //         match self {
    //             TokenType::Native => {}
    //             TokenType::NonNative => {}
    //             TokenType::Usdc => {}
    //         }
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for TokenType {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for TokenType {
        fn deserialize_variant<R: borsh::io::Read>(
            _reader: &mut R,
            variant_idx: u8,
        ) -> core::result::Result<Self, borsh::io::Error> {
            let return_value = match variant_idx {
                0u8 => TokenType::Native,
                1u8 => TokenType::NonNative,
                2u8 => TokenType::Usdc,
                _ => {
                    return Err(borsh::io::Error::new(borsh::io::ErrorKind::InvalidInput, {
                        let res = format!("Unexpected variant index: {0:?}", variant_idx);
                        res
                    }));
                }
            };
            Ok(return_value)
        }
    }
    impl Default for TokenType {
        fn default() -> Self {
            Self::Native
        }
    }
}

// pub mod state {
//     //! Structs of accounts which hold state.
//     use super::*;
//     /// Account: Card
//     #[derive(Default, Debug, Clone)]
//     pub struct Card {
//         pub index: u64,
//         pub zic_owner: Pubkey,
//         pub native_fee: u64,
//         pub non_native_fee: u64,
//         pub revenue_fee: u64,
//         pub usdc_mint: Pubkey,
//         pub revenue_vault: Pubkey,
//         pub commission_vault: Pubkey,
//         pub card_vault: Pubkey,
//         pub total_bought: u64,
//         pub daily_card_buy_limit: u64,
//         pub provider_config: typedefs::ProviderConfig,
//     }

//     impl borsh::ser::BorshSerialize for Card
//     where
//         u64: borsh::ser::BorshSerialize,
//         Pubkey: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         Pubkey: borsh::ser::BorshSerialize,
//         Pubkey: borsh::ser::BorshSerialize,
//         Pubkey: borsh::ser::BorshSerialize,
//         Pubkey: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         typedefs::ProviderConfig: borsh::ser::BorshSerialize,
//     {
//         fn serialize<W: borsh::io::Write>(
//             &self,
//             writer: &mut W,
//         ) -> core::result::Result<(), borsh::io::Error> {
//             borsh::BorshSerialize::serialize(&self.index, writer)?;
//             borsh::BorshSerialize::serialize(&self.zic_owner, writer)?;
//             borsh::BorshSerialize::serialize(&self.native_fee, writer)?;
//             borsh::BorshSerialize::serialize(&self.non_native_fee, writer)?;
//             borsh::BorshSerialize::serialize(&self.revenue_fee, writer)?;
//             borsh::BorshSerialize::serialize(&self.usdc_mint, writer)?;
//             borsh::BorshSerialize::serialize(&self.revenue_vault, writer)?;
//             borsh::BorshSerialize::serialize(&self.commission_vault, writer)?;
//             borsh::BorshSerialize::serialize(&self.card_vault, writer)?;
//             borsh::BorshSerialize::serialize(&self.total_bought, writer)?;
//             borsh::BorshSerialize::serialize(&self.daily_card_buy_limit, writer)?;
//             borsh::BorshSerialize::serialize(&self.provider_config, writer)?;
//             Ok(())
//         }
//     }
//     impl borsh::de::BorshDeserialize for Card
//     where
//         u64: borsh::BorshDeserialize,
//         Pubkey: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         Pubkey: borsh::BorshDeserialize,
//         Pubkey: borsh::BorshDeserialize,
//         Pubkey: borsh::BorshDeserialize,
//         Pubkey: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         typedefs::ProviderConfig: borsh::BorshDeserialize,
//     {
//         fn deserialize_reader<R: borsh::io::Read>(
//             reader: &mut R,
//         ) -> core::result::Result<Self, borsh::io::Error> {
//             Ok(Self {
//                 index: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 zic_owner: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 non_native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 revenue_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 usdc_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 revenue_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 commission_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 card_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 total_bought: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 daily_card_buy_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 provider_config: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             })
//         }
//     }

//     #[automatically_derived]
//     impl anchor_lang::AccountSerialize for Card {
//         fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
//             if writer
//                 .write_all(&[166, 250, 46, 230, 152, 63, 140, 182])
//                 .is_err()
//             {
//                 return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
//             }
//             if AnchorSerialize::serialize(self, writer).is_err() {
//                 return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
//             }
//             Ok(())
//         }
//     }
//     #[automatically_derived]
//     impl anchor_lang::AccountDeserialize for Card {
//         fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
//             if buf.len() < [166, 250, 46, 230, 152, 63, 140, 182].len() {
//                 return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
//             }
//             let given_disc = &buf[..8];
//             if &[166, 250, 46, 230, 152, 63, 140, 182] != given_disc {
//                 return Err(
//                     anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
//                         error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
//                             .name(),
//                         error_code_number:
//                             anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
//                         error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
//                             .to_string(),
//                         error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
//                             anchor_lang::error::Source {
//                                 filename: "src/lib.rs",
//                                 line: 1u32,
//                             },
//                         )),
//                         compared_values: None,
//                     })
//                     .with_account_name("Card"),
//                 );
//             }
//             Self::try_deserialize_unchecked(buf)
//         }
//         fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
//             let mut data: &[u8] = &buf[8..];
//             AnchorDeserialize::deserialize(&mut data)
//                 .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
//         }
//     }
//     #[automatically_derived]
//     impl Discriminator for Card {
//         const DISCRIMINATOR: [u8; 8] = [166, 250, 46, 230, 152, 63, 140, 182];
//     }
//     #[automatically_derived]
//     impl anchor_lang::Owner for Card {
//         fn owner() -> Pubkey {
//             ID
//         }
//     }
//     /// Account: PrepaidCardBuyer
//     #[derive(Debug, Default, Clone, Copy)]
//     pub struct PrepaidCardBuyer {
//         pub index: u64,
//         pub buyer_address: Pubkey,
//         pub amount: u64,
//         pub purchase_at: i64,
//     }
//     impl borsh::ser::BorshSerialize for PrepaidCardBuyer
//     where
//         u64: borsh::ser::BorshSerialize,
//         Pubkey: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         i64: borsh::ser::BorshSerialize,
//     {
//         fn serialize<W: borsh::io::Write>(
//             &self,
//             writer: &mut W,
//         ) -> core::result::Result<(), borsh::io::Error> {
//             borsh::BorshSerialize::serialize(&self.index, writer)?;
//             borsh::BorshSerialize::serialize(&self.buyer_address, writer)?;
//             borsh::BorshSerialize::serialize(&self.amount, writer)?;
//             borsh::BorshSerialize::serialize(&self.purchase_at, writer)?;
//             Ok(())
//         }
//     }
//     impl borsh::de::BorshDeserialize for PrepaidCardBuyer
//     where
//         u64: borsh::BorshDeserialize,
//         Pubkey: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         i64: borsh::BorshDeserialize,
//     {
//         fn deserialize_reader<R: borsh::io::Read>(
//             reader: &mut R,
//         ) -> core::result::Result<Self, borsh::io::Error> {
//             Ok(Self {
//                 index: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 buyer_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 purchase_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             })
//         }
//     }
//     #[automatically_derived]
//     impl anchor_lang::AccountSerialize for PrepaidCardBuyer {
//         fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
//             if writer
//                 .write_all(&[245, 103, 139, 240, 229, 33, 90, 141])
//                 .is_err()
//             {
//                 return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
//             }
//             if AnchorSerialize::serialize(self, writer).is_err() {
//                 return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
//             }
//             Ok(())
//         }
//     }
//     #[automatically_derived]
//     impl anchor_lang::AccountDeserialize for PrepaidCardBuyer {
//         fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
//             if buf.len() < [245, 103, 139, 240, 229, 33, 90, 141].len() {
//                 return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
//             }
//             let given_disc = &buf[..8];
//             if &[245, 103, 139, 240, 229, 33, 90, 141] != given_disc {
//                 return Err(
//                     anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
//                         error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
//                             .name(),
//                         error_code_number:
//                             anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
//                         error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
//                             .to_string(),
//                         error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
//                             anchor_lang::error::Source {
//                                 filename: "src/lib.rs",
//                                 line: 1u32,
//                             },
//                         )),
//                         compared_values: None,
//                     })
//                     .with_account_name("PrepaidCardBuyer"),
//                 );
//             }
//             Self::try_deserialize_unchecked(buf)
//         }
//         fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
//             let mut data: &[u8] = &buf[8..];
//             AnchorDeserialize::deserialize(&mut data)
//                 .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
//         }
//     }
//     #[automatically_derived]
//     impl Discriminator for PrepaidCardBuyer {
//         const DISCRIMINATOR: [u8; 8] = [245, 103, 139, 240, 229, 33, 90, 141];
//     }
//     #[automatically_derived]
//     impl anchor_lang::Owner for PrepaidCardBuyer {
//         fn owner() -> Pubkey {
//             ID
//         }
//     }
//     /// Account: Vault
//     #[derive(Default, Debug, Clone, Copy)]
//     pub struct Vault {
//         pub user_address: Pubkey,
//         pub total_bought_per_day: u64,
//         pub date_time_in_unix: u64,
//     }
//     impl borsh::ser::BorshSerialize for Vault
//     where
//         Pubkey: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//         u64: borsh::ser::BorshSerialize,
//     {
//         fn serialize<W: borsh::io::Write>(
//             &self,
//             writer: &mut W,
//         ) -> core::result::Result<(), borsh::io::Error> {
//             borsh::BorshSerialize::serialize(&self.user_address, writer)?;
//             borsh::BorshSerialize::serialize(&self.total_bought_per_day, writer)?;
//             borsh::BorshSerialize::serialize(&self.date_time_in_unix, writer)?;
//             Ok(())
//         }
//     }
//     impl borsh::de::BorshDeserialize for Vault
//     where
//         Pubkey: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//         u64: borsh::BorshDeserialize,
//     {
//         fn deserialize_reader<R: borsh::io::Read>(
//             reader: &mut R,
//         ) -> core::result::Result<Self, borsh::io::Error> {
//             Ok(Self {
//                 user_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 total_bought_per_day: borsh::BorshDeserialize::deserialize_reader(reader)?,
//                 date_time_in_unix: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             })
//         }
//     }
//     #[automatically_derived]
//     impl anchor_lang::AccountSerialize for Vault {
//         fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
//             if writer
//                 .write_all(&[211, 8, 232, 43, 2, 152, 117, 119])
//                 .is_err()
//             {
//                 return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
//             }
//             if AnchorSerialize::serialize(self, writer).is_err() {
//                 return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
//             }
//             Ok(())
//         }
//     }
//     #[automatically_derived]
//     impl anchor_lang::AccountDeserialize for Vault {
//         fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
//             if buf.len() < [211, 8, 232, 43, 2, 152, 117, 119].len() {
//                 return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
//             }
//             let given_disc = &buf[..8];
//             if &[211, 8, 232, 43, 2, 152, 117, 119] != given_disc {
//                 return Err(
//                     anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
//                         error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
//                             .name(),
//                         error_code_number:
//                             anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
//                         error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
//                             .to_string(),
//                         error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
//                             anchor_lang::error::Source {
//                                 filename: "src/lib.rs",
//                                 line: 1u32,
//                             },
//                         )),
//                         compared_values: None,
//                     })
//                     .with_account_name("Vault"),
//                 );
//             }
//             Self::try_deserialize_unchecked(buf)
//         }
//         fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
//             let mut data: &[u8] = &buf[8..];
//             AnchorDeserialize::deserialize(&mut data)
//                 .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
//         }
//     }
//     #[automatically_derived]
//     impl Discriminator for Vault {
//         const DISCRIMINATOR: [u8; 8] = [211, 8, 232, 43, 2, 152, 117, 119];
//     }
//     #[automatically_derived]
//     impl anchor_lang::Owner for Vault {
//         fn owner() -> Pubkey {
//             ID
//         }
//     }
// }

// pub use state::*;
use crate::types::Discriminator;
pub use typedefs::*;

pub mod instruction {

    use super::*;
    /// Instruction.
    pub struct InitCardConfigs {
        pub _params: typedefs::InitCardConfigParams,
    }
    // impl borsh::ser::BorshSerialize for InitCardConfigs
    // where
    //     typedefs::InitCardConfigParams: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._params, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for InitCardConfigs
    where
        typedefs::InitCardConfigParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for InitCardConfigs {
        const DISCRIMINATOR: [u8; 8] = [34, 81, 224, 7, 116, 120, 93, 93];
    }
    // impl anchor_lang::InstructionData for InitCardConfigs {}
    // impl anchor_lang::Owner for InitCardConfigs {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct Deposit {
        pub _params: typedefs::DepositParams,
    }
    // impl borsh::ser::BorshSerialize for Deposit
    // where
    //     typedefs::DepositParams: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._params, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for Deposit
    where
        typedefs::DepositParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for Deposit {
        const DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
    }
    // impl anchor_lang::InstructionData for Deposit {}
    // impl anchor_lang::Owner for Deposit {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct BuyPrepaidDigitalCard {
        pub _params: typedefs::BuyPrepaidCardParams,
    }
    // impl borsh::ser::BorshSerialize for BuyPrepaidDigitalCard
    // where
    //     typedefs::BuyPrepaidCardParams: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._params, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for BuyPrepaidDigitalCard
    where
        typedefs::BuyPrepaidCardParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for BuyPrepaidDigitalCard {
        const DISCRIMINATOR: [u8; 8] = [171, 194, 149, 170, 240, 124, 158, 125];
    }
    // impl anchor_lang::InstructionData for BuyPrepaidDigitalCard {}
    // impl anchor_lang::Owner for BuyPrepaidDigitalCard {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct GenerateYield {
        pub _amount: u64,
    }
    // impl borsh::ser::BorshSerialize for GenerateYield
    // where
    //     u64: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._amount, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for GenerateYield
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for GenerateYield {
        const DISCRIMINATOR: [u8; 8] = [249, 43, 72, 231, 65, 55, 83, 118];
    }
    // impl anchor_lang::InstructionData for GenerateYield {}
    // impl anchor_lang::Owner for GenerateYield {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct WithdrawYield {
        pub _amount: u64,
        pub _withdraw_all: bool,
    }
    // impl borsh::ser::BorshSerialize for WithdrawYield
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     bool: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self._withdraw_all, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for WithdrawYield
    where
        u64: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _withdraw_all: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for WithdrawYield {
        const DISCRIMINATOR: [u8; 8] = [62, 9, 132, 32, 96, 57, 101, 82];
    }
    // impl anchor_lang::InstructionData for WithdrawYield {}
    // impl anchor_lang::Owner for WithdrawYield {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct Withdraw {
        pub _amount: u64,
    }
    // impl borsh::ser::BorshSerialize for Withdraw
    // where
    //     u64: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._amount, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for Withdraw
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
    }
    // impl anchor_lang::InstructionData for Withdraw {}
    // impl anchor_lang::Owner for Withdraw {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct SetCardConfigs {
        pub _params: typedefs::SetCardConfigParams,
    }
    // impl borsh::ser::BorshSerialize for SetCardConfigs
    // where
    //     typedefs::SetCardConfigParams: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self._params, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for SetCardConfigs
    where
        typedefs::SetCardConfigParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                _params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for SetCardConfigs {
        const DISCRIMINATOR: [u8; 8] = [249, 192, 181, 54, 181, 252, 103, 66];
    }
    // impl anchor_lang::InstructionData for SetCardConfigs {}
    // impl anchor_lang::Owner for SetCardConfigs {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
}

/// The static program ID
pub static ID: Pubkey = [
    251u8, 247u8, 13u8, 243u8, 235u8, 51u8, 124u8, 87u8, 111u8, 74u8, 167u8, 228u8, 46u8, 74u8,
    43u8, 33u8, 14u8, 251u8, 183u8, 143u8, 143u8, 193u8, 126u8, 76u8, 41u8, 91u8, 214u8, 146u8,
    179u8, 245u8, 76u8, 164u8,
];

/// Returns the program ID
pub fn id() -> Pubkey {
    ID
}

pub fn id_bytes() -> Vec<u8> {
    id().to_vec()
}

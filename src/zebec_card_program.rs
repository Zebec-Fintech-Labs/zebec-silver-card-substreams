use crate::types::Pubkey;
// mod states {

// use crate::types::{Discriminator, Pubkey, Space};

// #[derive(Clone)]
// pub struct Card {
//     pub index: u64,
//     pub zic_owner: Pubkey,
//     pub native_fee: u64,
//     pub non_native_fee: u64,
//     pub revenue_fee: u64,
//     pub usdc_mint: Pubkey,
//     pub revenue_vault: Pubkey,
//     pub commission_vault: Pubkey,
//     pub card_vault: Pubkey,
//     pub total_bought: u64,
//     pub daily_card_buy_limit: u64,
//     pub provider_config: ProviderConfig,
// }
// impl Space for Card {
//     const INIT_SPACE: usize =
//         0 + 8 + 32 + 8 + 8 + 8 + 32 + 32 + 32 + 32 + 8 + 8 + <ProviderConfig>::INIT_SPACE;
// }

// impl Discriminator for Card {
//     const DISCRIMINATOR: [u8; 8] = [166, 250, 46, 230, 152, 63, 140, 182];
// }

// impl borsh::de::BorshDeserialize for Card
// where
//     u64: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     ProviderConfig: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             index: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             zic_owner: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             non_native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             revenue_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             usdc_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             revenue_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             commission_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             card_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             total_bought: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             daily_card_buy_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             provider_config: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct ProviderConfig {
//     pub min_card_amount: u64,
//     pub max_card_amount: u64,
//     pub fee_tiers: FeeMap,
// }

// impl Space for ProviderConfig {
//     const INIT_SPACE: usize = 0 + 8 + 8 + <FeeMap as Space>::INIT_SPACE;
// }

// impl borsh::de::BorshDeserialize for ProviderConfig
// where
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     FeeMap: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             min_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             max_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             fee_tiers: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct FeeTier {
//     min_amount: u64,
//     max_amount: u64,
//     fee: u64,
// }

// impl Space for FeeTier {
//     const INIT_SPACE: usize = 0 + 8 + 8 + 8;
// }

// impl borsh::de::BorshDeserialize for FeeTier
// where
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             min_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             max_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// impl FeeTier {
//     fn new(min_amount: u64, max_amount: u64, fee: u64) -> Self {
//         Self {
//             min_amount,
//             max_amount,
//             fee,
//         }
//     }
// }

// #[derive(Clone)]
// pub struct FeeMap {
//     tiers: Vec<FeeTier>,
// }

// impl Space for FeeMap {
//     const INIT_SPACE: usize = 0 + (4 + <FeeTier as Space>::INIT_SPACE * 5); // 5 mean max len is assumed to be 5
// }

// impl borsh::de::BorshDeserialize for FeeMap
// where
//     Vec<FeeTier>: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             tiers: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// impl Default for FeeMap {
//     fn default() -> Self {
//         Self { tiers: Vec::new() }
//     }
// }
// impl FeeMap {
//     pub fn get_fee(&self, amount: u64) -> Option<u64> {
//         match self.tiers.binary_search_by(|tier| {
//             if amount < tier.min_amount {
//                 Ordering::Less
//             } else if amount <= tier.max_amount {
//                 Ordering::Equal
//             } else {
//                 Ordering::Greater
//             }
//         }) {
//             Ok(index) => Some(self.tiers[index].fee),
//             Err(_) => None,
//         }
//     }
//     pub fn set_fee(&mut self, min_amount: u64, max_amount: u64, fee: u64) {
//         let index = self.tiers.binary_search_by(|tier| {
//             if min_amount < tier.min_amount {
//                 Ordering::Less
//             } else if min_amount == tier.min_amount {
//                 Ordering::Equal
//             } else {
//                 Ordering::Greater
//             }
//         });
//         match index {
//             Ok(i) => {
//                 self.tiers[i].fee = fee;
//             }
//             Err(i) => {
//                 self.tiers
//                     .insert(i, FeeTier::new(min_amount, max_amount, fee));
//             }
//         }
//     }
//     pub fn set_fee_using_vec(&mut self, tiers: Vec<FeeTier>) {
//         self.tiers = tiers;
//     }
// }

// #[derive(Clone)]
// pub struct CardBot {
//     pub bot_admin: Pubkey,
// }

// impl Space for CardBot {
//     const INIT_SPACE: usize = 0 + 32;
// }

// impl Discriminator for CardBot {
//     const DISCRIMINATOR: [u8; 8] = [13, 71, 227, 202, 175, 54, 231, 96];
// }

// impl borsh::de::BorshDeserialize for CardBot
// where
//     Pubkey: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             bot_admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct CardCustodyData {
//     pub user_id: String,
//     pub total_bought_per_day: u64,
//     pub date_time_in_unix: u64,
// }
// impl Space for CardCustodyData {
//     const INIT_SPACE: usize = 0 + (4 + 32) + 8 + 8;
// }

// impl Discriminator for CardCustodyData {
//     const DISCRIMINATOR: [u8; 8] = [195, 85, 79, 49, 52, 100, 253, 90];
// }

// impl borsh::de::BorshDeserialize for CardCustodyData
// where
//     String: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             user_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             total_bought_per_day: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             date_time_in_unix: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct CustomFeeMap {
//     fee_map: Vec<Fees>,
// }

// impl Space for CustomFeeMap {
//     const INIT_SPACE: usize = 0 + (4 + <Fees as Space>::INIT_SPACE * 250);
// }

// impl Discriminator for CustomFeeMap {
//     const DISCRIMINATOR: [u8; 8] = [127, 235, 106, 68, 238, 133, 121, 187];
// }

// impl borsh::de::BorshDeserialize for CustomFeeMap
// where
//     Vec<Fees>: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             fee_map: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct Fees {
//     pub token_address: Pubkey,
//     pub fee: u64,
// }

// impl Fees {
//     fn new(token_address: Pubkey, fee: u64) -> Self {
//         Self { token_address, fee }
//     }
// }

// impl Space for Fees {
//     const INIT_SPACE: usize = 0 + 32 + 8;
// }

// impl borsh::de::BorshDeserialize for Fees
// where
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             token_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// impl Default for CustomFeeMap {
//     fn default() -> Self {
//         Self {
//             fee_map: Vec::new(),
//         }
//     }
// }
// impl CustomFeeMap {
//     pub fn set_custom_fee_one(&mut self, token_address: Pubkey, fee: u64) {
//         for f in self.fee_map.iter_mut() {
//             if f.token_address == token_address {
//                 f.fee = fee;
//                 return;
//             }
//         }
//         self.fee_map.push(Fees::new(token_address, fee));
//     }
//     pub fn set_custom_fee(&mut self, fees: Vec<Fees>) {
//         for f in fees.iter() {
//             self.set_custom_fee_one(f.token_address, f.fee);
//         }
//     }
//     pub fn delete_custom_fee(&mut self, token_address: Pubkey) {
//         self.fee_map.retain(|f| f.token_address != token_address);
//     }
//     pub fn get_custom_fee(&self, token_address: Pubkey) -> Option<u64> {
//         for f in self.fee_map.iter() {
//             if f.token_address == token_address {
//                 return Some(f.fee);
//             }
//         }
//         Some(500)
//     }
// }

// #[derive(Clone)]
// pub struct OnRamp {
//     pub admin: Pubkey,
//     pub zbcn_token: Pubkey,
// }

// impl Space for OnRamp {
//     const INIT_SPACE: usize = 0 + 32 + 32;
// }

// impl Discriminator for OnRamp {
//     const DISCRIMINATOR: [u8; 8] = [126, 68, 64, 246, 54, 233, 218, 241];
// }

// impl borsh::de::BorshDeserialize for OnRamp
// where
//     Pubkey: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             zbcn_token: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct OnRampCustody {
//     pub user_id: String,
// }

// impl Space for OnRampCustody {
//     const INIT_SPACE: usize = 0 + (4 + 32);
// }

// impl Discriminator for OnRampCustody {
//     const DISCRIMINATOR: [u8; 8] = [187, 4, 79, 142, 223, 152, 58, 235];
// }

// impl borsh::de::BorshDeserialize for OnRampCustody
// where
//     String: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             user_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct PrepaidCardBuyer {
//     pub index: u64,
//     pub buyer_address: Pubkey,
//     pub amount: u64,
//     pub purchase_at: i64,
// }
// impl Space for PrepaidCardBuyer {
//     const INIT_SPACE: usize = 0 + 8 + 32 + 8 + 8;
// }

// impl Discriminator for PrepaidCardBuyer {
//     const DISCRIMINATOR: [u8; 8] = [245, 103, 139, 240, 229, 33, 90, 141];
// }

// impl borsh::de::BorshDeserialize for PrepaidCardBuyer
// where
//     u64: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     i64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             index: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             buyer_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             purchase_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct PurchaseRecord {
//     pub user_address: Pubkey,
//     pub total_bought_per_day: u64,
//     pub date_time_in_unix: u64,
// }

// impl Space for PurchaseRecord {
//     const INIT_SPACE: usize = 0 + 32 + 8 + 8;
// }

// impl Discriminator for PurchaseRecord {
//     const DISCRIMINATOR: [u8; 8] = [211, 8, 232, 43, 2, 152, 117, 119];
// }

// impl borsh::de::BorshDeserialize for PurchaseRecord
// where
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             user_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             total_bought_per_day: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             date_time_in_unix: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct ProposalConfigs {
//     pub staking_token: Pubkey,
//     pub creator_staking_cap: u64,
//     pub voters_staking_cap: u64,
// }
// // #[automatically_derived]
// impl Space for ProposalConfigs {
//     const INIT_SPACE: usize = 0 + 32 + 8 + 8;
// }

// impl Discriminator for ProposalConfigs {
//     const DISCRIMINATOR: [u8; 8] = [216, 204, 153, 70, 82, 180, 161, 57];
// }

// impl borsh::de::BorshDeserialize for ProposalConfigs
// where
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             staking_token: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             creator_staking_cap: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             voters_staking_cap: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct Proposal {
//     pub creator: Pubkey,
//     // #[max_len(32)]
//     pub name: String,
//     pub start_time: i64,
//     pub end_time: i64,
//     pub total_votes: u64,
// }

// impl Space for Proposal {
//     const INIT_SPACE: usize = 0 + 32 + (4 + 32) + 8 + 8 + 8;
// }

// impl Discriminator for Proposal {
//     const DISCRIMINATOR: [u8; 8] = [26, 94, 189, 187, 116, 136, 53, 33];
// }

// impl borsh::de::BorshDeserialize for Proposal
// where
//     Pubkey: borsh::BorshDeserialize,
//     String: borsh::BorshDeserialize,
//     i64: borsh::BorshDeserialize,
//     i64: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             name: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             start_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             end_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             total_votes: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }

// #[derive(Clone)]
// pub struct Voter {
//     pub voter_address: Pubkey,
//     pub proposal_address: Pubkey,
//     pub voting_amount: u64,
//     pub vote_status: bool,
//     pub voter_flag: bool,
//     pub is_withdrawn: bool,
// }
// // #[automatically_derived]
// impl Space for Voter {
//     const INIT_SPACE: usize = 0 + 32 + 32 + 8 + 1 + 1 + 1;
// }

// impl Discriminator for Voter {
//     const DISCRIMINATOR: [u8; 8] = [241, 93, 35, 191, 254, 147, 17, 202];
// }

// impl borsh::de::BorshDeserialize for Voter
// where
//     Pubkey: borsh::BorshDeserialize,
//     Pubkey: borsh::BorshDeserialize,
//     u64: borsh::BorshDeserialize,
//     bool: borsh::BorshDeserialize,
//     bool: borsh::BorshDeserialize,
//     bool: borsh::BorshDeserialize,
// {
//     fn deserialize_reader<R: borsh::io::Read>(
//         reader: &mut R,
//     ) -> ::core::result::Result<Self, borsh::io::Error> {
//         Ok(Self {
//             voter_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             proposal_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             voting_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             vote_status: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             voter_flag: borsh::BorshDeserialize::deserialize_reader(reader)?,
//             is_withdrawn: borsh::BorshDeserialize::deserialize_reader(reader)?,
//         })
//     }
// }
// }

/// Confirms that a given pubkey is equivalent to the program ID
// pub fn check_id(id: &Pubkey) -> bool {
//     id == &[
//         251u8, 247u8, 13u8, 243u8, 235u8, 51u8, 124u8, 87u8, 111u8, 74u8, 167u8, 228u8, 46u8, 74u8,
//         43u8, 33u8, 14u8, 251u8, 183u8, 143u8, 143u8, 193u8, 126u8, 76u8, 41u8, 91u8, 214u8, 146u8,
//         179u8, 245u8, 76u8, 164u8,
//     ]
// }

/// Returns the program ID
pub fn id() -> Pubkey {
    [
        251u8, 247u8, 13u8, 243u8, 235u8, 51u8, 124u8, 87u8, 111u8, 74u8, 167u8, 228u8, 46u8, 74u8,
        43u8, 33u8, 14u8, 251u8, 183u8, 143u8, 143u8, 193u8, 126u8, 76u8, 41u8, 91u8, 214u8, 146u8,
        179u8, 245u8, 76u8, 164u8,
    ]
}

pub fn id_bytes() -> Vec<u8> {
    id().to_vec()
}

pub mod instruction {
    use crate::types::Discriminator;

    use super::*;

    /// Instruction.
    // pub struct InitCardConfigs {
    //     pub params: InitCardConfigParams,
    // }

    // pub struct InitCardConfigParams {
    //     pub native_fee: u64,
    //     pub non_native_fee: u64,
    //     pub revenue_fee: u64,
    //     pub card_vault: Pubkey,
    //     pub revenue_vault: Pubkey,
    //     pub commission_vault: Pubkey,
    //     pub min_card_amount: u64,
    //     pub max_card_amount: u64,
    //     pub daily_card_buy_limit: u64,
    //     pub fee_tier: Vec<FeeTier>,
    // }

    // impl borsh::de::BorshDeserialize for InitCardConfigParams
    // where
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     Vec<FeeTier>: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             non_native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             revenue_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             card_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             revenue_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             commission_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             min_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             max_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             daily_card_buy_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             fee_tier: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for InitCardConfigs
    // where
    //     InitCardConfigParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for InitCardConfigs {
    //     const DISCRIMINATOR: [u8; 8] = [34, 81, 224, 7, 116, 120, 93, 93];
    // }

    pub struct DepositV2 {
        pub params: DepositParamsV2,
    }

    pub struct DepositParamsV2 {
        pub amount: u64,                  // 8
        pub source_token_address: Pubkey, // 32
    }

    impl borsh::de::BorshDeserialize for DepositParamsV2
    where
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                source_token_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    impl borsh::de::BorshDeserialize for DepositV2
    where
        DepositParamsV2: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for DepositV2 {
        const DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
    }

    #[derive(Debug, Clone, Default)]
    pub struct DepositParamsV1 {
        pub amount: u64,             // 8
        pub token_type: TokenTypeV1, // 1
    }

    impl borsh::de::BorshDeserialize for DepositParamsV1
    where
        u64: borsh::BorshDeserialize,
        TokenTypeV1: borsh::BorshDeserialize,
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

    #[derive(Debug, Clone, Copy)]
    pub enum TokenTypeV1 {
        Native,
        NonNative,
        Usdc,
    }

    impl borsh::de::BorshDeserialize for TokenTypeV1 {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for TokenTypeV1 {
        fn deserialize_variant<R: borsh::io::Read>(
            _reader: &mut R,
            variant_idx: u8,
        ) -> core::result::Result<Self, borsh::io::Error> {
            let return_value = match variant_idx {
                0u8 => TokenTypeV1::Native,
                1u8 => TokenTypeV1::NonNative,
                2u8 => TokenTypeV1::Usdc,
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
    impl Default for TokenTypeV1 {
        fn default() -> Self {
            Self::Native
        }
    }

    pub struct DepositV1 {
        pub params: DepositParamsV1,
    }

    impl borsh::de::BorshDeserialize for DepositV1
    where
        DepositParamsV1: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    impl Discriminator for DepositV1 {
        const DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
    }

    pub struct BuyPrepaidDigitalCard {
        pub params: BuyPrepaidCardParams,
    }

    pub struct BuyPrepaidCardParams {
        pub index: u64,
        pub amount: u64,
        pub card_type: String,
    }

    impl borsh::de::BorshDeserialize for BuyPrepaidCardParams
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                card_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    impl borsh::de::BorshDeserialize for BuyPrepaidDigitalCard
    where
        BuyPrepaidCardParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for BuyPrepaidDigitalCard {
        const DISCRIMINATOR: [u8; 8] = [171, 194, 149, 170, 240, 124, 158, 125];
    }
    // impl InstructionData for BuyPrepaidDigitalCard {}
    // impl Owner for BuyPrepaidDigitalCard {
    //     fn owner() -> Pubkey {
    //         ID
    //     }
    // }
    /// Instruction.
    pub struct BuyCardDirect {
        pub params: BuyCardDirectParams,
    }
    pub struct BuyCardDirectParams {
        pub index: u64,
        pub amount: u64,
        pub card_type: String,
        pub source_token_address: Pubkey,
    }

    impl borsh::de::BorshDeserialize for BuyCardDirectParams
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        String: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                card_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
                source_token_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::de::BorshDeserialize for BuyCardDirect
    where
        BuyCardDirectParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for BuyCardDirect {
        const DISCRIMINATOR: [u8; 8] = [118, 94, 183, 2, 126, 224, 70, 136];
    }

    pub struct GenerateYield {
        pub amount: u64,
    }

    impl borsh::de::BorshDeserialize for GenerateYield
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for GenerateYield {
        const DISCRIMINATOR: [u8; 8] = [249, 43, 72, 231, 65, 55, 83, 118];
    }

    pub struct WithdrawYield {
        pub amount: u64,
        pub withdraw_all: bool,
    }

    impl borsh::de::BorshDeserialize for WithdrawYield
    where
        u64: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                withdraw_all: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for WithdrawYield {
        const DISCRIMINATOR: [u8; 8] = [62, 9, 132, 32, 96, 57, 101, 82];
    }

    pub struct Withdraw {
        pub amount: u64,
    }

    impl borsh::de::BorshDeserialize for Withdraw
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
    }

    // pub struct SetCardConfigs {
    //     pub params: SetCardConfigParams,
    // }

    // pub struct SetCardConfigParams {
    //     pub native_fee: u64,
    //     pub non_native_fee: u64,
    //     pub revenue_fee: u64,
    //     pub card_vault: Pubkey,
    //     pub revenue_vault: Pubkey,
    //     pub commission_vault: Pubkey,
    //     pub zic_owner: Pubkey,
    //     pub min_card_amount: u64,
    //     pub max_card_amount: u64,
    //     pub daily_card_buy_limit: u64,
    //     pub fee_tier: Vec<FeeTier>,
    // }

    // impl borsh::de::BorshDeserialize for SetCardConfigParams
    // where
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     Vec<FeeTier>: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             non_native_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             revenue_fee: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             card_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             revenue_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             commission_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             zic_owner: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             min_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             max_card_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             daily_card_buy_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             fee_tier: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for SetCardConfigs
    // where
    //     SetCardConfigParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for SetCardConfigs {
    //     const DISCRIMINATOR: [u8; 8] = [249, 192, 181, 54, 181, 252, 103, 66];
    // }

    // pub struct SetCustomFees {
    //     pub params: Vec<Fees>,
    // }

    // impl borsh::de::BorshDeserialize for SetCustomFees
    // where
    //     Vec<Fees>: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for SetCustomFees {
    //     const DISCRIMINATOR: [u8; 8] = [114, 185, 193, 242, 211, 79, 131, 180];
    // }

    // pub struct DeleteCustomFees {
    //     pub params: Vec<Pubkey>,
    // }

    // impl borsh::de::BorshDeserialize for DeleteCustomFees
    // where
    //     Vec<Pubkey>: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for DeleteCustomFees {
    //     const DISCRIMINATOR: [u8; 8] = [86, 194, 127, 189, 66, 97, 134, 230];
    // }

    // pub struct InitBotConfig {
    //     pub params: InitBotConfigParams,
    // }

    // pub struct InitBotConfigParams {
    //     pub bot_admin: Pubkey,
    // }

    // impl borsh::de::BorshDeserialize for InitBotConfigParams
    // where
    //     Pubkey: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             bot_admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl borsh::de::BorshDeserialize for InitBotConfig
    // where
    //     InitBotConfigParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for InitBotConfig {
    //     const DISCRIMINATOR: [u8; 8] = [43, 92, 23, 39, 30, 35, 230, 234];
    // }

    pub struct BuyCardBot {
        pub params: BuyCardBotParams,
    }
    pub struct BuyCardBotParams {
        pub user_id: String,
        pub amount: u64,
        pub source_token_address: Pubkey,
    }

    impl borsh::de::BorshDeserialize for BuyCardBotParams
    where
        String: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                user_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                source_token_address: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl borsh::de::BorshDeserialize for BuyCardBot
    where
        BuyCardBotParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for BuyCardBot {
        const DISCRIMINATOR: [u8; 8] = [69, 20, 13, 69, 156, 51, 4, 99];
    }
    pub struct InitBotPda {
        pub params: InitBotPdaParams,
    }

    pub struct InitBotPdaParams {
        pub user_id: String,
    }

    impl borsh::de::BorshDeserialize for InitBotPdaParams
    where
        String: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                user_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    impl borsh::de::BorshDeserialize for InitBotPda
    where
        InitBotPdaParams: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                params: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl Discriminator for InitBotPda {
        const DISCRIMINATOR: [u8; 8] = [28, 168, 3, 27, 146, 154, 208, 106];
    }

    // pub struct SetBotAdmin {
    //     pub new_admin: Pubkey,
    // }

    // impl borsh::de::BorshDeserialize for SetBotAdmin
    // where
    //     Pubkey: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             new_admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for SetBotAdmin {
    //     const DISCRIMINATOR: [u8; 8] = [221, 58, 231, 50, 206, 94, 206, 123];
    // }

    // pub struct InitOnRampConfig {
    //     pub params: InitOnRampParams,
    // }

    // pub struct InitOnRampParams {
    //     pub admin: Pubkey,
    //     pub zbcn_token: Pubkey,
    // }

    // impl borsh::de::BorshDeserialize for InitOnRampParams
    // where
    //     Pubkey: borsh::BorshDeserialize,
    //     Pubkey: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             zbcn_token: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for InitOnRampConfig
    // where
    //     InitOnRampParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for InitOnRampConfig {
    //     const DISCRIMINATOR: [u8; 8] = [20, 156, 140, 216, 220, 187, 190, 241];
    // }

    // pub struct InitOnRampPda {
    //     pub params: InitOnRampPdaParams,
    // }

    // pub struct InitOnRampPdaParams {
    //     pub user_id: String,
    // }

    // impl borsh::de::BorshDeserialize for InitOnRampPdaParams
    // where
    //     String: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             user_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for InitOnRampPda
    // where
    //     InitOnRampPdaParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for InitOnRampPda {
    //     const DISCRIMINATOR: [u8; 8] = [178, 247, 21, 17, 243, 75, 149, 66];
    // }

    // pub struct SetOnRampAdmin {
    //     pub new_admin: Pubkey,
    // }

    // impl borsh::de::BorshDeserialize for SetOnRampAdmin
    // where
    //     Pubkey: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             new_admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for SetOnRampAdmin {
    //     const DISCRIMINATOR: [u8; 8] = [18, 40, 8, 133, 143, 135, 33, 255];
    // }

    // pub struct TransferZbcn {
    //     pub params: TransferZbcnParams,
    // }

    // pub struct TransferZbcnParams {
    //     pub amount: u64,
    //     pub user_id: String,
    // }
    // impl borsh::ser::BorshSerialize for TransferZbcnParams
    // where
    //     u64: borsh::ser::BorshSerialize,
    //     String: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> ::core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.user_id, writer)?;
    //         Ok(())
    //     }
    // }
    // impl borsh::de::BorshDeserialize for TransferZbcnParams
    // where
    //     u64: borsh::BorshDeserialize,
    //     String: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             user_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for TransferZbcn
    // where
    //     TransferZbcnParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl Discriminator for TransferZbcn {
    //     const DISCRIMINATOR: [u8; 8] = [90, 47, 186, 212, 205, 108, 170, 41];
    // }

    // pub struct InitProposalConfigs {
    //     pub params: InitProposalConfigParams,
    // }

    // pub struct InitProposalConfigParams {
    //     pub staking_token: Pubkey,
    //     pub creator_staking_cap: u64,
    //     pub voters_staking_cap: u64,
    // }
    // impl borsh::ser::BorshSerialize for InitProposalConfigParams
    // where
    //     Pubkey: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> ::core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.staking_token, writer)?;
    //         borsh::BorshSerialize::serialize(&self.creator_staking_cap, writer)?;
    //         borsh::BorshSerialize::serialize(&self.voters_staking_cap, writer)?;
    //         Ok(())
    //     }
    // }
    // impl borsh::de::BorshDeserialize for InitProposalConfigParams
    // where
    //     Pubkey: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             staking_token: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             creator_staking_cap: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             voters_staking_cap: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for InitProposalConfigs
    // where
    //     InitProposalConfigParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for InitProposalConfigs {
    //     const DISCRIMINATOR: [u8; 8] = [40, 42, 108, 87, 192, 61, 218, 25];
    // }

    // pub struct InitProposal {
    //     pub params: InitProposalParams,
    // }

    // pub struct InitProposalParams {
    //     pub name: String,
    //     pub voting_start_time: i64,
    //     pub voting_end_time: i64,
    // }

    // impl borsh::de::BorshDeserialize for InitProposalParams
    // where
    //     String: borsh::BorshDeserialize,
    //     i64: borsh::BorshDeserialize,
    //     i64: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             name: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             voting_start_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             voting_end_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for InitProposal
    // where
    //     InitProposalParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for InitProposal {
    //     const DISCRIMINATOR: [u8; 8] = [113, 76, 165, 176, 110, 138, 198, 178];
    // }

    // pub struct Vote {
    //     pub params: VoteParams,
    // }

    // pub struct VoteParams {
    //     pub vote_status: bool,
    //     pub voting_amount: u64,
    // }

    // impl borsh::de::BorshDeserialize for VoteParams
    // where
    //     bool: borsh::BorshDeserialize,
    //     u64: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             vote_status: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //             voting_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }

    // impl borsh::de::BorshDeserialize for Vote
    // where
    //     VoteParams: borsh::BorshDeserialize,
    // {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {
    //             params: borsh::BorshDeserialize::deserialize_reader(reader)?,
    //         })
    //     }
    // }
    // impl Discriminator for Vote {
    //     const DISCRIMINATOR: [u8; 8] = [227, 110, 155, 23, 136, 126, 172, 25];
    // }

    // pub struct WithdrawVote;

    // impl borsh::de::BorshDeserialize for WithdrawVote {
    //     fn deserialize_reader<R: borsh::io::Read>(
    //         _reader: &mut R,
    //     ) -> ::core::result::Result<Self, borsh::io::Error> {
    //         Ok(Self {})
    //     }
    // }
    // impl Discriminator for WithdrawVote {
    //     const DISCRIMINATOR: [u8; 8] = [243, 255, 70, 200, 3, 242, 103, 137];
    // }
}

use borsh;

use crate::types::{Discriminator, Pubkey};

pub mod events {
    use super::*;

    pub struct SwapEvent {
        pub amm: Pubkey,
        pub input_mint: Pubkey,
        pub input_amount: u64,
        pub output_mint: Pubkey,
        pub output_amount: u64,
    }
    // impl borsh::ser::BorshSerialize for SwapEvent
    // where
    //     Pubkey: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    //     Pubkey: borsh::ser::BorshSerialize,
    //     u64: borsh::ser::BorshSerialize,
    // {
    //     fn serialize<W: borsh::io::Write>(
    //         &self,
    //         writer: &mut W,
    //     ) -> ::core::result::Result<(), borsh::io::Error> {
    //         borsh::BorshSerialize::serialize(&self.amm, writer)?;
    //         borsh::BorshSerialize::serialize(&self.input_mint, writer)?;
    //         borsh::BorshSerialize::serialize(&self.input_amount, writer)?;
    //         borsh::BorshSerialize::serialize(&self.output_mint, writer)?;
    //         borsh::BorshSerialize::serialize(&self.output_amount, writer)?;
    //         Ok(())
    //     }
    // }
    impl borsh::de::BorshDeserialize for SwapEvent
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::io::Error> {
            Ok(Self {
                amm: borsh::BorshDeserialize::deserialize_reader(reader)?,
                input_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                input_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                output_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                output_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }

    impl Discriminator for SwapEvent {
        const DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
    }
}

pub static ID: Pubkey = [
    4u8, 121u8, 213u8, 91u8, 242u8, 49u8, 192u8, 110u8, 238u8, 116u8, 197u8, 110u8, 206u8, 104u8,
    21u8, 7u8, 253u8, 177u8, 178u8, 222u8, 163u8, 244u8, 142u8, 81u8, 2u8, 177u8, 205u8, 162u8,
    86u8, 188u8, 19u8, 143u8,
];

pub fn id() -> Pubkey {
    ID
}

pub fn id_bytes() -> Vec<u8> {
    id().to_vec()
}

pub type Pubkey = [u8; 32];

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
}

pub type Pubkey = [u8; 32];

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
}

#[allow(dead_code)]
pub trait Space {
    const INIT_SPACE: usize;
}

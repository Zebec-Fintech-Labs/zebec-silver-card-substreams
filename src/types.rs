pub type Pubkey = [u8; 32];

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
}

pub trait Space {
    const INIT_SPACE: usize;
}

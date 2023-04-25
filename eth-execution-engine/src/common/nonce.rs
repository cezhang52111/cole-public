use primitive_types::U256;
use core::ops::{Add, AddAssign, Sub, SubAssign};
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
)]
pub struct Nonce(pub U256);

macro_rules! impl_nonce_from {
    ($x:ty) => {
        impl From<$x> for Nonce {
            fn from(input: $x) -> Self {
                Self(U256::from(input))
            }
        }
    };
}

impl_nonce_from!(u32);
impl_nonce_from!(u64);
impl_nonce_from!(usize);
impl_nonce_from!(i32);
impl_nonce_from!(i64);
impl_nonce_from!(isize);

impl Nonce {
    pub fn zero() -> Self {
        Self(U256::zero())
    }
}

impl Add for Nonce {
    type Output = Nonce;

    fn add(self, rhs: Nonce) -> Self::Output {
        Nonce::from(self.0 + rhs.0)
    }
}

impl AddAssign for Nonce {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Nonce {
    type Output = Nonce;

    fn sub(self, rhs: Nonce) -> Self::Output {
        Nonce::from(self.0 - rhs.0)
    }
}

impl SubAssign for Nonce {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}
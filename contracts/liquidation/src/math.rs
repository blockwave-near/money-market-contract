use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::U128;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

const DECIMAL: u128 = 1_000_000_000_000;        //1e12

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct D128 {
    pub num: U128,
    decimal: u32,
}

impl Default for D128 {
    /// set default value to 1.0
    fn default() -> Self {
        Self{num: U128::from(DECIMAL), decimal: 12}
    }
}

impl D128 {
    /// num: multiplied by DECIMAL constant
    pub fn new(num: u128) -> Self{
        Self{
            num: num.into(),
            ..Default::default()
        }
    }

    /// get Decimal number 1.0
    pub fn one() -> Self {
        Self::default()
    }
}

impl Add<D128> for D128 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        let num: u128 = self.num.0 + other.num.0;
        
        Self::new(num)
    }
}

impl Add<u128> for D128 {
    type Output = Self;
    #[inline]
    fn add(self, other: u128) -> Self {
        let num: u128 = other * DECIMAL + self.num.0;
        
        Self::new(num)
    }
}

impl Sub<D128> for D128 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        let num: u128 = self.num.0 - other.num.0;

        Self::new(num)
    }
}

impl Sub<u128> for D128 {
    type Output = Self;
    #[inline]
    fn sub(self, other: u128) -> Self {
        let num: u128 = other * DECIMAL - self.num.0;
        
        Self::new(num)
    }
}

impl Mul<D128> for D128 {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        let num: u128 = ((U256::from(self.num.0) * U256::from(other.num.0)) / U256::from(DECIMAL)).as_u128();

        Self::new(num)
    }
}

impl Mul<u128> for D128 {
    type Output = u128;
    #[inline]
    fn mul(self, other: u128) -> u128 {
        (U256::from(other) * U256::from(self.num.0) 
            / U256::from(DECIMAL)).as_u128()
    }
}

impl Div<D128> for D128 {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        let num: u128 = (U256::from(self.num.0) * U256::from(DECIMAL) / U256::from(other.num.0)).as_u128();

        Self::new(num)
    }
}

impl Ord for D128 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.0.cmp(&other.num.0)
    }
}

impl PartialOrd for D128 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for D128 {}

impl PartialEq for D128 {
    fn eq(&self, other: &Self) -> bool {
        self.num.0 == other.num.0
    }
}
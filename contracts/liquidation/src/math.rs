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

pub const DECIMAL: u128 = 100_000_000;        //1e8

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct D128 {
    pub num: U128,
    decimal: u32,
}

impl Default for D128 {
    /// set default value to 1.0
    fn default() -> Self {
        Self{num: U128::from(DECIMAL), decimal: 8}
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

    /// get Decimal number 0.0
    pub fn zero() -> Self {
        Self::new(0)
    }

    /// get decimal value
    pub fn get_decimal() -> u32 {
        Self::default().decimal
    }

    /// Returns num * (10**exp)
    /// Ex) new_exp(1, -2) == D128::new(1e10 as u128) == (0.01)
    pub fn new_exp(num: u128, exp: i32) -> Self {
        assert!(exp >= -12);
        Self::new(num * u128::pow(10, (Self::get_decimal() as i32 + exp) as u32))
    }

    pub fn ratio(numer: u128, denom: u128) -> Self {
        D128::new(numer * DECIMAL) / D128::new(denom * DECIMAL)
    }

    pub fn mul_int(self, other: u128) -> u128 {
        ((U256::from(self.num.0) * U256::from(other)) / U256::from(DECIMAL)).as_u128()
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

impl Add<D128> for u128 {
    type Output = D128;
    #[inline]
    fn add(self, other: D128) -> D128 {
        let num: u128 = self * DECIMAL + other.num.0;

        D128::new(num)
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
        let num: u128 = self.num.0 - other * DECIMAL;
        
        Self::new(num)
    }
}

impl Sub<D128> for u128 {
    type Output = D128;
    #[inline]
    fn sub(self, other: D128) -> D128 {
        let num: u128 = self * DECIMAL - other.num.0;

        D128::new(num)
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
    type Output = Self;
    /// NOTE: u128 value should be big integer or there may be round error.
    #[inline]
    fn mul(self, other: u128) -> Self {
        let num: u128 = ((U256::from(self.num.0) * U256::from(other)) / U256::from(DECIMAL)).as_u128();

        Self::new(num * DECIMAL)
    }
}

impl Mul<D128> for u128 {
    type Output = D128;
    /// NOTE: u128 value should be big integer or there may be round error.
    #[inline]
    fn mul(self, other: D128) -> D128 {
        let num: u128 = ((U256::from(self) * U256::from(other.num.0)) / U256::from(DECIMAL)).as_u128();

        D128::new(num * DECIMAL)
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

impl Div<u128> for D128 {
    type Output = Self;
    #[inline]
    fn div(self, other: u128) -> Self {
        let other: D128 = Self::new(other * DECIMAL);
        let num: u128 = (U256::from(self.num.0) * U256::from(DECIMAL) / U256::from(other.num.0)).as_u128();

        Self::new(num)
    }
}

impl Div<D128> for u128 {
    type Output = D128;
    #[inline]
    fn div(self, other: D128) -> D128 {
        let self_value: D128 = D128::new(self * DECIMAL);
        let num: u128 = (U256::from(self_value.num.0) * U256::from(DECIMAL) / U256::from(other.num.0)).as_u128();

        D128::new(num)
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
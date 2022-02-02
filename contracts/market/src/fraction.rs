use crate::*;
use std::cmp;
use std::ops::{Add, Div, Mul, Sub};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Fraction {
  numer: u128,
  denom: u128,
}

impl Fraction {
  pub fn new_raw(numer: u128, denom: u128) -> Fraction {
    Fraction { numer, denom }
  }

  pub fn new(numer: u128, denom: u128) -> Fraction {
    let mut ret = Fraction::new_raw(numer, denom);
    ret.reduce();
    ret
  }

  fn reduce(&mut self) {
    if self.denom == 0 {
      panic!("denominator == 0");
    }
    if self.numer == 0 {
      self.denom = 1;
      return;
    }
    if self.numer == self.denom {
      self.numer = 1;
      self.denom = 1;
      return;
    }
    let g: u128 = gcd(self.numer, self.denom);

    self.numer = self.numer.clone() / g.clone();
    self.denom = self.denom.clone() / g;

    // // keep denom positive!
    // if self.denom < 0u128 {
    //   self.numer = 0u128 - self.numer.clone();
    //   self.denom = 0u128 - self.denom.clone();
    // }
  }

  pub fn zero() -> Fraction {
    Fraction::new(0, 1)
  }

  pub fn one() -> Fraction {
    Fraction::new(1, 1)
  }
}

impl Mul<Fraction> for Fraction {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: Self) -> Self {
    let gcd_ad = gcd(self.numer, rhs.denom);
    let gcd_bc = gcd(self.denom, rhs.numer);
    Fraction::new(
      self.numer / gcd_ad.clone() * (rhs.numer / gcd_bc.clone()),
      self.denom / gcd_bc * (rhs.denom / gcd_ad),
    )
  }
}

impl Mul<u128> for Fraction {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: u128) -> Fraction {
    let gcd = gcd(self.denom, rhs);
    Fraction::new(self.numer * (rhs / gcd.clone()), self.denom / gcd)
  }
}

impl Div<Fraction> for Fraction {
  type Output = Self;
  #[inline]
  fn div(self, rhs: Self) -> Self {
    let gcd_ad = gcd(self.numer, rhs.numer);
    let gcd_bc = gcd(self.denom, rhs.denom);
    Fraction::new(
      self.numer / gcd_ad.clone() * (rhs.denom / gcd_bc.clone()),
      self.denom / gcd_bc * (rhs.numer / gcd_ad),
    )
  }
}

impl Div<u128> for Fraction {
  type Output = Self;
  #[inline]
  fn div(self, rhs: u128) -> Fraction {
    let gcd = gcd(self.numer, rhs);
    Fraction::new(self.numer / gcd.clone(), self.denom * (rhs / gcd))
  }
}

impl Add<Fraction> for Fraction {
  type Output = Self;
  #[inline]
  fn add(self, other: Fraction) -> Self {
    if self.denom == other.denom {
      Fraction::new(self.numer + other.numer, self.denom)
    } else {
      let lcm = lcm(self.denom, other.denom);
      let lhs_numer = self.numer.clone() * (lcm.clone() / self.denom.clone());
      let rhs_numer = other.numer * (lcm.clone() / other.denom);
      Fraction::new(lhs_numer + rhs_numer, lcm)
    }
  }
}

impl Add<u128> for Fraction {
  type Output = Self;
  #[inline]
  fn add(self, other: u128) -> Self {
    let numer = self.numer + other * self.denom.clone();
    Fraction::new(numer, self.denom)
  }
}

impl Sub<Fraction> for Fraction {
  type Output = Self;
  #[inline]
  fn sub(self, other: Fraction) -> Self {
    if self.denom == other.denom {
      Fraction::new(self.numer + other.numer, self.denom)
    } else {
      let lcm = lcm(self.denom, other.denom);
      let lhs_numer = self.numer.clone() * (lcm.clone() / self.denom.clone());
      let rhs_numer = other.numer * (lcm.clone() / other.denom);
      Fraction::new(lhs_numer - rhs_numer, lcm)
    }
  }
}

impl Sub<u128> for Fraction {
  type Output = Self;
  #[inline]
  fn sub(self, other: u128) -> Self {
    let numer = self.numer - other * self.denom.clone();
    Fraction::new(numer, self.denom)
  }
}

impl PartialOrd for Fraction {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Fraction {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == cmp::Ordering::Equal
  }
}

impl Eq for Fraction {}

impl Ord for Fraction {
  #[inline]
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    if self.denom == other.denom {
      let ord = self.numer.cmp(&other.numer);
      return ord;
    }

    if self.numer == other.numer {
      if self.numer == 0 {
        return cmp::Ordering::Equal;
      }
      let ord = self.denom.cmp(&other.denom);
      return ord.reverse();
    }

    let (self_int, self_rem) = (self.numer / self.denom, self.numer % self.denom);
    let (other_int, other_rem) = (other.numer / other.denom, other.numer % other.denom);
    match self_int.cmp(&other_int) {
      cmp::Ordering::Greater => cmp::Ordering::Greater,
      cmp::Ordering::Less => cmp::Ordering::Less,
      cmp::Ordering::Equal => match (self_rem == 0, other_rem == 0) {
        (true, true) => cmp::Ordering::Equal,
        (true, false) => cmp::Ordering::Less,
        (false, true) => cmp::Ordering::Greater,
        (false, false) => {
          let self_recip = Fraction::new_raw(self.denom.clone(), self_rem);
          let other_recip = Fraction::new_raw(other.denom.clone(), other_rem);
          self_recip.cmp(&other_recip).reverse()
        }
      },
    }
  }
}

fn lcm(x: u128, y: u128) -> u128 {
  x * y / gcd(x, y)
}

fn gcd(x: u128, y: u128) -> u128 {
  let mut x = x;
  let mut y = y;
  while y != 0 {
    let t = y;
    y = x % y;
    x = t;
  }
  x
}

#[test]
fn test_new_reduce() {
  assert_eq!(Fraction::new(2, 2), Fraction::one());
  assert_eq!(Fraction::new(0, u128::MAX), Fraction::zero());
  assert_eq!(Fraction::new(u128::MAX, u128::MAX), Fraction::one());
}

#[test]
#[should_panic]
fn test_new_zero() {
  let _a = Fraction::new(1, 0);
}

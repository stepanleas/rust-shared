use bigdecimal::BigDecimal;
use num_traits::FromPrimitive;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use uuid::Uuid;

pub struct OrderId(Uuid);
pub struct CustomerId(Uuid);
pub struct ProductId(Uuid);
pub struct CategoryId(Uuid);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Money {
    amount: BigDecimal,
}

impl Money {
    pub fn new(amount: BigDecimal) -> Self {
        Self { amount }
    }

    pub fn from_parts(units: i64, cents: i64) -> Self {
        let total_cents = units * 100 + cents;
        let value = BigDecimal::from(total_cents).with_scale(2);
        Self { amount: value }
    }

    pub fn from_f64(value: f64) -> Result<Self, anyhow::Error> {
        let amount =
            BigDecimal::from_f64(value).ok_or_else(|| anyhow::Error::msg("invalid money value"))?;

        Ok(Money { amount })
    }

    pub fn zero() -> Self {
        Money { amount: BigDecimal::from(0) }
    }

    pub fn is_greater_than_zero(&self) -> bool {
        self.amount >= BigDecimal::from(0)
    }

    pub fn value(self) -> BigDecimal {
        self.amount
    }
}

impl Add for Money {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Money::new(self.amount + other.amount)
    }
}

impl Sub for Money {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Money::new(self.amount - other.amount)
    }
}

impl Mul<i32> for Money {
    type Output = Self;
    fn mul(self, factor: i32) -> Self {
        Money::new(self.amount * BigDecimal::from(factor))
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.amount)
    }
}

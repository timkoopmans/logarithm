use crate::decimal::Decimal;

pub trait Div<T>: Sized {
    fn div(self, rhs: T) -> Self;
}

/// Divide a [Decimal] over another [Decimal], including signed division.
impl Div<Decimal> for Decimal {
    fn div(self, rhs: Decimal) -> Self {
        Self {
            value: self
                .value
                .checked_mul(rhs.denominator())
                .unwrap_or_else(|| panic!("decimal: overflow in method Decimal::div().checked_mul"))
                .checked_div(rhs.value)
                .unwrap_or_else(|| {
                    panic!("decimal: overflow in method Decimal::div().checked_div")
                }),
            scale: self.scale,
            negative: self.negative != rhs.negative,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::decimal::ops::Div;
    use crate::decimal::Decimal;

    #[test]
    fn test_div() {
        {
            // test: 0.0000002 / 0.002 = 0.00010000
            let a = Decimal::new(2, 8, false);
            let b = Decimal::new(2, 3, false);
            let actual = a.div(b);
            let expected = Decimal::new(1000, 8, false);

            assert_eq!(actual, expected);
        }

        {
            // test: 0.0000002 / 0.003 = 0.00006666
            let a = Decimal::new(2, 8, false);
            let b = Decimal::new(3, 3, false);
            let actual = a.div(b);
            let expected = Decimal::new(666, 8, false);

            assert_eq!(actual, expected);
        }

        {
            // test: -12 / -3 = 4
            let a = Decimal::new(12, 0, true);
            let b = Decimal::new(3, 0, true);
            let expected = Decimal::new(4, 0, false);
            assert_eq!(a.div(b), expected);
        }

        {
            // test: -12 / 3 = -4
            let a = Decimal::new(12, 0, true);
            let b = Decimal::new(3, 0, false);
            let expected = Decimal::new(4, 0, true);
            assert_eq!(a.div(b), expected);
        }

        {
            // test: 12 / -3 = -4
            let a = Decimal::new(12, 0, false);
            let b = Decimal::new(3, 0, true);
            let expected = Decimal::new(4, 0, true);
            assert_eq!(a.div(b), expected);
        }

        {
            // test: 12 / 3 = 4
            let a = Decimal::new(12, 0, false);
            let b = Decimal::new(3, 0, false);
            let expected = Decimal::new(4, 0, false);
            assert_eq!(a.div(b), expected);
        }
    }

    #[test]
    #[should_panic(expected = "decimal: overflow in method Decimal::div().checked_div")]
    fn test_div_panic() {
        let a = Decimal::new(10, 3, false);
        let b = Decimal::new(0, 1, false);
        a.div(b);
    }
}

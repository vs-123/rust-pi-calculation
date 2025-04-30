use bigdecimal::BigDecimal;
use std::str::FromStr;

fn multiply_consecutive(n: BigDecimal) -> BigDecimal {
    (n.clone() + BigDecimal::from_str("1.0").unwrap())
        * (n.clone() + BigDecimal::from_str("2.0").unwrap())
        * (n.clone() + BigDecimal::from_str("3.0").unwrap())
}

// We're gonna calculate Pi using the Nilakantha series.
// According to the Nilakantha series
// π = 3 + 4/(2*3*4) - 4/(4*5*6) + 4/(6*7*8) - 4/(8*9*10) + 4/(10*11*12) - 4/(12*13*14)...

fn pi(x: i64) -> BigDecimal {
    let big_two = BigDecimal::from_str("2.0").unwrap();
    let big_four = BigDecimal::from_str("4.0").unwrap();

    let mut output = BigDecimal::from_str("3.0").unwrap();
    let mut n = BigDecimal::from_str("1.0").unwrap();
    let mut plus = true;

    for _ in 0..x {
        if plus {
            output =
                output + (big_four.clone() / multiply_consecutive(n.clone()));
        } else {
            output =
                output - (big_four.clone() / multiply_consecutive(n.clone()));
        }

        plus = !plus;
        n += big_two.clone();
    }

    output
}

fn main() {
    let iterations: i64 = 1000;

    println!("{}", pi(iterations));
}

///Calculate the fifth root of the first 100
///Odd numbers.
fn main() {

    let sum = sum() as f64;
    println!("{}", sum.powf(0.2));
}

fn sum() -> u32 {
    let mut sum = 0;
    for i in (1..200).step_by(2) {
        sum += i;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn fifth_root() {
        assert_eq!(2.0, 32f64.powf(0.2));
    }
}

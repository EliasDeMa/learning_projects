use std::io;
use std::io::Write;

struct Coin {
    value: f64,
    amount: u32,
}

impl Coin {
    fn worth(&self) -> f64 {
        self.value * self.amount as f64
    }
}

fn main() {
    let mut coins = vec![];

    println!("Enter amount of each coin in your piggybank");

    get_worth_input(&mut coins, 2.0);
    get_worth_input(&mut coins, 1.0);
    get_worth_input(&mut coins, 0.5);
    get_worth_input(&mut coins, 0.2);
    get_worth_input(&mut coins, 0.1);
    get_worth_input(&mut coins, 0.05);

    println!("The total amount in your piggybank is: €{:.2}", total_worth(coins));
}

// Enter function for each coin
// returns a u32 to fill into the coin struct
fn enter() -> u32 {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read line");

    buf.trim().parse().expect("Failed to parse number")
}

// Get total worth of coins in the vector
fn total_worth(coins: Vec<Coin>) -> f64 {
    let mut total: f64 = 0.0;

    for coin in coins {
        total += coin.worth();
    }
    total
}

// Format the output since there is a lot of repetition
// push coin struct into a vector
fn get_worth_input(coins: &mut Vec<Coin>, cent: f64) {
    print!("€{:.2}: ", cent);
    io::stdout()
        .flush()
        .expect("Unable to flush stdout");
    coins.push(Coin {
        value: cent,
        amount: enter(),
    });
}

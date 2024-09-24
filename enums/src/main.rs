#[derive(Debug)]
enum UsState {
    Alabama,
    Arkansas,
}

enum Coin {
    Quarter(UsState),
    _Penny,
    Nickel,
    _Dime,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        Coin::_Penny => {
            println!("Lucky Penny!");
            1
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn count_coins(coins: &[Coin]) -> u32 {
    let mut count = 0;
    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State: {state:?}");
        }
        count += 1;
    }
    count
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Quarter(UsState::Alabama);
    let coin3 = Coin::Quarter(UsState::Arkansas);
    let coin4 = Coin::Quarter(UsState::Arkansas);
    let coin5 = Coin::Nickel;

    let coins = [coin1, coin2, coin3, coin4, coin5];

    for coin in &coins {
        println!("Value: {}", value_in_cents(coin));
    }

    let maybe_int1: Option<i32> = Some(23);
    let maybe_int2: Option<i32> = None;

    let sum1 = plus_one(maybe_int1);
    let sum2 = plus_one(maybe_int2);

    match sum1 {
        Some(i) => println!("Sum1 is {}", i),
        None => println!("Sum1 is None!"),
    }

    match sum2 {
        Some(i) => println!("Sum2 is {}", i),
        None => println!("Sum2 is None!"),
    }

    let count = count_coins(&coins);
    println!("{count} total coins!");
}

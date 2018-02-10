fn main() {
    let x = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("quarter: {}!", x);
}

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    //Dollar //uncomment will require update match coin below
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>  {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

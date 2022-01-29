#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Fake(u8),
}

fn interprete_coin (coin: &Coin) -> &u8 {
    match coin{
        Coin::Penny => &1,
        Coin::Nickel => &5,
        Coin::Dime => &10,
        Coin::Quarter => &25,
        Coin::Fake(c) => c,
    }
}

#[derive(Debug)]
enum message {

}




pub fn run() {
    let unknown = Coin::Fake(20);
    println!("It has a value of: {:?}", interprete_coin(&unknown));
    println!("The coin is a {:?}", unknown);
}

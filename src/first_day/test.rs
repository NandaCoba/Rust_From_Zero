
pub fn user(text:&str) {
    println!("my name is {}", text)
}

pub fn any_user() {
    let mut storage = ["nanda","coba","dh"];

    let first = storage.first();

    println!("{:?}",storage);

    println!("{:?}",first);
}
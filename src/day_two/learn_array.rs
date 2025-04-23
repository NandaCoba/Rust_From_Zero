

pub fn array() {
    let mut storage = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];

    for items in storage {
        if items % 5 == 0 && items % 3 == 0 {
            println!("FizzBuzz");
        }
        else if items % 3 == 0 {
            println!("Fizz");
        }
        else if items % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}",items)
        }
    }
}


// use std::io;
// use std::cmp::Ordering;



// fn main() {
//     println!("Hello, world!");
//
//     let secret_number = rand::random_range(1..=100);
//     'test_loop: loop {
//
//         println!("Secret number : {secret_number}" );
//         println!("Enter a number ");
//
//         let charr : char = 'Z';
//         println!("char : {charr}" );
//
//         let mut guess: String   = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect(" Failed to read");
//
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Lesser !"),
//             Ordering::Greater => println!("Greater !"),
//             Ordering::Equal => {
//                 println!("WHOOOOOOHOOOO");
//                 break 'test_loop;
//             },
//         }
//         println!("Number Guessed: {guess}" );
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // let string = "hello world"; // string hardcode cant be mutable
    // let mut string_2 = "hello world";
    let mut string_3: String = String::from("Hello");
    string_3.push_str(", world !");
    drop(string_3);

}

use std::env::args;

fn main(){
    for(i,arg) in args().enumerate(){
        println!("Arg {i}: {arg}");

        if i == 1 {
            parse(arg);
        }
    }
}

fn parse(given_string: String) {

    let parsed_string: [&str; 8] = given_string
    .split(":")
    .collect::<Vec<&str>>()
    .try_into()
    .unwrap_or_default();

    let mut count = 1;
    for string in parsed_string {
        println!("String {count}: {string}");
        count = count + 1;
    }
}
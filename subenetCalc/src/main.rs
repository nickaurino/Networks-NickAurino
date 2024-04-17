use std::env::args;

fn main(){
    // let subnet_input: Vec<String> = args().collect();

    // for (i, arg) in subnet_input.iter().enumerate(){

    for(i,arg) in args().enumerate(){
        println!("Arg {i}: {arg}");

        if i == 1 {
            calculate_subnet(arg);
        }
    }
}

fn calculate_subnet(target_subnet: String) {
    //iterate over a split string
    // let parts =  target_subnet.split("/");
    // for part in parts{
    //     println!("Subnet part: {part}")
    // }

    //split strings into named variables
    let [target_ip, cidr]: [&str; 2] = target_subnet
        .split("/")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    println!("Target IP: {target_ip}");
    println!("CIDR: {cidr}");

    let [o1, o2, o3, o4]: [&str; 4] = target_ip
        .split(".")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    println!("1st Octet: {o1}");
    println!("2nd Octet: {o2}");
    println!("3rd Octet: {o3}");
    println!("4th Octet: {o4}");

    let octet_strings: [&str; 4] = target_ip
        .split(".")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    for octet in octet_strings {
        let curr_octect: i32 = octet.parse().unwrap();
        // let curr_octect _ octet.parse::<int32>.unwrap()
        let octet_math = curr_octect + 1;
        println!("{octet_math}")
    }

}

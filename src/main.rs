
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use sha2::{Sha256, Digest};


fn main() {
    println!("Hello, PassCracker!");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid arguments provided");
        println!("Example: cargo run <hash>");
        exit(1);
    }

    let req_hash: &String = &args[1];
    let password_src = "src/passwordtext.txt";
    let mut attempts = 1;

    println!("Attempting to crack: {}!\n", req_hash );
    let password_list = File::open(password_src).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let hashed_passwords = format!("{:x}", Sha256::digest(&password));

        println!("[{}] {} == {}" , attempts, std::str::from_utf8(&password).unwrap(), hashed_passwords);

        if &hashed_passwords == req_hash {
            println!("Password hash found after {} attempts {} hashes to {}", attempts, std::str::from_utf8(&password).unwrap(), hashed_passwords);
            exit(0);
        }

        attempts = attempts + 1;
    }

    println!("Password hash not found");

}


/* Hashes of some common passwords to test
 Hash                                                                 ---> Pass/Fail to crack
1] 5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8    - Pass 
2] 5c8f52ca28966103ff0aad98160bc8e978c9ca0285a2043a521481d11ed17506    - Pass
3] 3b9b05c720497601bfacaf66e58e8e37e3545ea9ab671665ad2e72e483c4872f    - Pass
4] d9e218ac870d788c2b761701c15ad2bef9c4849926963abeba20011d2538930e    - Pass
5] b3ea8743fe89bca75d268af3ecbdc8f9c0ae5a9e2242448a4f222f4ff77e3ec2    - Fail
Conclusion :- Please use a strong password with various characters involved and of the highest length possible
Ex:- Sahbaaz@123.xyz
*/
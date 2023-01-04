use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: <hash> <password> <cost>");
        eprintln!("usage: <verify> <password> <hash>");
        return;
    }
    
    if let Some(process) = args.get(1) {
        if process == "hash" {
            let password = &args[2];
            // cost can be optional and default to DEFAULT_COST
            let cost = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_COST);
            match hash(password, cost) {
                Ok(hashed) => println!("{}", hashed),
                Err(e) => println!("Error hashing password: {}", e),
            }
            return;
        }
        
        else if process == "verify" {
            let password = &args[2];
            let hash = &args[3].trim();
            match verify(password, hash) {
                Ok(matches) => println!("{}", matches),
                Err(e) => println!("Error verifying password: {}", e),
            }
            return;
        }
        
        else {
            eprintln!("usage: <hash> <password>");
            eprintln!("usage: <verify> <password> <hash>");
            return;
        }
    }
}
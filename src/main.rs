use clap::Parser;
use std::collections::BTreeSet;

#[derive(Parser)]
struct Cli {
    action: String,
    domain: String,
}

fn main() {
    let args = Cli::parse();
    let mut blocked = BTreeSet::new();

    if args.action == "block" {
        blocked.insert(args.domain.to_string());
    }
    else if args.action == "unblock" {
        blocked.remove(&args.domain);
    }
    else if args.action == "activate" {   
        let mut file_content: String = std::fs::read_to_string("/etc/hosts").expect("failed to read /etc/hosts");
        for item in blocked {
            file_content.push_str("0.0.0.0 {item}\n::1 {item}\n");
        }
    }
    else if args.action == "deactivate" {
        let mut file_content: String = std::fs::read_to_string("/etc/hosts").expect("failed to read /etc/hosts");
        for item in blocked {
            file_content.replace("0.0.0.0 {item}\n::1 {item}\n", "");
        }
    }
    else {
        println!("Invalid command.");
    }
}

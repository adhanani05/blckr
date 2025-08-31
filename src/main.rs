use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = include_str!("../README.md"))]
struct Cli {
    action: String,
    domain: String,
}

fn main() {
    let args = Cli::parse();
    let path = "/etc/hosts";

    if args.action == "block" {
        let mut file_content = std::fs::read_to_string(path).expect("failed to read /etc/hosts");
        if !file_content.contains(&args.domain) {
            file_content.push_str(&format!("0.0.0.0 {}\n::1 {}\n", args.domain, args.domain));
            std::fs::write(path, file_content).expect("failed to write to file");
        }
    } else if args.action == "unblock" {
        let file_content = std::fs::read_to_string(path).expect("failed to read /etc/hosts");
        if file_content.contains(&args.domain) {
            let modified = file_content
                .replace(&format!("0.0.0.0 {}\n", args.domain), "")
                .replace(&format!("::1 {}\n", args.domain), "");  

            std::fs::write(path, modified).expect("failed to write to file");
        }
    } else {
        println!("Invalid command.");
    }
}

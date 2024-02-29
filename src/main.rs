use clap::Parser;
#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct  Arg{
    pub cmd: String,
}

fn main() {
    let args = Arg::parse();
    match args.cmd.as_str() {
        "init" => {
            let dir = std::fs::read_dir("bact");
            if dir.is_ok() {
                println!("❌ already initialized");
            } else {
                std::fs::create_dir("bact").unwrap();
                std::fs::write("bact/test.toml", "[main]
url=\"localhost:8080\"

[args]

[header]");
                println!("✅ initialized");
            }
        }
        "test" => {
            let content = std::fs::read_to_string("bact/test.toml").unwrap();
            let a: toml::Value = toml::from_str(&content).unwrap();
            println!("{:?}", a["main"])
        }
        _ => {}
    }
}
use clap::Parser;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub cmd: String,
}

fn main() {
    env_logger::init();
    let args = Arg::parse();
    match args.cmd.as_str() {
        "init" => {
            let dir = std::fs::read_dir("bact");
            if dir.is_ok() {
                log::error!("already initialized");
            } else {
                std::fs::create_dir("bact").unwrap();
                std::fs::write(
                    "bact/test.toml",
                    "[main]\nurl=\"localhost:8080\"\nmethod=\"GET\"\n\n[args]\n\n[header]",
                )
                .unwrap();
                log::info!("initialized");
            }
        }
        "test" => {
            let content = std::fs::read_to_string("bact/test.toml").unwrap();
            let a: toml::Value = toml::from_str(&content).unwrap();
            let url = &a["main"]["url"].to_string();
            let method = &a["main"]["method"].to_string();
            match method.as_str() {
                "GET" => {
                    log::info!("url {}", url);
                }
                "POST" => {
                    log::info!("url {}", url);
                }
                "DELETE" => {
                    log::info!("url {}", url);
                }
                "FETCH" => {
                    log::info!("url {}", url);
                }
                method => {
                    log::error!("{} is unknown method", method);
                }
            }
        }
        _ => {}
    }
}

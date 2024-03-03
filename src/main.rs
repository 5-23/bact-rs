use clap::Parser;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub cmd: String,
}
#[tokio::main]
async fn main() {
    let mut log_builder = env_logger::builder();
    log_builder.filter_level(log::LevelFilter::Trace);
    log_builder.init();
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
            for entry in std::fs::read_dir("bact").unwrap() {
                if entry.is_ok() {
                    let path = entry
                        .unwrap()
                        .path()
                        .to_str()
                        .unwrap()
                        .replace("bact", "")
                        .replace("\\", "")
                        .replace(".toml", "")
                        .to_owned();
                    let content = std::fs::read_to_string(format!("bact/{}.toml", &path)).unwrap();
                    let a: toml::Value = toml::from_str(&content).unwrap();

                    let url = &a["main"]["url"].to_string().replace("\"", "");
                    let method = &a["main"]["method"].to_string().replace("\"", "");
                    // log::info!(target: format!("{}::url", &path).as_str(), "{url}");
                    // log::info!(target: format!("{}::method", &path).as_str(), "{method}");

                    match method.to_uppercase().as_str() {
                        "GET" => {
                            let client = reqwest::Client::new();
                            let req = client.get(url).send().await.unwrap();
                            log::debug!(target: format!("{}", &path).as_str(), "{:#?}", req.headers())
                        }
                        "POST" => {
                            let client = reqwest::Client::new();
                            let req = client.post(url).send().await.unwrap();
                            log::debug!(target: format!("{}", &path).as_str(), "{:#?}", req.headers())
                        }
                        "DELETE" => {
                            let client = reqwest::Client::new();
                            let req = client.delete(url).send().await.unwrap();
                            log::debug!(target: format!("{}", &path).as_str(), "{:#?}", req.headers())
                        }
                        "PATCH" => {
                            let client = reqwest::Client::new();
                            let req = client.patch(url).send().await.unwrap();
                            log::debug!(target: format!("{}", &path).as_str(), "{:#?}", req.headers())
                        }
                        method => {
                            log::error!(target: format!("{}", &path).as_str(), "{:?} is unknown method", method);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

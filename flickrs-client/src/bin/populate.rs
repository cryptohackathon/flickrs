use std::path::PathBuf;

use structopt::StructOpt;

const SERVER: &'static str = "https://flickrs.opencloudedge.be";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "populate",
    about = "Populate the database with attributes and pictures"
)]
enum Opts {
    Upload {
        #[structopt(parse(from_os_str))]
        filename: PathBuf,
    },
    ListAttributes,
    AddAttribute {
        name: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::from_args();
    let client = reqwest::blocking::Client::new();
    match opts {
        Opts::ListAttributes => {
            let url = format!("{}/api/{}", SERVER, "attributes");
            let res = client.get(&url).send()?.text()?;
            println!("{}", res);
        }
        Opts::AddAttribute { name } => {
            let url = format!("{}/api/{}", SERVER, "attributes/new");
            let res = client.post(&url).body(name).send()?.text()?;
            println!("{}", res);
        }
        Opts::Upload { path } => {
            let url = format!("{}/api/{}", SERVER, "upload");
            let res = client.post(&url).body(name).send()?.text()?;
            println!("{}", res);
        }
        _ => {}
    }

    Ok(())
}

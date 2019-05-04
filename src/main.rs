//use structopt::StructOpt;
use github_rs::client::{Executor, Github};
use serde_json::Value;

/// Search for a pattern in a file and display the lines that contain it.
//#[derive(StructOpt)]
//struct Cli {
//    /// The pattern to look for
//    pattern: String,
//    /// The path to the file to read
//    #[structopt(parse(from_os_str))]
//    path: std::path::PathBuf,
//}

fn main() {
    // TODO(Jonathon): Will uncomment when I actually need cmd line args
//    let args = Cli::from_args();

//    let content = std::fs::read_to_string(&args.path)
//        .expect("could not read file");
//
//    for line in content.lines() {
//        if line.contains(&args.pattern) {
//            println!("{}", line);
//        }
//    }

    let github_key = "GITHUB_ACCESS_TOKEN";
    let github_access_token = match std::env::var(github_key) {
        Ok(val) => val,
        Err(e) => panic!("Must include Github token in environment var {}: {}", github_key, e),
    };

    let gh_client = Github::new(github_access_token).unwrap();
    let me = gh_client.get()
        .user()
        .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}

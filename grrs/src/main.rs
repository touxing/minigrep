use exitfailure::ExitFailure;
use failure::ResultExt;
use grrs::find_matches;
use indicatif::ProgressBar;
use log::{info, warn};
use std::fs::read_to_string;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

/// Search for a patern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The path to the look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}



fn main() -> Result<(), ExitFailure> {
    let pb = ProgressBar::new(10);
    for _ in 0..10 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    pb.finish_with_message("done");

    let args = Cli::from_args();

    let content = read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    // println!("file content: {}", &content);
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}



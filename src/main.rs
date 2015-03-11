#![feature(path)]
#![feature(path_ext)]

use std::env;

use std::path::Path;
use std::fs::PathExt;

mod asverify;

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string())
                                       .collect();

    match &args[..] {
        [_, ref cmd, ref filename1, ref filename2] => {
            let current_path = Path::new(".");
            let path1        = current_path.join(filename1);
            let path2        = current_path.join(filename2);

            if (!(path1.exists() && path2.exists())){
                println!("Please provide the path of two existing files.");
                return;
            }

            println!("Verifying {:?} and {:?}", path1.display(), path2.display());
            asverify::verify(path1, path2);
        },

        [_, ref cmd] => {
            match &cmd[..] {
                _ => {
                    println!("error: invalid command");
                    help();
                },
            }
        },

        _ => {
            // show a help message
            help();
        }
    }
}

fn help() {
    println!("Usage:
Available commands:
- verify: verifies that an App Store IPA matches the one uploaded to iTunes Connect.
");
}

#[cfg(test)]
#[macro_use]
mod test_helpers;

mod list_all_files;
mod types;
mod witd;

use std::env;
use witd::{WError, Witd};

fn main() -> Result<(), WError> {
    let args: Vec<String> = env::args().collect();

    let mut witd = Witd::new();
    for arg in args.iter().skip(1) {
        witd.add_path(arg)?;
    }

    while !witd.should_exit() {
        for path in witd.paths() {
            let files = list_all_files::execute(path);
            println!("files: {:#?}", files);
        }

        witd.quit();
    }

    println!("args: {:?}", args);
    Ok(())
}

extern crate clap;
extern crate ttaw;
use clap::App;
use std::path::Path;

#[cfg_attr(tarpaulin, skip)]
pub fn main() {
    let matches = App::new("ttaw")
        .version(clap::crate_version!())
        .author("Shea Newton <shnewto@gmail.com>")
        .about("An CLI for ttaw, a piecemeal natural language processing library")
        .args_from_usage("<PATH>    'path to download new serialized CMU dictionary'")
        .get_matches();

    let path = matches.value_of("PATH").unwrap();

    ttaw::cmu::download_and_serialze(Path::new(path))
        .expect("Something went wrong with the <PATH> parameter!");
}

use super::{dmap, download};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use spinners::{Spinner, Spinners};
use std::fs::File;

pub fn install(name: &str, version: &str) {
    let mut spinner = Spinner::new(Spinners::Line, "Mapping all dependencies...".to_string());
    let deps = dmap::get_all_deps(&name, &version).unwrap();
    spinner.stop_with_message(format!(
        "Mapped {} dependencies, downloading...",
        style(deps.len()).cyan().bold()
    ));

    // Download all deps
    let pb = ProgressBar::new(deps.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );

    deps.par_iter().for_each(|dep| {
        // Read dep name, version and tarball url from dep string
        // name@version@tarball
        let dep_split: Vec<&str> = dep.split('@').collect();
        let mut name = dep_split[0];
        let mut _version = dep_split[1];
        let mut tarball = dep_split[2];

        // Is a @org package
        if dep.starts_with("@") {
            name = dep_split[1];
            _version = dep_split[2];
            tarball = dep_split[3];
        }

        download::from_url(name, tarball).unwrap();

        // Extract tarball & rename
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
            File::open(format!("./node_modules/.temp/{}.tar.gz", name)).unwrap(),
        ));

        // TODO: Remove this control and move it to the dmap function
        match archive.unpack(format!("./node_modules/.temp/{}", name)) {
            Ok(_) => (),
            Err(_) => (),
        }

        match std::fs::rename(
            format!("./node_modules/.temp/{}/package", name),
            format!("./node_modules/{}", name),
        ) {
            Ok(_) => (),
            Err(_) => (),
        }

        // Delete tarball
        match std::fs::remove_file(format!("./node_modules/.temp/{}.tar.gz", name)) {
            Ok(_) => (),
            Err(_) => (),
        }
        match std::fs::remove_dir(format!("./node_modules/.temp/{}", name)) {
            Ok(_) => (),
            Err(_) => (),
        }

        pb.set_position(pb.position() + 1);
    });

    // Clean up
    std::fs::remove_dir_all("./node_modules/.temp").unwrap();
    pb.finish_and_clear();

    println!(
        "Installed {} dependencies successfully!",
        style(deps.len()).cyan().bold()
    );
    println!(
        "Installed {}@{} successfully!",
        style(name).green().bold(),
        style(version).cyan().bold()
    );
}

use std::{fs::File, sync::{Arc, Mutex}};

use super::{dmap, download};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

pub fn install(name: &str, version: &str) {
    println!("Installing {}@{}...", name, version);
    let deps = dmap::get_all_deps(&name, &version).unwrap();

    // Download all deps

    let pb = ProgressBar::new(deps.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap().progress_chars("##-"),
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
        archive
            .unpack(format!("./node_modules/.temp/{}", name))
            .unwrap();
        std::fs::rename(
            format!("./node_modules/.temp/{}/package", name),
            format!("./node_modules/{}", name),
        )
        .unwrap();

        // Delete tarball
        std::fs::remove_file(format!("./node_modules/.temp/{}.tar.gz", name)).unwrap();
        std::fs::remove_dir_all(format!("./node_modules/.temp/{}", name)).unwrap();

        pb.set_position(pb.position() + 1);
    });

    // Clean up
    std::fs::remove_dir_all("./node_modules/.temp").unwrap();

    pb.finish_with_message("Done");
}

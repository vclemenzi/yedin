use std::{fs::File, sync::{Arc, Mutex}};

use super::{dmap, download};
use rayon::prelude::*;
use spinners::{Spinner, Spinners};

pub fn install(name: &str, version: &str) {
    let mut sp = Spinner::new(
        Spinners::Point,
        format!("Mapping dependencies for {}@{}", name, version).into(),
    );
    let deps = dmap::get_all_deps(&name, &version).unwrap();
    sp.stop_with_message(format!("Mapped dependencies for {}@{}", name, version).to_string());

    // Download all deps
    let mut spd = Spinner::new(
        Spinners::Point,
        format!("Downloading dependencies for {}@{}", name, version).into(),
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
    });

    spd.stop_with_message(format!("Downloaded dependencies for {}@{}", name, version).to_string());

    // Clean up

    let mut spc = Spinner::new(
        Spinners::Point,
        format!("Cleaning up for {}@{}", name, version).into(),
    );

    std::fs::remove_dir_all("./node_modules/.temp").unwrap();

    spc.stop_with_message(format!("Cleaned up for {}@{}", name, version).to_string());
}

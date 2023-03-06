use super::{dmap, download};
use crate::package::version::clean_version;
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
        let mut version = dep_split[1];
        let mut tarball = dep_split[2];

        // Is a @org package
        if dep.starts_with("@") {
            name = dep_split[1];
            version = dep_split[2];
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

        // Update yedin.lock
        let mut content = std::fs::read_to_string("./yedin.lock").unwrap();
        content.push_str(&format!("{}@{}@{}\n", name, version, tarball));
        std::fs::write("./yedin.lock", content).unwrap();

        // Update progress bar
        pb.set_position(pb.position() + 1);
    });

    // Add the dep to the package.json
    let package_json = std::fs::read_to_string("./package.json").unwrap();
    let mut package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();

    if package_json["dependencies"].is_null() {
        package_json["dependencies"] = serde_json::Value::Object(serde_json::Map::new());
    }

    let deps_json = package_json["dependencies"].as_object_mut().unwrap();

    deps_json.insert(
        name.to_string(),
        serde_json::Value::String(clean_version(name, version)),
    );

    std::fs::write(
        "./package.json",
        serde_json::to_string_pretty(&package_json).unwrap(),
    ).unwrap();

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

use std::{sync::{Arc, Mutex}, collections::HashSet};
use rayon::prelude::*;

use super::version::clean_version;

pub fn dmap(name: &str, version: &str, deps: Arc<Mutex<HashSet<String>>>) -> Result<(), reqwest::Error> {
    let url = format!("https://registry.npmjs.org/{}/{}", name, clean_version(name, version));
    let response = reqwest::blocking::get(&url)?.json::<serde_json::Value>()?;

    let dependencies = response["dependencies"].as_object();

    if let Some(dependencies) = dependencies {
        let deps_vec: Vec<(String, &str)> = dependencies.iter().map(|(name, version)| {
            (name.to_string(), version.as_str().unwrap())
        }).collect();

        deps_vec.par_iter().for_each(|&(ref name, ref version)| {
            // Get tarball url
            let url = format!("https://registry.npmjs.org/{}/{}", name, clean_version(name, version));
            let response = reqwest::blocking::get(&url).unwrap().json::<serde_json::Value>().unwrap();
            let dist = response["dist"].as_object().unwrap();
            let tarball = dist["tarball"].as_str().unwrap();

            let dependency = format!("{}@{}@{}", name, clean_version(name, version), tarball).to_string();
            if deps.lock().unwrap().contains(&dependency) {
                return;
            }

            deps.lock().unwrap().insert(dependency.clone());
            dmap(name, version, Arc::clone(&deps)).unwrap();
        });
    }

    Ok(())
}




pub fn get_all_deps(name: &str, version: &str) -> Result<HashSet<String>, reqwest::Error> {
    let deps = Arc::new(Mutex::new(HashSet::new()));
    let package_clone = name.clone();
    dmap(package_clone, &clean_version(name, version), Arc::clone(&deps))?;

    let mut cloned_deps = {
        let guard = deps.lock().unwrap();
        guard.clone()
    };

    // current dep
    let url = format!("https://registry.npmjs.org/{}/{}", name, version);
    let response = reqwest::blocking::get(&url)?.json::<serde_json::Value>()?;
    let dist = response["dist"].as_object().unwrap();
    let tarball = dist["tarball"].as_str().unwrap();
    
    cloned_deps.insert(format!("{}@{}@{}", name, response["version"].to_string().replace("\"", ""), tarball).to_string().to_string());
    Ok(cloned_deps)
}
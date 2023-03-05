use super::dmap::get_all_deps;
use spinners::{Spinner, Spinners};

pub fn install(name: &str, version: &str) {
    let mut sp = Spinner::new(
        Spinners::Point,
        format!("Mapping dependencies for {}@{}", name, version).into(),
    );
    let _deps = get_all_deps(&name, &version).unwrap();
    sp.stop_with_message(format!("Mapped dependencies for {}@{}", name, version).to_string());
}

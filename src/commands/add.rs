use crate::package;

pub(crate) fn run(name: String) {
    package::install::install(&name, "latest");
}
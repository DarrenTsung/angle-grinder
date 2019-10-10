//! Instructions on adding a new alias:
//! 1. Create a new file for the alias in `aliases`.
//!     1a. The filename is the string to be replaced.
//!     1b. The string inside the file is the replacement.
//! 2. Create a new test config inside `tests/structured_tests/aliases`.
//! 3. Add the test config to the `test_aliases()` test.
use lazy_static::lazy_static;

use include_dir::Dir;

const ALIASES_DIR: Dir = include_dir!("aliases");

lazy_static! {
    static ref LOADED_ALIASES: Vec<AliasConfig> = ALIASES_DIR
        .files()
        .iter()
        .map(|file| {
            toml::from_str(file.contents_utf8().expect("load string")).expect("toml valid")
        })
        .collect();
}

#[derive(Debug, Deserialize)]
struct AliasConfig {
    keyword: String,
    template: String,
}

pub fn substitute_aliases(mut v: String) -> String {
    for alias in LOADED_ALIASES.iter() {
        v = v.replace(&alias.keyword, &alias.template);
    }
    v
}

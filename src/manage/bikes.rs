use std::{cell::RefCell, error::Error, path::PathBuf, str::FromStr};

use crate::preferences::Preferences;

pub fn get_bikes_list(
    preferences: RefCell<Option<Preferences>>,
) -> Result<Vec<String>, std::boxed::Box<dyn Error>> {
    let mut mod_dir = preferences.borrow().clone().unwrap().game_dir;
    mod_dir.push(PathBuf::from_str("bikes").unwrap());
    let iterator = mod_dir.read_dir()?;
    Ok(iterator
        .filter_map(|file| {
            let file = file.ok()?;
            if file.path().is_dir() {
                Some(file.file_name().into_string().ok()?)
            } else {
                None
            }
        })
        .collect())
}

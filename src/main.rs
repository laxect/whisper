use std::collections::HashMap;

use color_eyre::Result;
use gio::Cancellable;
use libsecret::{prelude::RetrievableExtManual, SearchFlags};

fn main() -> Result<()> {
    let mut attr: HashMap<&str, &str> = HashMap::new();
    attr.insert("Type", "Menu");
    let flags = SearchFlags::ALL;
    let res = libsecret::password_search_sync(None, attr, flags, Option::<&Cancellable>::None)?;
    for entry in res {
        if let Some(app) = entry.attributes().get("App") {
            println!("{app}");
        }
    }
    Ok(())
}

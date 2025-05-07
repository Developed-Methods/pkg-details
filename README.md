## pkg-details

#### Why does this exist?
Sometimes dependencies want to know what the application's package name and version is.
Think logging or metric libraries. This crate provides a way for the application to
register its details so downstream dependencies can read them.

#### Usage
```
fn main() {
    pkg_details::init!();
}


## In a library / dependency

fn setup_logging() {
    let details = pkg_details::get();
    println!("Starting logging, pkg: {}, version: {}", details.pkg_name, details.pkg_version);
}

```

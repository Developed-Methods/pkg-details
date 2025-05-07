use std::sync::{LazyLock, RwLock};

pub static __PKG_DETAILS: LazyLock<RwLock<Option<PkgDetails>>> = LazyLock::new(|| RwLock::new(None));

#[derive(Debug, Clone)]
pub struct PkgDetails {
    pub pkg_name: &'static str,
    pub pkg_version: &'static str,
}

#[macro_export]
macro_rules! init {
    () => {{
        let d = $crate::PkgDetails {
            pkg_name: env!("CARGO_PKG_NAME"),
            pkg_version: env!("CARGO_PKG_VERSION"),
        };
        let mut lock = $crate::__PKG_DETAILS.write().unwrap();
        lock.replace(d);
    }};
}

pub fn try_get() -> Option<PkgDetails> {
    __PKG_DETAILS.read().ok()?.clone()
}

pub fn get() -> PkgDetails {
    __PKG_DETAILS
        .read().expect("had a panic writing to lock")
        .clone().expect("call to pkg_details::init!() required")
}


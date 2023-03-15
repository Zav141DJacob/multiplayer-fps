#![allow(dead_code)]

mod args;
mod client;
mod game;
mod menu;
mod net_test;
pub mod program;
mod helpers;
mod connecting;
mod error;
mod errorwindow;

/// Like [puffin::profile_scope], but allows chaining multiple scopes after each other, instead of inside another.
#[macro_export]
macro_rules! profile_scope_chain {
    (start $name:ident, $id:expr) => {
        $crate::profile_scope_chain!(start $name, $id, "");
    };
    (start $name:ident, $id:expr, $data:expr) => {
        let $name = if puffin::are_scopes_on() {
            Some(puffin::ProfilerScope::new(
                $id,
                puffin::current_file_name!(),
                $data,
            ))
        } else {
            None
        };
    };
    ($name:ident, $id:expr) => {
        $crate::profile_scope_chain!($name, $id, "");
    };
    ($name:ident, $id:expr, $data:expr) => {
        $crate::profile_scope_chain!(end $name);
        $crate::profile_scope_chain!(start $name, $id, $data);
    };
    (end $name:ident) => {
        drop($name);
    };
}
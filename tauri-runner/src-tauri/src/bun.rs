use std::env;

pub trait BunCommands {
    fn has_bun(&self) -> Option<bool>;
}

#[derive(Default)]
pub struct Bun;

impl BunCommands for Bun {
    fn has_bun(&self) -> Option<bool>
    {
        env::var_os("PATH").and_then(|paths| {
            env::split_paths(&paths).filter_map(|dir| {
                let full_path = dir.join("bun");
                if full_path.is_file() {
                    Some(true)
                } else {
                    Some(false)
                }
            }).next()
        })
    }
}
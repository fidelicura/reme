use std::env;

pub(crate) struct Cli;

impl Cli {
    pub(crate) fn mode() -> bool {
        env::args()
            .collect::<Vec<String>>()
            .iter()
            .any(|arg| arg == "-dbg" || arg == "-debug")
    }
}

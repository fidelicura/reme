use std::env;

const DEBUG_FLAG_PATTERN: [&str; 4] = ["-dbg", "-debug", "--dbg", "--debug"];

pub(crate) struct Cli;

impl Cli {
    pub(crate) fn mode() -> bool {
        env::args()
            .collect::<Vec<String>>()
            .iter()
            .any(|arg| DEBUG_FLAG_PATTERN.iter().any(|&flag| arg == flag))
    }
}

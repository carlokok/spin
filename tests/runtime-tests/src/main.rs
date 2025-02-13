use std::path::PathBuf;

use runtime_tests::{run_all, Config, OnTestError};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut args = std::env::args().skip(1).map(PathBuf::from);
    let spin_binary_path = args.next().unwrap_or_else(|| PathBuf::from("spin"));
    let tests_path = args
        .next()
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"));
    let config = Config {
        spin_binary_path,
        tests_path,
        on_error: OnTestError::Log,
    };
    run_all(config)
}

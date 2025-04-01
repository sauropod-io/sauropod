/// Get the version of the tool.
#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "version")]
pub struct VersionSubcommand {}

/// Print the version of the tool and return from the current function.
#[macro_export]
macro_rules! print_version_and_return {
    () => {
        println!(env!("CARGO_PKG_VERSION"));
        return;
    };
    ($val:expr) => {
        println!(env!("CARGO_PKG_VERSION"));
        return $val;
    };
}

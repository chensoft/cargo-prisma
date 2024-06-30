/// Prisma's command line tools
///
/// # Usage
///
/// ```bash,no_run
/// cargo install --git https://github.com/chensoft/cargo-prisma
/// cargo prisma <COMMAND> ...
/// ```
fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.get(1) != Some(&"prisma".to_string()) {
        return prisma_client_rust_cli::run();
    }

    duct::cmd(std::env::current_exe().unwrap(), &args[2..]).run().unwrap();
}
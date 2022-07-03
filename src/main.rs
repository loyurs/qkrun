use qkrun::cli;


fn main() {
    tracing_subscriber::fmt::init();
    cli::cli_run();
}

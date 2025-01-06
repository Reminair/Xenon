mod boot;
mod cli;
mod socha;
fn main() {
    boot::boot();
    cli::run_cli();
}
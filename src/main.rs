mod cli;
mod status;

fn main() {
    let mut status = status::Status::new();

    

    cli::run_cli(&mut status);
    status.write_update();
}

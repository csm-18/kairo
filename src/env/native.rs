// return command-line arguments provided by the user
pub fn get_cli_args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

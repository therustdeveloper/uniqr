fn main() {
    if let Err(e) = uniqr::exec() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

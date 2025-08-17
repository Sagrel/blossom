use tracing_appender::non_blocking::WorkerGuard;

mod token;
mod ast;
mod pretty;

fn compile_file(path: &str) -> ast::Module {
    let (tokens, interner) = token::parse(&std::fs::read_to_string(path).expect("Failed to read file"));
    ast::parse(tokens)
    
}

fn setup_logging() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::never("logs", "blossom.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();
    guard // Keep guard to prevent the log file from being dropped
}

fn main() {
    let  _guard = setup_logging();

    let (tokens, interner) = token::parse("res := 3*if x >10 { return x} else { return 0 } + 2");

    let module = ast::parse(tokens);

    println!("{}", pretty::print(&module, &interner));
}

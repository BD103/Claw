use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[cfg(not(debug_assertions))]
const MAX_LEVEL: Level = Level::WARN;

#[cfg(debug_assertions)]
const MAX_LEVEL: Level = Level::TRACE;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(MAX_LEVEL)
        .finish();
    


    tracing::subscriber::set_global_default(subscriber)
        .expect("Error setting global logging subscriber.");

    todo!();
}

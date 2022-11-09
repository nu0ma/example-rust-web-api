use tracing::Level;

pub fn set_log() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
}

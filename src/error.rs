use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to count IP"))]
    CountIpError,
    #[snafu(display("Failed to initialize app"))]
    InitializationError,
}

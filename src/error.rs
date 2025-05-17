pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BingoCardAlreadySent,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "You have already recieved a BINGO card for this event. Please check your DMs."
        )
    }
}

impl std::error::Error for Error {}

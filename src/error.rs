pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingGuildId,

    Lfg(lfg::Error),
    TempVoice(temp_voice::Error),
    Ticket(ticket::Error),
    Suggestions(suggestions::Error),
    Levels(levels::Error),
}

impl zayden_core::ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::MissingGuildId => zayden_core::Error::MissingGuildId.to_response(),
            Error::Lfg(e) => e.to_response(),
            Error::TempVoice(e) => e.to_response(),
            Error::Ticket(e) => e.to_response(),
            Error::Suggestions(e) => e.to_response(),
            Error::Levels(e) => e.to_response(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<lfg::Error> for Error {
    fn from(e: lfg::Error) -> Self {
        Error::Lfg(e)
    }
}

impl From<temp_voice::Error> for Error {
    fn from(e: temp_voice::Error) -> Self {
        Error::TempVoice(e)
    }
}

impl From<ticket::Error> for Error {
    fn from(e: ticket::Error) -> Self {
        Error::Ticket(e)
    }
}

impl From<suggestions::Error> for Error {
    fn from(e: suggestions::Error) -> Self {
        Error::Suggestions(e)
    }
}

impl From<levels::Error> for Error {
    fn from(e: levels::Error) -> Self {
        Error::Levels(e)
    }
}

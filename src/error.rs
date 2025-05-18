use serenity::all::{DiscordJsonError, ErrorResponse, HttpError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BingoCardAlreadySent,
    UnableToSendDM(serenity::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BingoCardAlreadySent => write!(
                f,
                "You have already recieved a BINGO card for this event. Please check your DMs."
            ),
            Self::UnableToSendDM(_) => {
                write!(f, "Unable to send user a DM. Please check pivacy settings")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        match value {
            serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 50007, .. },
                ..
            })) => Self::UnableToSendDM(value),
            e => panic!("Unhandled serenity error: {e:?}"),
        }
    }
}

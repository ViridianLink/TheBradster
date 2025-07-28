use zayden_core::Error as ZaydenError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BingoCardAlreadySent,
    UnableToSendDM(serenity::Error),

    Ticket(ticket::Error),

    ZaydenCore(ZaydenError),
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

            Self::Ticket(e) => e.fmt(f),

            Self::ZaydenCore(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<ticket::Error> for Error {
    fn from(value: ticket::Error) -> Self {
        match value {
            ticket::Error::ZaydenCore(e) => Self::ZaydenCore(e),
            value => Self::Ticket(value),
        }
    }
}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        Self::ZaydenCore(ZaydenError::Serenity(value))
    }
}

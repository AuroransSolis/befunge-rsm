use serde::{Deserialize, Serialize};

pub use ciborium;
pub use serde;

/// Each of the request/message types that can be sent to/from an interface.
#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    OpenConnection,
    Ack,
    Nack,
    DivByZero,
    DivByZeroAns(isize),
    ModByZero,
    ModByZeroAns(isize),
    PrintInteger(isize),
    PrintAscii(u8),
    GetInteger,
    GetIntegerAns(isize),
    GetAscii,
    GetAsciiAns(u8),
    FlushOutput,
    Debug(String),
    CloseConnection,
    CloseUi,
}

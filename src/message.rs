use std::fmt::{self, Display};
use std::convert::From;
use std::usize;

use serde::{Serialize, Deserialize};
use log::trace;

#[derive(Debug, PartialEq)]
pub enum MessageCmd {
    Ack,
    Psh,
    Wask,
    Wins,
    Inv
}

impl From<u8> for MessageCmd {
    fn from(item: u8) -> Self {
        match item {
            1 => Self::Ack,
            2 => Self::Psh,
            3 => Self::Wask,
            4 => Self::Wins,
            _ => Self::Inv
        }
    }
}

// The number of bytes in the message header
static MSG_OVERHEAD: usize = 224;

// 消息头
// 32 * 6 + 16 + 8 * 2 = 224
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Header {
    conv: u32,
    cmd: u8,
    frg: u8,
    wnd: u16,
    ts: u32,
    sn: u32,
    nua: u32,
    len: u32,
    opt: u32,
}

impl Header {
    fn is_valid(&self) -> bool {
        if let MessageCmd::Inv = self.cmd.into() {
            return false;
        }

        // TODO 更多的检查

        true
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
            conv: {}\n
            cmd: {}\n
            frg: {}\n
            wnd: {}\n
            ts: {}\n
            sn: {}\n
            nua: {}\n
            len: {}\n
            opt: {}\n
        ", self.conv, self.cmd, self.frg, self.wnd, self.ts, self.sn, self.nua, self.len, self.opt)
    }
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Ack(Header),
    Psh(Header, Vec<u8>), // 字节流的定义在这里有点奇怪🤔？不能直接用 &[u8]
    Wack(Header), // 窗口大小请求
    Wins(Header), // 窗口大小回应
}

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotAvailable,
    InvalidHeader,
    InvalidCmd,
    DeserializeError(String)
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self {
            Self::NotAvailable => "Not available".to_string(),
            Self::InvalidHeader => "Invalid header".to_string(),
            Self::InvalidCmd => "Invalid cmd".to_string(),
            Self::DeserializeError(error) => format!("Failed to deserialize: {}", error)
        };

        write!(f, "{}", err_msg)
    }
}

impl Message {
    pub fn check(buf: &[u8]) -> usize {
        if buf.len() < MSG_OVERHEAD {
            return 0;
        }

        let header: Header = bincode::deserialize(buf).unwrap();
        trace!("[Message] Header: {}", header);

        // TODO 设计MSS
        if header.len > 1500 {
            return 0;
        }

        if buf.len() < (MSG_OVERHEAD + header.len as usize) {
            return MSG_OVERHEAD + header.len as usize;
        }
        
        0
    }

    pub fn parse(buf: &[u8]) -> Result<Message> {
        let header: Header = bincode::deserialize(buf).map_err(|err| ParseError::DeserializeError(format!("{err}")))?;
        // TODO 引入log
        trace!("[Message] Header: {}", header);

        let message = match header.cmd.into() {
            MessageCmd::Ack => Message::Ack(header),
            MessageCmd::Psh => {
                // 如果类型是 Vec 的话，就只能拷贝数据了
                // TOUP 或者，试试生命周期，返回一个 Message 引用
                let (start, end): (usize, usize) = (MSG_OVERHEAD + 1, MSG_OVERHEAD + header.len as usize);
                let data = buf[start..=end].to_vec();
                Message::Psh(header, data)
            }
            MessageCmd::Wask => Message::Wack(header),
            MessageCmd::Wins => Message::Wins(header),
            MessageCmd::Inv => {
                return Err(ParseError::InvalidCmd);
            }
        };

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_message_convert_u8_into_cmd() {
        let one: MessageCmd = 1u8.into();
        assert_eq!(one, MessageCmd::Ack);

        let three: MessageCmd = 3u8.into();
        assert_eq!(three, MessageCmd::Wask);

        let invalid: MessageCmd = 200u8.into();
        assert_eq!(invalid, MessageCmd::Inv);
    }

    #[test]
    fn test_message_how_big_is_the_header() {
        //assert_eq!(MSG_OVERHEAD, size_of::<Header>());
    }

    #[test]
    fn test_message_check() {
        let not_available_buf = [1];
        assert_eq!(Message::check(&not_available_buf[..]), 0);

        let invalid_header_buf = [1u8; 225];
        assert_eq!(Message::check(&invalid_header_buf[..]), 0);
    }

    #[test]
    fn test_message_parse() {
        // TODO
    }
}
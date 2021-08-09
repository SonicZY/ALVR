mod control_socket;
mod stream_socket;

pub use control_socket::*;
pub use stream_socket::*;

use std::net::{IpAddr, Ipv4Addr};

pub const LOCAL_IP: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
<<<<<<< HEAD
pub const CONTROL_PORT: u16 = 34843;
=======
pub const CONTROL_PORT: u16 = 9844;
>>>>>>> a53f5a2985f0b6f5e88c1f402cca278289349cda
pub const MAX_HANDSHAKE_PACKET_SIZE_BYTES: usize = 4_000;

type Ldc = tokio_util::codec::LengthDelimitedCodec;

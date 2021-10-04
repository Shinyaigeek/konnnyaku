use native_tls::TlsConnector as NativeTlsConnector;
pub use native_tls::TlsStream;

pub struct TlsConnector {
    pub connection: NativeTlsConnector,
}

impl TlsConnector {
    pub fn new() -> TlsConnector {
        TlsConnector {
            connection: NativeTlsConnector::new().unwrap(),
        }
    }
}

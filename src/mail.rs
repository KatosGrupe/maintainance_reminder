use crate::config::EmailConfig;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::response::Response;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{
    ClientSecurity, ClientTlsParameters, EmailAddress, Envelope, SendableEmail,
    SmtpClient as LettreSmtpClient, SmtpTransport, Transport,
};
use native_tls::{Protocol, TlsConnector};

pub struct SmtpClient {
    transport: SmtpTransport,
}

impl SmtpClient {
    pub fn connect(config: EmailConfig) -> SmtpClient {
        let credentials = Credentials::new(config.username, config.password);

        let tls_connector = TlsConnector::builder()
            .min_protocol_version(Some(Protocol::Tlsv11))
            .build()
            .unwrap();

        let tls_parameters = ClientTlsParameters::new(config.server.clone(), tls_connector);

        let transport = LettreSmtpClient::new(
            (config.server, config.port),
            ClientSecurity::Wrapper(tls_parameters),
        )
        .unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(credentials)
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

        SmtpClient { transport }
    }

    pub fn send_email(
        &mut self,
        header: &str,
        body: &str,
        to: Vec<EmailAddress>,
        from: EmailAddress,
    ) -> Result<Response, lettre::smtp::error::Error> {
        let email = SendableEmail::new(
            Envelope::new(
                Some(from), //from
                to,         // vec![
                            //     EmailAddress::new("ignas@kata.lt".to_string()).unwrap(),
                            //     EmailAddress::new("ignas@metaloamzius.lt".to_string()).unwrap(),
                            // ], //to
            )
            .unwrap(),
            "message_id".to_string(),
            format!("SUBJECT: {}\n\n{}", header, body)
                .to_string()
                .into_bytes(),
        );

        self.transport.send(email)
    }
}

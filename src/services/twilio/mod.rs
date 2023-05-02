use crate::env::ENV;
use twilio::{Client as TwilioClient, Message, OutboundMessage, TwilioError};

lazy_static! {
    pub static ref TWILIO: Client = create_client();
}

pub struct Client(pub TwilioClient);

impl Client {
    pub async fn sms(
        &self,
        to: impl ToString,
        content: impl ToString,
    ) -> Result<Message, TwilioError> {
        debug!("Sending sms to {:?}", to.to_string());

        self.0
            .send_message(OutboundMessage::new(
                &ENV.twilio_phone,
                to.to_string().as_str(),
                content.to_string().as_str(),
            ))
            .await
    }
}

fn create_client() -> Client {
    debug!("Creating twilio client");
    trace!("account id: {}", &ENV.twilio_acc_id,);
    trace!("auth token: {}", &ENV.twilio_auth_tkn);

    let twilio_client = TwilioClient::new(&ENV.twilio_acc_id, &ENV.twilio_auth_tkn);

    let client = Client(twilio_client);
    debug!("Twilio client created");

    client
}

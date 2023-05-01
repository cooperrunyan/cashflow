use crate::ENV;
use twilio::{Client, Message, OutboundMessage, TwilioError};

lazy_static! {
    static ref TWILIO: Client = create_client();
}

fn create_client() -> Client {
    debug!("Creating twilio client");
    let client = Client::new(&ENV.twilio_acc_id, &ENV.twilio_auth_tkn);
    debug!("Twilio client created");

    client
}

pub async fn sms(to: impl ToString, content: impl ToString) -> Result<Message, TwilioError> {
    info!("Sending sms to {:?}", to.to_string());

    TWILIO
        .send_message(OutboundMessage::new(
            &ENV.twilio_phone,
            to.to_string().as_str(),
            content.to_string().as_str(),
        ))
        .await
}

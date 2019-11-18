use sendgrid::{Sender,Message,Email,Content,Personalization,
//  errors::SendgridResult
 };
use std::env;
// use futures::future::Future;

//
use crate::models::Invitation;
use crate::errors::ServiceError;

fn get_api_key() -> String {
    env::var("SENDGRID_API_KEY").expect("failed reading SENDGRID_API_KEY from env")
}

fn email_text(invitation:&Invitation) -> String {
    format!(
        "Almost there ! Just click the following link to verify your email:
    <br/>
         <a href=\"http://localhost:3000/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>

        <!--Button-->
        <center>
        <table align=\"center\" cellspacing=\"0\" cellpadding=\"0\" width=\"100%\">
        <tr>
            <td align=\"center\" style=\"padding: 10px;\">
            <table border=\"0\" class=\"mobile-button\" cellspacing=\"0\" cellpadding=\"0\">
                <tr>
                <td align=\"center\" bgcolor=\"#03b073\" style=\"background-color: #03b073; margin: auto; max-width: 600px; -webkit-border-radius: 5px; -moz-border-radius: 5px; border-radius: 5px; padding: 15px 20px; \" width=\"100%\">
                <!--[if mso]>&nbsp;<![endif]-->
                    <a href=\"#\" target=\"_blank\" style=\"18px; font-family: Helvetica, Arial, sans-serif; color: #ffffff; font-weight:normal; text-align:center; background-color: #03b073; text-decoration: none; border: none; -webkit-border-radius: 5px; -moz-border-radius: 5px; border-radius: 5px; display: inline-block;\">
                        <span style=\"font-size: 18px; font-family: Helvetica, Arial, sans-serif; color: #ffffff; font-weight:normal; line-height:1.5em; text-align:center;\">Click Here</span>
                    </a>
                <!--[if mso]>&nbsp;<![endif]-->
                </td>
                </tr>
            </table>
            </td>
        </tr>
        </table>
        </center> <br/>

        your Invitation expires on <strong>{}</strong>
    ",
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    )
}

pub async fn send_invitation(invitation: &Invitation) ->  Result<String,ServiceError>{
    // let mut cool_header = HashMap::new();
    // cool_header.insert(String::from("x-cool"), String::from("indeed"));
    // cool_header.insert(String::from("x-cooler"), String::from("cold"));

    let p = Personalization::new()
        .add_to(Email::new().set_email("marshall.nicolas@gmail.com"));
        // .add_headers(cool_header);

    let m = Message::new()
        .set_from(Email::new().set_email("notifications@recipist.co"))
        .set_subject("Verify your email")
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(&*email_text(&invitation)),
        )
        .add_personalization(p);

    let sender = Sender::new(get_api_key());

    return sender.send(&m)
}


//  let tm = Transmission::new_eu(get_api_key());
//     dbg!(&tm);
//     let sending_email = std::env::var("SENDING_EMAIL_ADDRESS")
//         .expect("failed reading SENDING_EMAIL_ADDRESS from env");
//     let mut email = Message::new(EmailAddress::new(sending_email, "let's organize"));

//     let options = Options {
//         open_tracking: false,
//         click_tracking: false,
//         transactional: true,
//         sandbox: false,
//         inline_css: false,
//         start_time: None,
//     };

//     let recipient: Recipient = invitation.email.as_str().into();

//     let email_body = format!(
//         "Please click on this button to register. Just do it !
//     <br/>
//          <a href=\"http://localhost:3000/register.html?id={}&email={}\">
//          http://localhost:3030/register</a> <br>
//          your Invitation expires on <strong>{}</strong>
//     ",
//         invitation.id,
//         invitation.email,
//         invitation
//             .expires_at
//             .format("%I:%M %p %A, %-d %B, %C%y")
//             .to_string()
//     );

//     email
//         .add_recipient(recipient)
//         .options(options)
//         .subject("You have been invited to join Simple-Auth-Server Rust")
//         .html(email_body);


//     match tm.send(&email) {
//         Ok(send_res) => match send_res {
//             TransmissionResponse::ApiResponse(api_res) => {
//                 println!("API response: \n {:#?}", api_res);
//                 Ok(())
//             }
//             TransmissionResponse::ApiError(api_errors) => {
//                 println!("API errors: \n {:#?}", &api_errors);
//                 Ok(())
//             }
//         },
//         Err(err) => {
//             println!("error: \n {:#?}", err);
//             return Err(Box::new(err))
//         }
//     }
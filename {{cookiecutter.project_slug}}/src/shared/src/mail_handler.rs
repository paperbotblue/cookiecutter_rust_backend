use chrono::Utc;
use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use std::fs::OpenOptions;

use std::io::Write;

use lettre::message::{header, SinglePart};
use lettre::transport::smtp::authentication::Mechanism;

pub fn send_mail(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Replace with your SMTP credentials and domain
    let smtp_host = "smtp.gmail.com";
    let smtp_email = "noreply@nimsuniversity.org"; // Must match your actual sender domain
    let reply_to = "noreply@nimsuniversity.org"; // Must match your actual sender domain
    let smtp_password = "wmlf smxu xwye eqxh";

    let email = Message::builder()
        .from(Mailbox::new(
            Some("digiischool".to_string()),
            smtp_email.parse()?,
        ))
        .reply_to(reply_to.parse()?)
        .to(Mailbox::new(None, to.parse()?))
        .subject(subject)
        .date_now()
        .header(header::ContentType::TEXT_HTML)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(format!(
                    r#"
                    <html>
                        <body>
                            <p>{}</p>
                            <br />
                            <p style="font-size: 12px; color: gray;">
                                Sent by <strong>Versai tech solutions</strong>
                            </p>
                        </body>
                    </html>
                    "#,
                    body
                )),
        )?;

    let creds = Credentials::new(smtp_email.to_string(), smtp_password.to_string());

    let mailer = SmtpTransport::relay(smtp_host)?
        .credentials(creds)
        .authentication(vec![Mechanism::Plain])
        .build();

    let result = mailer.send(&email);

    if let Err(e) = result {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("email_failures.log")?;
        writeln!(
            file,
            "{} ----Failed to send email to {}: {}\n DATA: {}\n",
            Utc::now(),
            to,
            e,
            body
        )?;
        return Err(Box::new(e));
    }

    println!(" Email sent successfully!");
    Ok(())
}

use crate::domain::iam::OtpPurpose;
use crate::domain::iam::policy::otp_ttl_minutes;

pub struct Body {
    pub subject: &'static str,
    pub text: String,
    pub html: String,
}

struct Letter {
    subject: &'static str,
    heading: &'static str,
    lead: &'static str,
    closing: &'static str,
}

const IGNORE_CODE: &str = "If you did not ask for this, ignore the message. Your entries stay sealed with your passphrase.";
const IGNORE_NOTICE: &str =
    "If that was not you, nothing has changed and you can ignore this message.";
const FOOTER: &str = "leafyPuff — your diary, kept to yourself.";

const VERIFY_EMAIL: Letter = Letter {
    subject: "Verify your leafyPuff email",
    heading: "Confirm your email",
    lead: "Use this code to finish setting up your diary.",
    closing: IGNORE_CODE,
};

const SIGN_IN: Letter = Letter {
    subject: "Your leafyPuff sign-in code",
    heading: "Your sign-in code",
    lead: "Use this code to sign in to leafyPuff.",
    closing: IGNORE_CODE,
};

const RESET_PASSWORD: Letter = Letter {
    subject: "Reset your leafyPuff password",
    heading: "Reset your password",
    lead: "Use this code to choose a new password. Your diary itself stays sealed by the password it was created with, so keep your recovery code within reach.",
    closing: IGNORE_CODE,
};

const EXISTING_ACCOUNT: Letter = Letter {
    subject: "You already have a leafyPuff account",
    heading: "You already have an account",
    lead: "Someone just tried to sign up with this address. There is no need — your diary is already here. Open leafyPuff and choose Log in instead.",
    closing: IGNORE_NOTICE,
};

const HTML_OPEN: &str = r#"<div style="margin:0;padding:32px 16px;background:#f6f8ec;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Helvetica,Arial,sans-serif;">
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0"><tr><td align="center">
<table role="presentation" width="480" cellpadding="0" cellspacing="0" border="0" style="width:100%;max-width:480px;background:#ffffff;border:1px solid #e7ebda;border-radius:22px;">
<tr><td style="padding:36px 34px 30px 34px;">
<p style="margin:0 0 26px 0;font-size:17px;font-weight:700;color:#6f7c48;">leafyPuff</p>
<h1 style="margin:0 0 10px 0;font-size:23px;line-height:1.3;font-weight:700;color:#242d35;">{heading}</h1>
<p style="margin:0 0 26px 0;font-size:15px;line-height:1.6;color:#6b7580;">{lead}</p>"#;

const HTML_PANEL: &str = r#"<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:#f6f8ec;border:1px solid #dee5bc;border-radius:16px;">
<tr><td align="center" style="padding:22px 16px;">
<span style="font-family:Consolas,'Liberation Mono',Menlo,monospace;font-size:32px;font-weight:700;letter-spacing:9px;color:#242d35;">{code}</span>
</td></tr></table>
<p style="margin:20px 0 0 0;font-size:14px;line-height:1.6;color:#6b7580;">This code expires in {minutes} minutes.</p>"#;

const HTML_CLOSE: &str = r#"<p style="margin:26px 0 0 0;padding-top:22px;border-top:1px solid #e7ebda;font-size:13px;line-height:1.6;color:#9ba1a8;">{closing}</p>
</td></tr></table>
<p style="margin:20px 0 0 0;font-size:12px;color:#9ba1a8;">{footer}</p>
</td></tr></table></div>"#;

const fn letter(purpose: OtpPurpose) -> Letter {
    match purpose {
        OtpPurpose::VerifyEmail => VERIFY_EMAIL,
        OtpPurpose::SignIn => SIGN_IN,
        OtpPurpose::ResetPassword => RESET_PASSWORD,
    }
}

pub fn code(code: &str, purpose: OtpPurpose) -> Body {
    let letter = letter(purpose);
    let minutes = otp_ttl_minutes();
    let panel = HTML_PANEL
        .replace("{code}", code)
        .replace("{minutes}", &minutes.to_string());
    Body {
        subject: letter.subject,
        text: format!(
            "{heading}\n\n{lead}\n\nYour code is {code}\nIt expires in {minutes} minutes.\n\n{closing}\n\n{FOOTER}",
            heading = letter.heading,
            lead = letter.lead,
            closing = letter.closing,
        ),
        html: wrap(&letter, &panel),
    }
}

pub fn existing_account() -> Body {
    let letter = EXISTING_ACCOUNT;
    Body {
        subject: letter.subject,
        text: format!(
            "{heading}\n\n{lead}\n\n{closing}\n\n{FOOTER}",
            heading = letter.heading,
            lead = letter.lead,
            closing = letter.closing,
        ),
        html: wrap(&letter, ""),
    }
}

fn wrap(letter: &Letter, panel: &str) -> String {
    let head = HTML_OPEN
        .replace("{heading}", letter.heading)
        .replace("{lead}", letter.lead);
    let tail = HTML_CLOSE
        .replace("{closing}", letter.closing)
        .replace("{footer}", FOOTER);
    format!("{head}{panel}{tail}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_two_letters_share_a_subject_or_a_heading() {
        let subjects = [
            VERIFY_EMAIL.subject,
            SIGN_IN.subject,
            RESET_PASSWORD.subject,
            EXISTING_ACCOUNT.subject,
        ];
        let headings = [
            VERIFY_EMAIL.heading,
            SIGN_IN.heading,
            RESET_PASSWORD.heading,
            EXISTING_ACCOUNT.heading,
        ];
        for pair in [subjects, headings] {
            let mut seen = pair.to_vec();
            seen.sort_unstable();
            seen.dedup();
            assert_eq!(seen.len(), pair.len());
        }
    }

    #[test]
    fn a_code_letter_carries_the_code_and_leaves_no_placeholder_behind() {
        let body = code("316361", OtpPurpose::SignIn);
        assert!(body.html.contains("316361"));
        assert!(body.text.contains("316361"));
        assert!(body.text.contains(&otp_ttl_minutes().to_string()));
        assert!(!body.html.contains('{'));
    }

    #[test]
    fn the_existing_account_notice_never_carries_a_code_panel() {
        let body = existing_account();
        assert!(!body.html.contains("letter-spacing:9px"));
        assert!(!body.html.contains("expires in"));
        assert!(!body.html.contains('{'));
    }
}

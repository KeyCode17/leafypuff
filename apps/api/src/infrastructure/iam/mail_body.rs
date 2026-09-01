use crate::domain::iam::OtpPurpose;
use crate::domain::iam::policy::otp_ttl_minutes;

const SUBJECT_VERIFY_EMAIL: &str = "Verify your leafyPuff email";
const SUBJECT_SIGN_IN: &str = "Your leafyPuff sign-in code";

const HEADING_VERIFY_EMAIL: &str = "Confirm your email";
const HEADING_SIGN_IN: &str = "Your sign-in code";

const LEAD_VERIFY_EMAIL: &str = "Use this code to finish setting up your diary.";
const LEAD_SIGN_IN: &str = "Use this code to sign in to leafyPuff.";

const CLOSING: &str = "If you did not ask for this, ignore the message. Your entries stay sealed with your passphrase.";
const FOOTER: &str = "leafyPuff — your diary, kept to yourself.";

const HTML_TEMPLATE: &str = r#"<div style="margin:0;padding:32px 16px;background:#f6f8ec;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Helvetica,Arial,sans-serif;">
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0"><tr><td align="center">
<table role="presentation" width="480" cellpadding="0" cellspacing="0" border="0" style="width:100%;max-width:480px;background:#ffffff;border:1px solid #e7ebda;border-radius:22px;">
<tr><td style="padding:36px 34px 30px 34px;">
<p style="margin:0 0 26px 0;font-size:17px;font-weight:700;color:#6f7c48;">leafyPuff</p>
<h1 style="margin:0 0 10px 0;font-size:23px;line-height:1.3;font-weight:700;color:#242d35;">{heading}</h1>
<p style="margin:0 0 26px 0;font-size:15px;line-height:1.6;color:#6b7580;">{lead}</p>
<table role="presentation" width="100%" cellpadding="0" cellspacing="0" border="0" style="background:#f6f8ec;border:1px solid #dee5bc;border-radius:16px;">
<tr><td align="center" style="padding:22px 16px;">
<span style="font-family:Consolas,'Liberation Mono',Menlo,monospace;font-size:32px;font-weight:700;letter-spacing:9px;color:#242d35;">{code}</span>
</td></tr></table>
<p style="margin:20px 0 0 0;font-size:14px;line-height:1.6;color:#6b7580;">This code expires in {minutes} minutes.</p>
<p style="margin:26px 0 0 0;padding-top:22px;border-top:1px solid #e7ebda;font-size:13px;line-height:1.6;color:#9ba1a8;">{closing}</p>
</td></tr></table>
<p style="margin:20px 0 0 0;font-size:12px;color:#9ba1a8;">{footer}</p>
</td></tr></table></div>"#;

pub const fn subject(purpose: OtpPurpose) -> &'static str {
    match purpose {
        OtpPurpose::VerifyEmail => SUBJECT_VERIFY_EMAIL,
        OtpPurpose::SignIn => SUBJECT_SIGN_IN,
    }
}

const fn heading(purpose: OtpPurpose) -> &'static str {
    match purpose {
        OtpPurpose::VerifyEmail => HEADING_VERIFY_EMAIL,
        OtpPurpose::SignIn => HEADING_SIGN_IN,
    }
}

const fn lead(purpose: OtpPurpose) -> &'static str {
    match purpose {
        OtpPurpose::VerifyEmail => LEAD_VERIFY_EMAIL,
        OtpPurpose::SignIn => LEAD_SIGN_IN,
    }
}

pub fn text(code: &str, purpose: OtpPurpose) -> String {
    format!(
        "{heading}\n\n{lead}\n\nYour code is {code}\nIt expires in {minutes} minutes.\n\n{CLOSING}\n\n{FOOTER}",
        heading = heading(purpose),
        lead = lead(purpose),
        minutes = otp_ttl_minutes(),
    )
}

pub fn html(code: &str, purpose: OtpPurpose) -> String {
    HTML_TEMPLATE
        .replace("{heading}", heading(purpose))
        .replace("{lead}", lead(purpose))
        .replace("{code}", code)
        .replace("{minutes}", &otp_ttl_minutes().to_string())
        .replace("{closing}", CLOSING)
        .replace("{footer}", FOOTER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_purposes_never_share_a_subject_or_a_heading() {
        assert_ne!(
            subject(OtpPurpose::VerifyEmail),
            subject(OtpPurpose::SignIn)
        );
        assert_ne!(
            heading(OtpPurpose::VerifyEmail),
            heading(OtpPurpose::SignIn)
        );
    }

    #[test]
    fn the_html_carries_the_code_and_leaves_no_placeholder_behind() {
        let body = html("316361", OtpPurpose::SignIn);
        assert!(body.contains("316361"));
        assert!(!body.contains('{'));
    }

    #[test]
    fn the_text_fallback_carries_the_code_and_the_expiry() {
        let body = text("316361", OtpPurpose::VerifyEmail);
        assert!(body.contains("316361"));
        assert!(body.contains(&otp_ttl_minutes().to_string()));
    }
}

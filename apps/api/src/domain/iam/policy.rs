pub const ACCESS_TOKEN_TTL_SECONDS: i64 = 15 * 60;
pub const REFRESH_TOKEN_TTL_SECONDS: i64 = 30 * 24 * 60 * 60;
pub const OTP_TTL_SECONDS: i64 = 10 * 60;

pub const fn otp_ttl_minutes() -> i64 {
    OTP_TTL_SECONDS / 60
}

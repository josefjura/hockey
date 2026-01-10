use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Sign a session ID using HMAC-SHA256
///
/// Returns the session ID with an appended signature: `session_id.signature`
///
/// # Arguments
/// * `session_id` - The raw session ID (UUID)
/// * `secret` - The secret key used for signing
///
/// # Example
/// ```ignore
/// let signed = sign_session_id("abc123", "my-secret");
/// // Returns: "abc123.deadbeef..."
/// ```
pub fn sign_session_id(session_id: &str, secret: &str) -> String {
    let signature = compute_signature(session_id, secret);
    format!("{}.{}", session_id, signature)
}

/// Verify a signed session ID and extract the original session ID
///
/// Returns `Some(session_id)` if the signature is valid, `None` otherwise
///
/// # Arguments
/// * `signed_session_id` - The signed session ID from the cookie (`session_id.signature`)
/// * `secret` - The secret key used for signing
///
/// # Example
/// ```ignore
/// let session_id = verify_signed_session_id("abc123.deadbeef...", "my-secret");
/// // Returns: Some("abc123") if valid, None if invalid
/// ```
pub fn verify_signed_session_id(signed_session_id: &str, secret: &str) -> Option<String> {
    // Split into session_id and signature
    let parts: Vec<&str> = signed_session_id.splitn(2, '.').collect();
    if parts.len() != 2 {
        return None;
    }

    let session_id = parts[0];
    let provided_signature = parts[1];

    // Compute expected signature
    let expected_signature = compute_signature(session_id, secret);

    // Constant-time comparison to prevent timing attacks
    if constant_time_compare(provided_signature, &expected_signature) {
        Some(session_id.to_string())
    } else {
        None
    }
}

/// Compute HMAC-SHA256 signature for a session ID
fn compute_signature(session_id: &str, secret: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(session_id.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut result = 0u8;
    for i in 0..a_bytes.len() {
        result |= a_bytes[i] ^ b_bytes[i];
    }

    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_valid_session() {
        let session_id = "abc123-def456-ghi789";
        let secret = "my-secret-key-with-at-least-32-characters-for-security";

        let signed = sign_session_id(session_id, secret);
        let verified = verify_signed_session_id(&signed, secret);

        assert_eq!(verified, Some(session_id.to_string()));
    }

    #[test]
    fn test_verify_invalid_signature() {
        let session_id = "abc123-def456-ghi789";
        let secret = "my-secret-key-with-at-least-32-characters-for-security";

        let signed = sign_session_id(session_id, secret);

        // Tamper with the signature
        let tampered = signed.replace('a', "b");

        let verified = verify_signed_session_id(&tampered, secret);
        assert_eq!(verified, None);
    }

    #[test]
    fn test_verify_wrong_secret() {
        let session_id = "abc123-def456-ghi789";
        let secret1 = "secret-one-with-at-least-32-chars-here!";
        let secret2 = "secret-two-with-at-least-32-chars-here!";

        let signed = sign_session_id(session_id, secret1);
        let verified = verify_signed_session_id(&signed, secret2);

        assert_eq!(verified, None);
    }

    #[test]
    fn test_verify_malformed_signed_id() {
        let secret = "my-secret-key-with-at-least-32-characters-for-security";

        // No dot separator
        assert_eq!(verify_signed_session_id("abc123", secret), None);

        // Empty string
        assert_eq!(verify_signed_session_id("", secret), None);

        // Only dot
        assert_eq!(verify_signed_session_id(".", secret), None);
    }

    #[test]
    fn test_sign_produces_different_signatures_for_different_inputs() {
        let secret = "my-secret-key-with-at-least-32-characters-for-security";

        let signed1 = sign_session_id("session1", secret);
        let signed2 = sign_session_id("session2", secret);

        assert_ne!(signed1, signed2);
    }

    #[test]
    fn test_sign_produces_same_signature_for_same_input() {
        let session_id = "abc123-def456-ghi789";
        let secret = "my-secret-key-with-at-least-32-characters-for-security";

        let signed1 = sign_session_id(session_id, secret);
        let signed2 = sign_session_id(session_id, secret);

        assert_eq!(signed1, signed2);
    }

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare("hello", "hello"));
        assert!(!constant_time_compare("hello", "world"));
        assert!(!constant_time_compare("hello", "hell"));
        assert!(!constant_time_compare("hello", "helloo"));
        assert!(!constant_time_compare("", "hello"));
        assert!(constant_time_compare("", ""));
    }
}

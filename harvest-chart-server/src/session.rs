// store session data in memory. could be refactored to use redis or something if we went to multiple servers
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

use expiring_map::ExpiringMap;
use std::time::Duration;

use anyhow::{anyhow, Result};
use oauth2::{CsrfToken, PkceCodeVerifier};

#[derive(Debug)]
pub struct OAuthVerificationInfo {
    pub pkce_code_verifier: Option<PkceCodeVerifier>, // Option<> because the library author is our mom and doesn't want us reusing this
    pub csrf_state: CsrfToken,
}

static OAUTH_INFO: Lazy<Mutex<ExpiringMap<String, OAuthVerificationInfo>>> =
    Lazy::new(|| Mutex::new(ExpiringMap::new(Duration::from_secs(60))));

pub fn insert_oauth_info(session_value: String, oauth_info: OAuthVerificationInfo) {
    OAUTH_INFO.lock().unwrap().insert(session_value, oauth_info);
}

pub fn get_oauth_info(session_value: String) -> Result<(PkceCodeVerifier, String)> {
    let pkce_code_verifier;
    let csrf_state;

    // get these things out of OAUTH_INFO so we can drop the lock right away
    if let Some(oauth_info) = OAUTH_INFO.lock().unwrap().get_mut(&session_value) {
        // get ownership out of the Option<>
        let pkce_code_verifier_option = std::mem::replace(&mut oauth_info.pkce_code_verifier, None);
        csrf_state = oauth_info.csrf_state.secret().clone();

        if pkce_code_verifier_option.is_none() {
            return Err(anyhow!("oauth info already used"));
        }
        pkce_code_verifier = pkce_code_verifier_option.unwrap();
    } else {
        return Err(anyhow!("oauth info not found"));
    }

    return Ok((pkce_code_verifier, csrf_state));
}

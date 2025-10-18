use oauth2::{ClientId, ClientSecret, RedirectUrl, AuthUrl, TokenUrl, Scope, TokenResponse};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::errors::{AppError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub verified_email: bool,
}

pub struct GoogleOAuth {
    client: BasicClient,
    http_client: Client,
}

impl GoogleOAuth {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

        let http_client = Client::new();

        Self {
            client,
            http_client,
        }
    }

    pub fn get_authorization_url(&self) -> (String, String) {
        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url.to_string(), csrf_token.secret().to_string())
    }

    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        pkce_verifier: &str,
    ) -> Result<String> {
        let token_result = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier.to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|e| AppError::OAuth(format!("Token exchange failed: {}", e)))?;

        Ok(token_result.access_token().secret().to_string())
    }

    pub async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo> {
        let response = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::OAuth(format!("Failed to get user info: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::OAuth("Failed to get user info from Google".to_string()));
        }

        let user_info: GoogleUserInfo = response
            .json()
            .await
            .map_err(|e| AppError::OAuth(format!("Failed to parse user info: {}", e)))?;

        Ok(user_info)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    pub auth_url: String,
}

#[derive(Deserialize, Default)]
pub struct CallBackParams {
    #[serde(flatten)]
    pub success: Option<AuthSuccessParams>,

    #[serde(flatten)]
    pub error: Option<AuthErrorParams>,
}

#[derive(Debug, Deserialize)]
pub struct AuthSuccessParams {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthErrorParams {
    pub error: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

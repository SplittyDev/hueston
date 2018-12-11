use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HueBridgeRegistration {
    success: Option<HueBridgeRegistrationSuccess>,
    error: Option<HueBridgeError>,
}

#[derive(Debug, Deserialize)]
pub struct HueBridgeRegistrationSuccess {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct HueBridgeError {
    r#type: i32,
    address: String,
    description: String,
}

impl HueBridgeError {

    pub fn error_code(&self) -> HueErrorCode {
        match self.r#type {
            101 => HueErrorCode::LinkButtonNotPressed,
            _ => HueErrorCode::Unknown,
        }
    }
}

pub enum HueErrorCode {
    Unknown,
    LinkButtonNotPressed,
}

impl HueBridgeRegistration {

    /// Get the username
    pub fn get_username(&self) -> Option<&String> {
        match &self.success {
            Some(s) => {
                Some(&s.username)
            },
            None => None,
        }
    }

    /// Transform the response into a `Result`
    pub fn into_result(self) -> Result<HueBridgeRegistrationSuccess, HueBridgeError> {
        if let Some(success) = self.success {
            Ok(success)
        } else if let Some(error) = self.error {
            Err(error)
        } else {
            unimplemented!()
        }
    }
}
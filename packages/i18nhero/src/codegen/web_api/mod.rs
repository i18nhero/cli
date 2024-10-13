#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///AuthTokens
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "access_token",
    ///    "refresh_token"
    ///  ],
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "refresh_token": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AuthTokens {
        pub access_token: String,
        pub refresh_token: String,
    }

    impl From<&AuthTokens> for AuthTokens {
        fn from(value: &AuthTokens) -> Self {
            value.clone()
        }
    }

    ///CreateOrganizationInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateOrganizationInput {
        pub title: String,
    }

    impl From<&CreateOrganizationInput> for CreateOrganizationInput {
        fn from(value: &CreateOrganizationInput) -> Self {
            value.clone()
        }
    }

    ///CreateOrganizationInviteInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateOrganizationInviteInput {
        pub email: String,
    }

    impl From<&CreateOrganizationInviteInput> for CreateOrganizationInviteInput {
        fn from(value: &CreateOrganizationInviteInput) -> Self {
            value.clone()
        }
    }

    ///CreateOrganizationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateOrganizationResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&CreateOrganizationResponse> for CreateOrganizationResponse {
        fn from(value: &CreateOrganizationResponse) -> Self {
            value.clone()
        }
    }

    ///CreatePersonalAccessTokenInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreatePersonalAccessTokenInput {
        pub description: String,
    }

    impl From<&CreatePersonalAccessTokenInput> for CreatePersonalAccessTokenInput {
        fn from(value: &CreatePersonalAccessTokenInput) -> Self {
            value.clone()
        }
    }

    ///CreatePersonalAccessTokenResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreatePersonalAccessTokenResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&CreatePersonalAccessTokenResponse> for CreatePersonalAccessTokenResponse {
        fn from(value: &CreatePersonalAccessTokenResponse) -> Self {
            value.clone()
        }
    }

    ///CreateProjectInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "is_public",
    ///    "localizations",
    ///    "organization_id",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "is_public": {
    ///      "type": "boolean"
    ///    },
    ///    "localizations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CreateProjectInputLocalization"
    ///      }
    ///    },
    ///    "organization_id": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateProjectInput {
        pub description: String,
        pub is_public: bool,
        pub localizations: Vec<CreateProjectInputLocalization>,
        pub organization_id: String,
        pub title: String,
    }

    impl From<&CreateProjectInput> for CreateProjectInput {
        fn from(value: &CreateProjectInput) -> Self {
            value.clone()
        }
    }

    ///CreateProjectInputLocalization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "country_code",
    ///    "enabled",
    ///    "language_code"
    ///  ],
    ///  "properties": {
    ///    "country_code": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "language_code": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateProjectInputLocalization {
        pub country_code: Option<String>,
        pub enabled: bool,
        pub language_code: String,
    }

    impl From<&CreateProjectInputLocalization> for CreateProjectInputLocalization {
        fn from(value: &CreateProjectInputLocalization) -> Self {
            value.clone()
        }
    }

    ///CreateProjectResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateProjectResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&CreateProjectResponse> for CreateProjectResponse {
        fn from(value: &CreateProjectResponse) -> Self {
            value.clone()
        }
    }

    ///CreateProjectWebhookInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "enabled",
    ///    "triggers",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "triggers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProjectWebhookEvent"
    ///      }
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateProjectWebhookInput {
        pub description: String,
        pub enabled: bool,
        pub triggers: Vec<ProjectWebhookEvent>,
        pub url: String,
    }

    impl From<&CreateProjectWebhookInput> for CreateProjectWebhookInput {
        fn from(value: &CreateProjectWebhookInput) -> Self {
            value.clone()
        }
    }

    ///CreateProjectWebhookResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateProjectWebhookResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&CreateProjectWebhookResponse> for CreateProjectWebhookResponse {
        fn from(value: &CreateProjectWebhookResponse) -> Self {
            value.clone()
        }
    }

    ///CreateTranslationInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "identifier",
    ///    "locales"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "identifier": {
    ///      "type": "string"
    ///    },
    ///    "locales": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateTranslationInput {
        pub description: String,
        pub identifier: String,
        pub locales: ::std::collections::HashMap<String, String>,
    }

    impl From<&CreateTranslationInput> for CreateTranslationInput {
        fn from(value: &CreateTranslationInput) -> Self {
            value.clone()
        }
    }

    ///CreateTranslationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CreateTranslationResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&CreateTranslationResponse> for CreateTranslationResponse {
        fn from(value: &CreateTranslationResponse) -> Self {
            value.clone()
        }
    }

    ///DeleteOrganizationInviteByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteOrganizationInviteByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&DeleteOrganizationInviteByIdResponse> for DeleteOrganizationInviteByIdResponse {
        fn from(value: &DeleteOrganizationInviteByIdResponse) -> Self {
            value.clone()
        }
    }

    ///DeleteProjectWebhookByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteProjectWebhookByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&DeleteProjectWebhookByIdResponse> for DeleteProjectWebhookByIdResponse {
        fn from(value: &DeleteProjectWebhookByIdResponse) -> Self {
            value.clone()
        }
    }

    ///DeleteTranslationByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DeleteTranslationByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&DeleteTranslationByIdResponse> for DeleteTranslationByIdResponse {
        fn from(value: &DeleteTranslationByIdResponse) -> Self {
            value.clone()
        }
    }

    ///EmailLoginInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "password"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "password": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EmailLoginInput {
        pub email: String,
        pub password: String,
    }

    impl From<&EmailLoginInput> for EmailLoginInput {
        fn from(value: &EmailLoginInput) -> Self {
            value.clone()
        }
    }

    ///EmailRegisterInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "full_name",
    ///    "password"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "full_name": {
    ///      "type": "string"
    ///    },
    ///    "password": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EmailRegisterInput {
        pub email: String,
        pub full_name: String,
        pub password: String,
    }

    impl From<&EmailRegisterInput> for EmailRegisterInput {
        fn from(value: &EmailRegisterInput) -> Self {
            value.clone()
        }
    }

    ///ExportProjectOutput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "content",
    ///    "country_code",
    ///    "language_code"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "country_code": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "language_code": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExportProjectOutput {
        pub content: String,
        pub country_code: Option<String>,
        pub language_code: String,
    }

    impl From<&ExportProjectOutput> for ExportProjectOutput {
        fn from(value: &ExportProjectOutput) -> Self {
            value.clone()
        }
    }

    ///FileFormat
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "json"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FileFormat {
        #[serde(rename = "json")]
        Json,
    }

    impl From<&FileFormat> for FileFormat {
        fn from(value: &FileFormat) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for FileFormat {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Json => write!(f, "json"),
            }
        }
    }

    impl std::str::FromStr for FileFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "json" => Ok(Self::Json),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for FileFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for FileFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for FileFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///GetOrganizationByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetOrganizationByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetOrganizationByIdResponse> for GetOrganizationByIdResponse {
        fn from(value: &GetOrganizationByIdResponse) -> Self {
            value.clone()
        }
    }

    ///GetOrganizationMembersResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetOrganizationMembersResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetOrganizationMembersResponse> for GetOrganizationMembersResponse {
        fn from(value: &GetOrganizationMembersResponse) -> Self {
            value.clone()
        }
    }

    ///GetOrganizationProjectOverviewResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetOrganizationProjectOverviewResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetOrganizationProjectOverviewResponse> for GetOrganizationProjectOverviewResponse {
        fn from(value: &GetOrganizationProjectOverviewResponse) -> Self {
            value.clone()
        }
    }

    ///GetOrganizationsByUserIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetOrganizationsByUserIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetOrganizationsByUserIdResponse> for GetOrganizationsByUserIdResponse {
        fn from(value: &GetOrganizationsByUserIdResponse) -> Self {
            value.clone()
        }
    }

    ///GetPendingOrganizationInvitesResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetPendingOrganizationInvitesResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetPendingOrganizationInvitesResponse> for GetPendingOrganizationInvitesResponse {
        fn from(value: &GetPendingOrganizationInvitesResponse) -> Self {
            value.clone()
        }
    }

    ///GetPersonalAccessTokensResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetPersonalAccessTokensResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetPersonalAccessTokensResponse> for GetPersonalAccessTokensResponse {
        fn from(value: &GetPersonalAccessTokensResponse) -> Self {
            value.clone()
        }
    }

    ///GetProjectByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetProjectByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetProjectByIdResponse> for GetProjectByIdResponse {
        fn from(value: &GetProjectByIdResponse) -> Self {
            value.clone()
        }
    }

    ///GetProjectMetricsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetProjectMetricsResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetProjectMetricsResponse> for GetProjectMetricsResponse {
        fn from(value: &GetProjectMetricsResponse) -> Self {
            value.clone()
        }
    }

    ///GetProjectTranslationsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetProjectTranslationsResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetProjectTranslationsResponse> for GetProjectTranslationsResponse {
        fn from(value: &GetProjectTranslationsResponse) -> Self {
            value.clone()
        }
    }

    ///GetProjectWebhookByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetProjectWebhookByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetProjectWebhookByIdResponse> for GetProjectWebhookByIdResponse {
        fn from(value: &GetProjectWebhookByIdResponse) -> Self {
            value.clone()
        }
    }

    ///GetProjectWebhooksResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetProjectWebhooksResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetProjectWebhooksResponse> for GetProjectWebhooksResponse {
        fn from(value: &GetProjectWebhooksResponse) -> Self {
            value.clone()
        }
    }

    ///GetProjectsByOrganizationIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetProjectsByOrganizationIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetProjectsByOrganizationIdResponse> for GetProjectsByOrganizationIdResponse {
        fn from(value: &GetProjectsByOrganizationIdResponse) -> Self {
            value.clone()
        }
    }

    ///GetTranslationByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetTranslationByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetTranslationByIdResponse> for GetTranslationByIdResponse {
        fn from(value: &GetTranslationByIdResponse) -> Self {
            value.clone()
        }
    }

    ///GetUserByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetUserByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&GetUserByIdResponse> for GetUserByIdResponse {
        fn from(value: &GetUserByIdResponse) -> Self {
            value.clone()
        }
    }

    ///InvalidatePersonalAccessTokenByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InvalidatePersonalAccessTokenByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&InvalidatePersonalAccessTokenByIdResponse>
        for InvalidatePersonalAccessTokenByIdResponse
    {
        fn from(value: &InvalidatePersonalAccessTokenByIdResponse) -> Self {
            value.clone()
        }
    }

    ///InviteUserToOrganizationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InviteUserToOrganizationResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&InviteUserToOrganizationResponse> for InviteUserToOrganizationResponse {
        fn from(value: &InviteUserToOrganizationResponse) -> Self {
            value.clone()
        }
    }

    ///LeaveOrganizationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LeaveOrganizationResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&LeaveOrganizationResponse> for LeaveOrganizationResponse {
        fn from(value: &LeaveOrganizationResponse) -> Self {
            value.clone()
        }
    }

    ///LoginWithEmailResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LoginWithEmailResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&LoginWithEmailResponse> for LoginWithEmailResponse {
        fn from(value: &LoginWithEmailResponse) -> Self {
            value.clone()
        }
    }

    ///Organization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "allow_ai_translations",
    ///    "created_at",
    ///    "deleted_at",
    ///    "modified_at",
    ///    "title",
    ///    "users"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "allow_ai_translations": {
    ///      "type": "boolean"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "modified_at": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    },
    ///    "users": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrganizationUser"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Organization {
        pub allow_ai_translations: bool,
        pub created_at: String,
        pub deleted_at: Option<String>,
        #[serde(rename = "_id")]
        pub id: String,
        pub modified_at: String,
        pub title: String,
        pub users: Vec<OrganizationUser>,
    }

    impl From<&Organization> for Organization {
        fn from(value: &Organization) -> Self {
            value.clone()
        }
    }

    ///OrganizationInvite
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "created_at",
    ///    "deleted_at",
    ///    "email",
    ///    "modified_at",
    ///    "organization_id"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "modified_at": {
    ///      "type": "string"
    ///    },
    ///    "organization_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationInvite {
        pub created_at: String,
        pub deleted_at: Option<String>,
        pub email: String,
        #[serde(rename = "_id")]
        pub id: String,
        pub modified_at: String,
        pub organization_id: String,
    }

    impl From<&OrganizationInvite> for OrganizationInvite {
        fn from(value: &OrganizationInvite) -> Self {
            value.clone()
        }
    }

    ///OrganizationMember
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "email",
    ///    "full_name",
    ///    "profile_image_url",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "full_name": {
    ///      "type": "string"
    ///    },
    ///    "profile_image_url": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationMember {
        pub email: String,
        pub full_name: String,
        pub profile_image_url: String,
        pub user_id: String,
    }

    impl From<&OrganizationMember> for OrganizationMember {
        fn from(value: &OrganizationMember) -> Self {
            value.clone()
        }
    }

    ///OrganizationProjectOverview
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "projects",
    ///    "total_identifiers"
    ///  ],
    ///  "properties": {
    ///    "projects": {
    ///      "type": "number"
    ///    },
    ///    "total_identifiers": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationProjectOverview {
        pub projects: f64,
        pub total_identifiers: f64,
    }

    impl From<&OrganizationProjectOverview> for OrganizationProjectOverview {
        fn from(value: &OrganizationProjectOverview) -> Self {
            value.clone()
        }
    }

    ///OrganizationUser
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationUser {
        pub user_id: String,
    }

    impl From<&OrganizationUser> for OrganizationUser {
        fn from(value: &OrganizationUser) -> Self {
            value.clone()
        }
    }

    ///PersonalAccessToken
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "created_at",
    ///    "description",
    ///    "secret",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    },
    ///    "user_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PersonalAccessToken {
        pub created_at: String,
        pub description: String,
        #[serde(rename = "_id")]
        pub id: String,
        pub secret: String,
        pub user_id: String,
    }

    impl From<&PersonalAccessToken> for PersonalAccessToken {
        fn from(value: &PersonalAccessToken) -> Self {
            value.clone()
        }
    }

    ///Project
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "allow_ai_translations",
    ///    "auto_approve_translations",
    ///    "auto_translate",
    ///    "created_at",
    ///    "deleted_at",
    ///    "description",
    ///    "is_public",
    ///    "localizations",
    ///    "modified_at",
    ///    "organization_id",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "allow_ai_translations": {
    ///      "type": "boolean"
    ///    },
    ///    "auto_approve_translations": {
    ///      "type": "boolean"
    ///    },
    ///    "auto_translate": {
    ///      "type": "boolean"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "is_public": {
    ///      "type": "boolean"
    ///    },
    ///    "localizations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProjectLocalization"
    ///      }
    ///    },
    ///    "modified_at": {
    ///      "type": "string"
    ///    },
    ///    "organization_id": {
    ///      "type": "string"
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Project {
        pub allow_ai_translations: bool,
        pub auto_approve_translations: bool,
        pub auto_translate: bool,
        pub created_at: String,
        pub deleted_at: Option<String>,
        pub description: String,
        #[serde(rename = "_id")]
        pub id: String,
        pub is_public: bool,
        pub localizations: Vec<ProjectLocalization>,
        pub modified_at: String,
        pub organization_id: String,
        pub title: String,
    }

    impl From<&Project> for Project {
        fn from(value: &Project) -> Self {
            value.clone()
        }
    }

    ///ProjectLocalization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "country_code",
    ///    "enabled",
    ///    "language_code"
    ///  ],
    ///  "properties": {
    ///    "country_code": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "language_code": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectLocalization {
        pub country_code: Option<String>,
        pub enabled: bool,
        pub language_code: String,
    }

    impl From<&ProjectLocalization> for ProjectLocalization {
        fn from(value: &ProjectLocalization) -> Self {
            value.clone()
        }
    }

    ///ProjectMetrics
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "identifier_count",
    ///    "locales",
    ///    "project_id"
    ///  ],
    ///  "properties": {
    ///    "identifier_count": {
    ///      "type": "number"
    ///    },
    ///    "locales": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "number"
    ///      }
    ///    },
    ///    "project_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectMetrics {
        pub identifier_count: f64,
        pub locales: ::std::collections::HashMap<String, f64>,
        pub project_id: String,
    }

    impl From<&ProjectMetrics> for ProjectMetrics {
        fn from(value: &ProjectMetrics) -> Self {
            value.clone()
        }
    }

    ///ProjectWebhook
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "created_at",
    ///    "deleted_at",
    ///    "description",
    ///    "enabled",
    ///    "modified_at",
    ///    "organization_id",
    ///    "project_id",
    ///    "triggers",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "modified_at": {
    ///      "type": "string"
    ///    },
    ///    "organization_id": {
    ///      "type": "string"
    ///    },
    ///    "project_id": {
    ///      "type": "string"
    ///    },
    ///    "triggers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProjectWebhookEvent"
    ///      }
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ProjectWebhook {
        pub created_at: String,
        pub deleted_at: Option<String>,
        pub description: String,
        pub enabled: bool,
        #[serde(rename = "_id")]
        pub id: String,
        pub modified_at: String,
        pub organization_id: String,
        pub project_id: String,
        pub triggers: Vec<ProjectWebhookEvent>,
        pub url: String,
    }

    impl From<&ProjectWebhook> for ProjectWebhook {
        fn from(value: &ProjectWebhook) -> Self {
            value.clone()
        }
    }

    ///ProjectWebhookEvent
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "key:created",
    ///    "key:bulk:created",
    ///    "key:updated",
    ///    "key:bulk:updated",
    ///    "key:deleted",
    ///    "key:bulk:deleted"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ProjectWebhookEvent {
        #[serde(rename = "key:created")]
        KeyCreated,
        #[serde(rename = "key:bulk:created")]
        KeyBulkCreated,
        #[serde(rename = "key:updated")]
        KeyUpdated,
        #[serde(rename = "key:bulk:updated")]
        KeyBulkUpdated,
        #[serde(rename = "key:deleted")]
        KeyDeleted,
        #[serde(rename = "key:bulk:deleted")]
        KeyBulkDeleted,
    }

    impl From<&ProjectWebhookEvent> for ProjectWebhookEvent {
        fn from(value: &ProjectWebhookEvent) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ProjectWebhookEvent {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::KeyCreated => write!(f, "key:created"),
                Self::KeyBulkCreated => write!(f, "key:bulk:created"),
                Self::KeyUpdated => write!(f, "key:updated"),
                Self::KeyBulkUpdated => write!(f, "key:bulk:updated"),
                Self::KeyDeleted => write!(f, "key:deleted"),
                Self::KeyBulkDeleted => write!(f, "key:bulk:deleted"),
            }
        }
    }

    impl std::str::FromStr for ProjectWebhookEvent {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "key:created" => Ok(Self::KeyCreated),
                "key:bulk:created" => Ok(Self::KeyBulkCreated),
                "key:updated" => Ok(Self::KeyUpdated),
                "key:bulk:updated" => Ok(Self::KeyBulkUpdated),
                "key:deleted" => Ok(Self::KeyDeleted),
                "key:bulk:deleted" => Ok(Self::KeyBulkDeleted),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ProjectWebhookEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ProjectWebhookEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ProjectWebhookEvent {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///PushLocaleInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "files"
    ///  ],
    ///  "properties": {
    ///    "files": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PushLocaleInputFile"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PushLocaleInput {
        pub files: Vec<PushLocaleInputFile>,
    }

    impl From<&PushLocaleInput> for PushLocaleInput {
        fn from(value: &PushLocaleInput) -> Self {
            value.clone()
        }
    }

    ///PushLocaleInputFile
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "content",
    ///    "file_format",
    ///    "file_name"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "file_format": {
    ///      "$ref": "#/components/schemas/FileFormat"
    ///    },
    ///    "file_name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PushLocaleInputFile {
        pub content: String,
        pub file_format: FileFormat,
        pub file_name: String,
    }

    impl From<&PushLocaleInputFile> for PushLocaleInputFile {
        fn from(value: &PushLocaleInputFile) -> Self {
            value.clone()
        }
    }

    ///PushLocaleResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "inserted",
    ///    "modified"
    ///  ],
    ///  "properties": {
    ///    "inserted": {
    ///      "type": "number"
    ///    },
    ///    "modified": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PushLocaleResult {
        pub inserted: f64,
        pub modified: f64,
    }

    impl From<&PushLocaleResult> for PushLocaleResult {
        fn from(value: &PushLocaleResult) -> Self {
            value.clone()
        }
    }

    ///RefreshAuthTokensInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "refresh_token"
    ///  ],
    ///  "properties": {
    ///    "refresh_token": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RefreshAuthTokensInput {
        pub refresh_token: String,
    }

    impl From<&RefreshAuthTokensInput> for RefreshAuthTokensInput {
        fn from(value: &RefreshAuthTokensInput) -> Self {
            value.clone()
        }
    }

    ///RefreshAuthTokensResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RefreshAuthTokensResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&RefreshAuthTokensResponse> for RefreshAuthTokensResponse {
        fn from(value: &RefreshAuthTokensResponse) -> Self {
            value.clone()
        }
    }

    ///RegisterWithEmailResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RegisterWithEmailResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&RegisterWithEmailResponse> for RegisterWithEmailResponse {
        fn from(value: &RegisterWithEmailResponse) -> Self {
            value.clone()
        }
    }

    ///RemoveUserFromOrganizationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RemoveUserFromOrganizationResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&RemoveUserFromOrganizationResponse> for RemoveUserFromOrganizationResponse {
        fn from(value: &RemoveUserFromOrganizationResponse) -> Self {
            value.clone()
        }
    }

    ///TranslateAiInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "from"
    ///  ],
    ///  "properties": {
    ///    "from": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TranslateAiInput {
        pub from: String,
    }

    impl From<&TranslateAiInput> for TranslateAiInput {
        fn from(value: &TranslateAiInput) -> Self {
            value.clone()
        }
    }

    ///TranslateAiResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "locale",
    ///    "translated"
    ///  ],
    ///  "properties": {
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "translated": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TranslateAiResult {
        pub locale: String,
        pub translated: String,
    }

    impl From<&TranslateAiResult> for TranslateAiResult {
        fn from(value: &TranslateAiResult) -> Self {
            value.clone()
        }
    }

    ///TranslateUsingAiResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TranslateUsingAiResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&TranslateUsingAiResponse> for TranslateUsingAiResponse {
        fn from(value: &TranslateUsingAiResponse) -> Self {
            value.clone()
        }
    }

    ///Translation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "created_at",
    ///    "deleted_at",
    ///    "description",
    ///    "identifier",
    ///    "locales",
    ///    "modified_at",
    ///    "organization_id",
    ///    "project_id"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "identifier": {
    ///      "type": "string"
    ///    },
    ///    "locales": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "modified_at": {
    ///      "type": "string"
    ///    },
    ///    "organization_id": {
    ///      "type": "string"
    ///    },
    ///    "project_id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Translation {
        pub created_at: String,
        pub deleted_at: Option<String>,
        pub description: String,
        #[serde(rename = "_id")]
        pub id: String,
        pub identifier: String,
        pub locales: ::std::collections::HashMap<String, String>,
        pub modified_at: String,
        pub organization_id: String,
        pub project_id: String,
    }

    impl From<&Translation> for Translation {
        fn from(value: &Translation) -> Self {
            value.clone()
        }
    }

    ///UpdateOrganizationByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateOrganizationByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&UpdateOrganizationByIdResponse> for UpdateOrganizationByIdResponse {
        fn from(value: &UpdateOrganizationByIdResponse) -> Self {
            value.clone()
        }
    }

    ///UpdateOrganizationInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateOrganizationInput {
        pub title: String,
    }

    impl From<&UpdateOrganizationInput> for UpdateOrganizationInput {
        fn from(value: &UpdateOrganizationInput) -> Self {
            value.clone()
        }
    }

    ///UpdateProjectByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateProjectByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&UpdateProjectByIdResponse> for UpdateProjectByIdResponse {
        fn from(value: &UpdateProjectByIdResponse) -> Self {
            value.clone()
        }
    }

    ///UpdateProjectInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "is_public",
    ///    "localizations",
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "is_public": {
    ///      "type": "boolean"
    ///    },
    ///    "localizations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UpdateProjectInputLocalization"
    ///      }
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateProjectInput {
        pub description: String,
        pub is_public: bool,
        pub localizations: Vec<UpdateProjectInputLocalization>,
        pub title: String,
    }

    impl From<&UpdateProjectInput> for UpdateProjectInput {
        fn from(value: &UpdateProjectInput) -> Self {
            value.clone()
        }
    }

    ///UpdateProjectInputLocalization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "country_code",
    ///    "enabled",
    ///    "language_code"
    ///  ],
    ///  "properties": {
    ///    "country_code": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "language_code": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateProjectInputLocalization {
        pub country_code: Option<String>,
        pub enabled: bool,
        pub language_code: String,
    }

    impl From<&UpdateProjectInputLocalization> for UpdateProjectInputLocalization {
        fn from(value: &UpdateProjectInputLocalization) -> Self {
            value.clone()
        }
    }

    ///UpdateProjectWebhookByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateProjectWebhookByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&UpdateProjectWebhookByIdResponse> for UpdateProjectWebhookByIdResponse {
        fn from(value: &UpdateProjectWebhookByIdResponse) -> Self {
            value.clone()
        }
    }

    ///UpdateProjectWebhookInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "enabled",
    ///    "triggers",
    ///    "url"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "triggers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProjectWebhookEvent"
    ///      }
    ///    },
    ///    "url": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateProjectWebhookInput {
        pub description: String,
        pub enabled: bool,
        pub triggers: Vec<ProjectWebhookEvent>,
        pub url: String,
    }

    impl From<&UpdateProjectWebhookInput> for UpdateProjectWebhookInput {
        fn from(value: &UpdateProjectWebhookInput) -> Self {
            value.clone()
        }
    }

    ///UpdateTranslationByIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateTranslationByIdResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&UpdateTranslationByIdResponse> for UpdateTranslationByIdResponse {
        fn from(value: &UpdateTranslationByIdResponse) -> Self {
            value.clone()
        }
    }

    ///UpdateTranslationInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "description",
    ///    "identifier",
    ///    "locales"
    ///  ],
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "identifier": {
    ///      "type": "string"
    ///    },
    ///    "locales": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateTranslationInput {
        pub description: String,
        pub identifier: String,
        pub locales: ::std::collections::HashMap<String, String>,
    }

    impl From<&UpdateTranslationInput> for UpdateTranslationInput {
        fn from(value: &UpdateTranslationInput) -> Self {
            value.clone()
        }
    }

    ///UpdateTranslationLocaleInput
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "locale",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateTranslationLocaleInput {
        pub locale: String,
        pub value: String,
    }

    impl From<&UpdateTranslationLocaleInput> for UpdateTranslationLocaleInput {
        fn from(value: &UpdateTranslationLocaleInput) -> Self {
            value.clone()
        }
    }

    ///UpdateTranslationLocaleResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "statusCode"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "statusCode": {
    ///      "type": "number"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UpdateTranslationLocaleResponse {
        pub message: String,
        #[serde(rename = "statusCode")]
        pub status_code: f64,
    }

    impl From<&UpdateTranslationLocaleResponse> for UpdateTranslationLocaleResponse {
        fn from(value: &UpdateTranslationLocaleResponse) -> Self {
            value.clone()
        }
    }

    ///User
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "_id",
    ///    "created_at",
    ///    "deleted_at",
    ///    "full_name",
    ///    "modified_at"
    ///  ],
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "created_at": {
    ///      "type": "string"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "full_name": {
    ///      "type": "string"
    ///    },
    ///    "modified_at": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct User {
        pub created_at: String,
        pub deleted_at: Option<String>,
        pub full_name: String,
        #[serde(rename = "_id")]
        pub id: String,
        pub modified_at: String,
    }

    impl From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for @i18nhero/web-api
///
///Api for i18nhero.com
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}

#[allow(clippy::all)]
impl Client {
    ///Sends a `GET` request to `/`
    pub async fn get_hello<'a>(&'a self) -> Result<ResponseValue<String>, Error<()>> {
        let url = format!("{}/", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/users/{user_id}`
    pub async fn get_user_by_id<'a>(
        &'a self,
        user_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::User>, Error<types::GetUserByIdResponse>> {
        let url = format!(
            "{}/users/{}",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/users/{user_id}/organizations`
    pub async fn get_organizations_by_user_id<'a>(
        &'a self,
        user_id: &'a str,
        authorization: &'a str,
    ) -> Result<
        ResponseValue<Vec<types::Organization>>,
        Error<types::GetOrganizationsByUserIdResponse>,
    > {
        let url = format!(
            "{}/users/{}/organizations",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/users/{user_id}/organizations/{organization_id}/leave`
    pub async fn leave_organization<'a>(
        &'a self,
        user_id: &'a str,
        organization_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::LeaveOrganizationResponse>> {
        let url = format!(
            "{}/users/{}/organizations/{}/leave",
            self.baseurl,
            encode_path(&user_id.to_string()),
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/organizations`
    pub async fn create_organization<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::CreateOrganizationInput,
    ) -> Result<ResponseValue<types::Organization>, Error<types::CreateOrganizationResponse>> {
        let url = format!("{}/organizations", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/organizations/{organization_id}`
    pub async fn get_organization_by_id<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::Organization>, Error<types::GetOrganizationByIdResponse>> {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/organizations/{organization_id}`
    pub async fn update_organization_by_id<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
        body: &'a types::UpdateOrganizationInput,
    ) -> Result<ResponseValue<types::Organization>, Error<types::UpdateOrganizationByIdResponse>>
    {
        let url = format!(
            "{}/organizations/{}",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/organizations/{organization_id}/projects`
    pub async fn get_projects_by_organization_id<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<Vec<types::Project>>, Error<types::GetProjectsByOrganizationIdResponse>>
    {
        let url = format!(
            "{}/organizations/{}/projects",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/organizations/{organization_id}/users`
    pub async fn get_organization_members<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
    ) -> Result<
        ResponseValue<Vec<types::OrganizationMember>>,
        Error<types::GetOrganizationMembersResponse>,
    > {
        let url = format!(
            "{}/organizations/{}/users",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/organizations/{organization_id}/users/{user_id}/remove`
    pub async fn remove_user_from_organization<'a>(
        &'a self,
        organization_id: &'a str,
        user_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::RemoveUserFromOrganizationResponse>> {
        let url = format!(
            "{}/organizations/{}/users/{}/remove",
            self.baseurl,
            encode_path(&organization_id.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/organizations/{organization_id}/invites`
    pub async fn get_pending_organization_invites<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
    ) -> Result<
        ResponseValue<Vec<types::OrganizationInvite>>,
        Error<types::GetPendingOrganizationInvitesResponse>,
    > {
        let url = format!(
            "{}/organizations/{}/invites",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/organizations/{organization_id}/invites`
    pub async fn invite_user_to_organization<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
        body: &'a types::CreateOrganizationInviteInput,
    ) -> Result<ResponseValue<()>, Error<types::InviteUserToOrganizationResponse>> {
        let url = format!(
            "{}/organizations/{}/invites",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/organizations/{organization_id}/overview/projects`
    pub async fn get_organization_project_overview<'a>(
        &'a self,
        organization_id: &'a str,
        authorization: &'a str,
    ) -> Result<
        ResponseValue<types::OrganizationProjectOverview>,
        Error<types::GetOrganizationProjectOverviewResponse>,
    > {
        let url = format!(
            "{}/organizations/{}/overview/projects",
            self.baseurl,
            encode_path(&organization_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/projects`
    pub async fn create_project<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::CreateProjectInput,
    ) -> Result<ResponseValue<types::Project>, Error<types::CreateProjectResponse>> {
        let url = format!("{}/projects", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/projects/{project_id}`
    pub async fn get_project_by_id<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::Project>, Error<types::GetProjectByIdResponse>> {
        let url = format!(
            "{}/projects/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/projects/{project_id}`
    pub async fn update_project_by_id<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
        body: &'a types::UpdateProjectInput,
    ) -> Result<ResponseValue<types::Project>, Error<types::UpdateProjectByIdResponse>> {
        let url = format!(
            "{}/projects/{}",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/projects/{project_id}/translations`
    pub async fn get_project_translations<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<Vec<types::Translation>>, Error<types::GetProjectTranslationsResponse>>
    {
        let url = format!(
            "{}/projects/{}/translations",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/projects/{project_id}/translations`
    pub async fn create_translation<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
        body: &'a types::CreateTranslationInput,
    ) -> Result<ResponseValue<types::Translation>, Error<types::CreateTranslationResponse>> {
        let url = format!(
            "{}/projects/{}/translations",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/projects/{project_id}/pull`
    pub async fn pull_project<'a>(
        &'a self,
        project_id: &'a str,
        flat: &'a str,
        x_api_key: &'a str,
    ) -> Result<ResponseValue<Vec<types::ExportProjectOutput>>, Error<()>> {
        let url = format!(
            "{}/projects/{}/pull",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        query.push(("flat", flat.to_string()));
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("x-api-key", HeaderValue::try_from(x_api_key)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/projects/{project_id}/push`
    pub async fn push_locales_to_project<'a>(
        &'a self,
        project_id: &'a str,
        x_api_key: &'a str,
        body: &'a types::PushLocaleInput,
    ) -> Result<ResponseValue<types::PushLocaleResult>, Error<()>> {
        let url = format!(
            "{}/projects/{}/push",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("x-api-key", HeaderValue::try_from(x_api_key)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/projects/{project_id}/webhooks`
    pub async fn get_project_webhooks<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProjectWebhook>>, Error<types::GetProjectWebhooksResponse>>
    {
        let url = format!(
            "{}/projects/{}/webhooks",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/projects/{project_id}/webhooks`
    pub async fn create_project_webhook<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
        body: &'a types::CreateProjectWebhookInput,
    ) -> Result<ResponseValue<types::ProjectWebhook>, Error<types::CreateProjectWebhookResponse>>
    {
        let url = format!(
            "{}/projects/{}/webhooks",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/projects/{project_id}/metrics`
    pub async fn get_project_metrics<'a>(
        &'a self,
        project_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::ProjectMetrics>, Error<types::GetProjectMetricsResponse>> {
        let url = format!(
            "{}/projects/{}/metrics",
            self.baseurl,
            encode_path(&project_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/translations/{translation_id}`
    pub async fn get_translation_by_id<'a>(
        &'a self,
        translation_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::Translation>, Error<types::GetTranslationByIdResponse>> {
        let url = format!(
            "{}/translations/{}",
            self.baseurl,
            encode_path(&translation_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/translations/{translation_id}`
    pub async fn update_translation_by_id<'a>(
        &'a self,
        translation_id: &'a str,
        authorization: &'a str,
        body: &'a types::UpdateTranslationInput,
    ) -> Result<ResponseValue<types::Translation>, Error<types::UpdateTranslationByIdResponse>>
    {
        let url = format!(
            "{}/translations/{}",
            self.baseurl,
            encode_path(&translation_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/translations/{translation_id}`
    pub async fn delete_translation_by_id<'a>(
        &'a self,
        translation_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::DeleteTranslationByIdResponse>> {
        let url = format!(
            "{}/translations/{}",
            self.baseurl,
            encode_path(&translation_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PATCH` request to `/translations/{translation_id}`
    pub async fn update_translation_locale<'a>(
        &'a self,
        translation_id: &'a str,
        authorization: &'a str,
        body: &'a types::UpdateTranslationLocaleInput,
    ) -> Result<ResponseValue<types::Translation>, Error<types::UpdateTranslationLocaleResponse>>
    {
        let url = format!(
            "{}/translations/{}",
            self.baseurl,
            encode_path(&translation_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PATCH` request to `/translations/{translation_id}/ai`
    pub async fn translate_using_ai<'a>(
        &'a self,
        translation_id: &'a str,
        authorization: &'a str,
        body: &'a types::TranslateAiInput,
    ) -> Result<ResponseValue<Vec<types::TranslateAiResult>>, Error<types::TranslateUsingAiResponse>>
    {
        let url = format!(
            "{}/translations/{}/ai",
            self.baseurl,
            encode_path(&translation_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .patch(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/personal-access-tokens`
    pub async fn get_personal_access_tokens<'a>(
        &'a self,
        authorization: &'a str,
    ) -> Result<
        ResponseValue<Vec<types::PersonalAccessToken>>,
        Error<types::GetPersonalAccessTokensResponse>,
    > {
        let url = format!("{}/personal-access-tokens", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/personal-access-tokens`
    pub async fn create_personal_access_token<'a>(
        &'a self,
        authorization: &'a str,
        body: &'a types::CreatePersonalAccessTokenInput,
    ) -> Result<
        ResponseValue<types::PersonalAccessToken>,
        Error<types::CreatePersonalAccessTokenResponse>,
    > {
        let url = format!("{}/personal-access-tokens", self.baseurl,);
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/personal-access-tokens/{key_id}`
    pub async fn invalidate_personal_access_token_by_id<'a>(
        &'a self,
        key_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::InvalidatePersonalAccessTokenByIdResponse>> {
        let url = format!(
            "{}/personal-access-tokens/{}",
            self.baseurl,
            encode_path(&key_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/webhooks/project/{project_webhook_id}`
    pub async fn get_project_webhook_by_id<'a>(
        &'a self,
        project_webhook_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<types::ProjectWebhook>, Error<types::GetProjectWebhookByIdResponse>>
    {
        let url = format!(
            "{}/webhooks/project/{}",
            self.baseurl,
            encode_path(&project_webhook_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/webhooks/project/{project_webhook_id}`
    pub async fn update_project_webhook_by_id<'a>(
        &'a self,
        project_webhook_id: &'a str,
        authorization: &'a str,
        body: &'a types::UpdateProjectWebhookInput,
    ) -> Result<ResponseValue<types::ProjectWebhook>, Error<types::UpdateProjectWebhookByIdResponse>>
    {
        let url = format!(
            "{}/webhooks/project/{}",
            self.baseurl,
            encode_path(&project_webhook_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/webhooks/project/{project_webhook_id}`
    pub async fn delete_project_webhook_by_id<'a>(
        &'a self,
        project_webhook_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::DeleteProjectWebhookByIdResponse>> {
        let url = format!(
            "{}/webhooks/project/{}",
            self.baseurl,
            encode_path(&project_webhook_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/auth/login/email`
    pub async fn login_with_email<'a>(
        &'a self,
        body: &'a types::EmailLoginInput,
    ) -> Result<ResponseValue<types::AuthTokens>, Error<types::LoginWithEmailResponse>> {
        let url = format!("{}/auth/login/email", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/auth/register/email`
    pub async fn register_with_email<'a>(
        &'a self,
        body: &'a types::EmailRegisterInput,
    ) -> Result<ResponseValue<types::AuthTokens>, Error<types::RegisterWithEmailResponse>> {
        let url = format!("{}/auth/register/email", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            409u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/auth/refresh`
    pub async fn refresh_auth_tokens<'a>(
        &'a self,
        body: &'a types::RefreshAuthTokensInput,
    ) -> Result<ResponseValue<types::AuthTokens>, Error<types::RefreshAuthTokensResponse>> {
        let url = format!("{}/auth/refresh", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            418u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/organization-invites/{invite_id}`
    pub async fn delete_organization_invite_by_id<'a>(
        &'a self,
        invite_id: &'a str,
        authorization: &'a str,
    ) -> Result<ResponseValue<()>, Error<types::DeleteOrganizationInviteByIdResponse>> {
        let url = format!(
            "{}/organization-invites/{}",
            self.baseurl,
            encode_path(&invite_id.to_string()),
        );
        let mut header_map = HeaderMap::with_capacity(1usize);
        header_map.append("authorization", HeaderValue::try_from(authorization)?);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .headers(header_map)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}

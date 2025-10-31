use std::path::Path;
use sword::core::injectable;

use google_calendar3::{
    CalendarHub as GCalendarHub,
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key},
};

use crate::config::GoogleCalendarConfig;

#[injectable(provider)]
pub struct CalendarService {
    inner: GCalendarHub<HttpsConnector<HttpConnector>>,
    calendar_id: String,
}

impl CalendarService {
    pub async fn new(config: &GoogleCalendarConfig) -> Self {
        let secret =
            read_service_account_key(Path::new(&config.service_account_path))
                .await
                .unwrap();

        let auth = ServiceAccountAuthenticator::builder(secret)
            .build()
            .await
            .unwrap();

        let client = Client::builder(TokioExecutor::new()).build(
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

        CalendarService {
            inner: GCalendarHub::new(client, auth),
            calendar_id: config.calendar_id.clone(),
        }
    }

    pub fn hub(&self) -> &GCalendarHub<HttpsConnector<HttpConnector>> {
        &self.inner
    }

    pub fn id(&self) -> &str {
        &self.calendar_id
    }
}

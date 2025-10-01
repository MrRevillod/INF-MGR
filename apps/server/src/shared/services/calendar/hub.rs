use shaku::{Component, Interface};
use std::path::Path;

use google_calendar3::{
    CalendarHub as GCalendarHub,
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key},
};

#[derive(Component)]
#[shaku(interface = CalendarService)]
pub struct CalendarHub {
    inner: GCalendarHub<HttpsConnector<HttpConnector>>,
    calendar_id: String,
}

pub trait CalendarService: Interface {
    fn hub(&self) -> &GCalendarHub<HttpsConnector<HttpConnector>>;
    fn id(&self) -> &str;
}

impl CalendarService for CalendarHub {
    fn hub(&self) -> &GCalendarHub<HttpsConnector<HttpConnector>> {
        &self.inner
    }

    fn id(&self) -> &str {
        &self.calendar_id
    }
}

impl CalendarHub {
    pub async fn new(secret_path: &str, calendar_id: &str) -> Self {
        println!("Path: {}", secret_path);
        let secret = read_service_account_key(Path::new(secret_path))
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

        CalendarHub {
            inner: GCalendarHub::new(client, auth),
            calendar_id: calendar_id.to_string(),
        }
    }
}

impl From<CalendarHub> for CalendarHubParameters {
    fn from(hub: CalendarHub) -> Self {
        CalendarHubParameters {
            inner: hub.inner,
            calendar_id: hub.calendar_id,
        }
    }
}

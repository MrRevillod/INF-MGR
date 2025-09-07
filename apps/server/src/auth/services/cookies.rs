use sword::prelude::*;
use time::Duration;

pub struct CookieBuilder {
    cookie: Cookie<'static>,
}

impl CookieBuilder {
    pub fn new((name, value): (&'static str, String)) -> Self {
        let cookie = Cookie::build((name, value))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .build();

        Self { cookie }
    }

    pub fn max_age(mut self, exp_ms: usize) -> Self {
        self.cookie.set_max_age(Duration::milliseconds(exp_ms as i64));
        self
    }

    pub fn build(self) -> Cookie<'static> {
        self.cookie
    }
}

use sword::prelude::*;
use time::Duration;

pub struct CookieBuilder {
    cookie: Cookie<'static>,
}

impl CookieBuilder {
    pub fn new(name: &'static str, value: String) -> Self {
        let cookie = Cookie::build((name, value))
            .path("/")
            .http_only(false)
            .same_site(SameSite::Lax)
            .build();

        Self { cookie }
    }

    pub fn max_age(mut self, exp_ms: usize) -> Self {
        self.cookie
            .set_max_age(Duration::milliseconds(exp_ms as i64));
        self
    }

    pub fn build(self) -> Cookie<'static> {
        self.cookie
    }
}

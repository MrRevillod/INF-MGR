use tera::Context;

#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub tera_ctx: Context,
}

pub type RawContext = Vec<(&'static str, String)>;

impl TemplateContext {
    pub fn new() -> Self {
        let mut data = Context::new();

        data.insert("color:uct_blue", &"#00487C".to_string());
        data.insert("color:uct_yellow", &"#F2B705".to_string());
        data.insert("color:uct_white", &"#FFFFFF".to_string());
        data.insert("color:uct_black", &"#000000".to_string());

        let public_url = std::env::var("PUBLIC_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        data.insert("public_url", &public_url);

        let secretary_email = std::env::var("SECRETARY_EMAIL")
            .expect("SECRETARY_EMAIL must be set in the environment");

        data.insert("secretary_email", &secretary_email);

        TemplateContext { tera_ctx: data }
    }

    pub fn insert_ctx(&mut self, ctx: RawContext) {
        for (key, value) in ctx {
            self.tera_ctx.insert(key, &value);
        }
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

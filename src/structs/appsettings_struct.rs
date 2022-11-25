pub struct AppSettingsStruct<'a> {
    pub(crate) url_opendota: &'a str,
}

impl AppSettingsStruct<'_> {
    pub fn get_url_opendota(&self) -> &str {
        self.url_opendota
    }
}

use std::collections::HashMap;

pub fn read_config() -> HashMap<String, String>{
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();
    settings.try_into::<HashMap<String, String>>().unwrap()
}

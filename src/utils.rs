use ::std::env;

pub fn get_default_display_number() -> Result<u16, env::VarError> {
    env::var("DISPLAY")
        .map(|display_str| display_str.trim_left_matches(':').parse::<u16>().unwrap())
}

use ::std::env;

pub fn get_default_display_number() -> Result<u16, env::VarError> {
    env::var("DISPLAY")
        .map(|display_str| display_str.trim_left_matches(':').parse::<u16>().unwrap())
}

/// Function used to calculate pad for unused bytes.
///
/// See this for more:
/// https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html#Syntactic_Conventions_b
pub fn pad(e: usize) -> usize {
    ((4 - (e % 4)) % 4)
}

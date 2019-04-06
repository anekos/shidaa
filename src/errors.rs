
use failure::Fail;



pub type AppResultU = Result<(), AppError>;



#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Invalid color format: {}", 0)]
    Color(css_color_parser::ColorParseError),
    #[fail(display = "IO Error: {}", 0)]
    Io(std::io::Error),
    #[fail(display = "Invalid number: {}", 0)]
    NumberFormat(std::num::ParseIntError),
    #[fail(display = "PNG Error: {}", 0)]
    Png(apng_encoder::ApngError),
}


macro_rules! define_error {
    ($source:ty, $kind:ident) => {
        impl From<$source> for AppError {
            fn from(error: $source) -> AppError {
                AppError::$kind(error)
            }
        }
    }
}

define_error!(apng_encoder::ApngError, Png);
define_error!(std::io::Error, Io);
define_error!(std::num::ParseIntError, NumberFormat);
define_error!(css_color_parser::ColorParseError, Color);


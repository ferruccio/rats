use sdl2::{
    render::TextureValueError, video::WindowBuildError, IntegerOrSdlError,
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, VideoError>;

#[derive(Error, Debug)]
pub enum VideoError {
    #[error("SDL error: {0}")]
    Sdl(String),

    #[error("SDL window build error: {0}")]
    WindowBuildError(#[from] WindowBuildError),

    #[error("SDL error; {0}")]
    SdlError(#[from] IntegerOrSdlError),

    #[error("SDL texture value error: {0}")]
    TextureValueError(#[from] TextureValueError),
}

pub fn sdl_error(error: String) -> VideoError {
    VideoError::Sdl(error)
}

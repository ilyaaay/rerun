mod decoder;

use parking_lot::Mutex;
use std::sync::Arc;

use re_video::VideoLoadError;

use crate::{resource_managers::GpuTexture2D, RenderContext};

#[derive(thiserror::Error, Debug)]
pub enum VideoError {
    #[error(transparent)]
    Load(#[from] VideoLoadError),

    #[error(transparent)]
    Init(#[from] DecodingError),
}

/// Error that can occur during frame decoding.
// TODO(jan, andreas): These errors are for the most part specific to the web decoder right now.
#[derive(thiserror::Error, Debug)]
pub enum DecodingError {
    // TODO(#7298): Native support.
    #[error("Video playback not yet available in the native viewer. Try the web viewer instead.")]
    NoNativeSupport,

    #[error("Failed to create VideoDecoder: {0}")]
    DecoderSetupFailure(String),

    #[error("Video seems to be empty, no segments have beem found.")]
    EmptyVideo,

    #[error("The current segment is empty.")]
    EmptySegment,

    #[error("Failed to reset the decoder: {0}")]
    ResetFailure(String),

    #[error("Failed to configure the video decoder: {0}")]
    ConfigureFailure(String),

    #[error("The timestamp passed was negative.")]
    NegativeTimestamp,
}

/// Information about the status of a frame decoding.
pub enum FrameDecodingResult {
    /// The requested frame got decoded and is ready to be used.
    Ready(GpuTexture2D),

    /// The returned texture is from a previous frame or a placeholder, the decoder is still decoding the requested frame.
    Pending(GpuTexture2D),

    /// The decoder encountered an error and was not able to produce a texture for the requested timestamp.
    /// The returned texture is either a placeholder or the last successfully decoded texture.
    Error(DecodingError),
}

/// Video data + decoder(s).
///
/// Supports asynchronously decoding video into GPU textures via [`Video::frame_at`].
pub struct Video {
    data: Arc<re_video::VideoData>,

    // TODO(#7420): Support several tracks of video decoders.
    // TODO(andreas): Create lazily.
    decoder: Mutex<decoder::VideoDecoder>,
}

impl Video {
    /// Loads a video from the given data.
    ///
    /// Currently supports the following media types:
    /// - `video/mp4`
    pub fn load(
        render_context: &RenderContext,
        data: &[u8],
        media_type: Option<&str>,
    ) -> Result<Self, VideoError> {
        let data = Arc::new(re_video::VideoData::load_from_bytes(data, media_type)?);
        let decoder = Mutex::new(decoder::VideoDecoder::new(render_context, data.clone())?);

        Ok(Self { data, decoder })
    }

    /// The video data
    #[inline]
    pub fn data(&self) -> &Arc<re_video::VideoData> {
        &self.data
    }

    /// Natural width of the video.
    #[inline]
    pub fn width(&self) -> u32 {
        self.data.width()
    }

    /// Natural height of the video.
    #[inline]
    pub fn height(&self) -> u32 {
        self.data.height()
    }

    /// Returns a texture with the latest frame at the given timestamp.
    ///
    /// If the timestamp is negative, a zeroed texture is returned.
    ///
    /// This API is _asynchronous_, meaning that the decoder may not yet have decoded the frame
    /// at the given timestamp. If the frame is not yet available, the returned texture will be
    /// empty.
    pub fn frame_at(&self, timestamp_s: f64) -> FrameDecodingResult {
        re_tracing::profile_function!();
        self.decoder.lock().frame_at(timestamp_s)
    }
}

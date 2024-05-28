use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Title {
    pub id: usize,
    pub name: String,
    pub chapter_count: i8,
    pub duration: u32,
    pub disk_size: String,
    pub disk_size_bytes: i64,
    pub source_file_name: String,
    pub segments_count: i8,
    pub segments_map: String,
    pub output_file_name: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i32,
    pub video_stream: VideoStream,
    pub audio_streams: Vec<AudioStream>,
    pub subtitle_streams: Vec<SubtitleStream>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct VideoStream {
    pub stream_type: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub video_size: String,
    pub video_aspect_ratio: String,
    pub video_frame_rate: String,
    pub stream_flags: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i8,
    pub mkv_flags: String,
    pub output_conversion_type: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct AudioStream {
    pub stream_type: String,
    pub name: String,
    pub lang_code: String,
    pub lang_name: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub bitrate: String,
    pub audio_channels_count: i8,
    pub audio_sample_rate: i32,
    pub audio_sample_size: i8,
    pub stream_flags: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i8,
    pub mkv_flags: String,
    pub mkv_flags_text: String,
    pub audio_channel_layout_name: String,
    pub output_conversion_type: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct SubtitleStream {
    pub stream_type: String,
    pub lang_code: String,
    pub lang_name: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub stream_flags: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i8,
    pub mkv_flags: String,
    pub mkv_flags_text: String,
    pub output_conversion_type: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Disc {
    pub disc_type: String,
    pub name: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub volume_name: String,
    pub order_weight: i8,
    pub titles: Vec<Title>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Device {
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub path: String,
}

#[derive(Debug)]
pub struct VideoStream {
    pub stream_type: String,
    pub stream_index: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub video_size: String,
    pub video_aspect_ratio: String,
    pub video_frame_rate: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: String,
    pub output_format: String,
    pub output_format_description: String,
    pub output_codec_short: String,
    pub output_conversion_type: String,
    pub output_audio_sample_rate: String,
    pub output_audio_sample_size: String,
    pub output_audio_channels_count: String,
    pub output_audio_channel_layout_name: String,
    pub output_audio_channel_layout: String,
    pub output_audio_mix_description: String,
}

#[derive(Debug)]
pub struct Title {
    pub name: String,
    pub chapter_count: String,
    pub duration: String,
    pub disk_size: String,
    pub disk_size_bytes: String,
    pub source_file_name: String,
    pub segments_count: String,
    pub segments_map: String,
    pub output_file_name: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: String,
    pub video_stream: VideoStream,
}

#[derive(Debug, Default)]
pub struct Disc {
    pub disc_type: String,
    pub name: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub volume_name: String,
    pub order_weight: String,
    pub titles: Vec<Title>,
}

/*

    "unknown",                      // 0
    "type",                         // 1
    "name",                         // 2
    "langCode",                     // 3
    "langName",                     // 4
    "codecId",                      // 5
    "codecShort",                   // 6
    "codecLong",                    // 7
    "chapterCount",                 // 8
    "duration",                     // 9
    "diskSize",                     // 10
    "diskSizeBytes",                // 11
    "streamTypeExtension",          // 12
    "bitrate",                      // 13
    "audioChannelsCount",           // 14
    "angleInfo",                    // 15
    "sourceFileName",               // 16
    "audioSampleRate",              // 17
    "audioSampleSize",              // 18
    "videoSize",                    // 19
    "videoAspectRatio",             // 20
    "videoFrameRate",               // 21
    "streamFlags",                  // 22
    "dateTime",                     // 23
    "originalTitleId",              // 24
    "segmentsCount",                // 25
    "segmentsMap",                  // 26
    "outputFileName",               // 27
    "metadataLanguageCode",         // 28
    "metadataLanguageName",         // 29
    "treeInfo",                     // 30
    "panelTitle",                   // 31
    "volumename",                   // 32
    "orderWeight",                  // 33
    "outputFormat",                 // 34
    "outputFormatDescription",      // 35
    "seamlessInfo",                 // 36
    "panelText",                    // 37
    "mkvFlags",                     // 38
    "mkvFlagsText",                 // 39
    "audioChannelLayoutName",       // 40
    "outputCodecShort",             // 41
    "outputConversionType",         // 42
    "outputAudioSampleRate",        // 43
    "outputAudioSampleSize",        // 44
    "outputAudioChannelsCount",     // 45
    "outputAudioChannelLayoutName", // 46
    "outputAudioChannelLayout",     // 47
    "outputAudioMixDescription",    // 48
    "comment",                      // 49
    "offsetSequenceId",             // 50

SINFO:1,0,  1,      type,                   6201,"Video"
SINFO:1,0,  5,      codecId,                0,"V_MPEG4/ISO/AVC"
SINFO:1,0,  6,      codecShort,             0,"Mpeg4"
SINFO:1,0,  7,      codecLong,              0,"Mpeg4 AVC High@L4.1"
SINFO:1,0,  19,     videoSize,              0,"1920x1080"
SINFO:1,0,  20,     videoAspectRatio,       0,"16:9"
SINFO:1,0,  21,     videoFrameRate,         0,"23.976 (24000/1001)"
SINFO:1,0,  22,     streamFlags,            0,"0"
SINFO:1,0,  28,     metadataLanguageCode,   0,"eng"
SINFO:1,0,  29,     metadataLanguageName,   0,"English"
SINFO:1,0,  30,     treeInfo,               0,"Mpeg4 AVC High@L4.1"
SINFO:1,0,  31,     panelTitle,             6121,"<b>Track information</b><br>"
SINFO:1,0,  33,     orderWeight,            0,"0"
SINFO:1,0,  38,     mkvFlags,               0,""
SINFO:1,0,  42,     outputConversionType,   5088,"( Lossless conversion )"

SINFO:1,1,  2,      name,   5091,"Stereo"
SINFO:1,1,  3,      langCode,   0,"eng"
SINFO:1,1,  4,      langName,   0,"English"
SINFO:1,1,  13,     bitrate,   0,"256 Kb/s"
SINFO:1,1,  14,     audioChannelsCount,   0,"2"
SINFO:1,1,  17,     _,   0,"48000"
SINFO:1,1,  39,     _,   0,"Default"
SINFO:1,1,  40,     _,   0,"stereo"

*/

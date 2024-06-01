use models::Profile;

pub mod models;

pub fn get_encoding_profiles() -> Vec<Profile> {
    vec![Profile {
        label: "Live Action 1080p".to_string(),
        name: "h264_1080p_medium_22crf_opus_5.1_384kbps.json".to_string(),
    }]
}

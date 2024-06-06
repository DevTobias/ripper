use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub label: String,
    pub file_name: String,
    pub preset_name: String,
}

/// Loads encoding profiles from a given base directory path.
///
/// This function reads an `index.json` file located in the specified base path,
/// parses it to retrieve a list of encoding profiles, and updates each profile's
/// `file_name` field to include the full path.
///
/// # Arguments
///
/// * `profile_base_path` - A string slice that holds the path to the directory
///                         containing the `index.json` file and the encoding profiles.
///
/// # Returns
///
/// This function returns a `Result` containing either:
/// * A vector of `Profile` structs if the operation is successful.
/// * An error if the file cannot be read or the JSON cannot be parsed.
///
/// # Errors
///
/// This function will return an error if:
/// * The `index.json` file cannot be read.
/// * The JSON content of the `index.json` file cannot be parsed into a vector of `Profile` structs.
///
/// # Example
///
/// ```
/// use your_crate::get_encoding_profiles;
///
/// let profiles = get_encoding_profiles("/path/to/profiles").expect("Failed to get encoding profiles");
/// for profile in profiles {
///     println!("{:?}", profile);
/// }
/// ```
pub fn get_encoding_profiles(profile_base_path: &str) -> Result<Vec<Profile>> {
    let profile_index_file = Path::new(profile_base_path).join("index.json");
    let contents = fs::read_to_string(profile_index_file).context("Should have been able to read the file")?;

    let profiles: Vec<Profile> = serde_json::from_str(&contents).context("Failed to parse json")?;

    let profiles = profiles
        .iter()
        .map(|profile| {
            let mut profile = profile.clone();
            profile.file_name = Path::new(profile_base_path).join(&profile.file_name).to_str().unwrap().to_string();
            profile
        })
        .collect();

    Ok(profiles)
}

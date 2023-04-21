use pretty_assertions::{assert_eq};

use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct AudioClip {
    pub slug: String,
    pub file: Option<String>,
    pub delay_ms: Option<i32>,
    pub base_volume_multiplier: Option<f32>,
    pub normalize_volume: Option<bool>,
    pub mix: Option<Vec<AudioClip>>,
}

#[derive(Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct AudioConfig {
    pub slug: String,
    pub output_file: String,
    pub clips: Vec<AudioClip>,
}

pub fn parse(json: &str) -> Result<AudioConfig> {
    let config: AudioConfig = serde_json::from_str(json)
        .expect("Failed to parse config");

    Ok(config)
}

#[test]
pub fn parse_config_test() {
    let config_json = r#"{
        "slug": "example",
        "output_file": "example.mp3",
        "clips": [
            {
                "slug": "intro",
                "file": "intro.mp3",
                "base_volume_multiplier": 0.8
            },
            {
                "slug": "hosts",
                "delay_ms": 15000,
                "normalize_volume": true,
                "mix": [
                    {
                        "slug": "caleb",
                        "file": "caleb.mp3"
                    },
                    {
                        "slug": "daniel",
                        "file": "daniel.mp3",
                        "base_volume_multiplier": 1.3
                    }
                ]
            }
        ]
    }"#;

    let config: AudioConfig = parse(config_json).expect("Couldn't Parse Config");

    assert_eq!(
        config,
        AudioConfig {
            slug: "example".to_string(),
            output_file: "example.mp3".to_string(),
            clips: vec![
                AudioClip {
                    slug: "intro".to_string(),
                    file: Some("intro.mp3".to_string()),
                    delay_ms: None,
                    base_volume_multiplier: Some(0.8),
                    normalize_volume: None,
                    mix: None,
                },
                AudioClip {
                    slug: "hosts".to_string(),
                    file: None,
                    delay_ms: Some(15000),
                    base_volume_multiplier: None,
                    normalize_volume: Some(true),
                    mix: Some(vec![
                        AudioClip {
                            slug: "caleb".to_string(),
                            file: Some("caleb.mp3".to_string()),
                            delay_ms: None,
                            base_volume_multiplier: None,
                            normalize_volume: None,
                            mix: None,
                        },
                        AudioClip {
                            slug: "daniel".to_string(),
                            file: Some("daniel.mp3".to_string()),
                            delay_ms: None,
                            base_volume_multiplier: Some(1.3),
                            normalize_volume: None,
                            mix: None,
                        },
                    ]),
                },
            ],
        }
    );
}
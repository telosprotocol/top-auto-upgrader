use chrono::{DateTime, Utc};
use json::JsonValue;
use std::str::FromStr;

struct ReleaseInfo {
    tag_name: String, // todo use version struct
    published_at: DateTime<Utc>,
    assets: Vec<ReleaseAsset>,
    body: String,
}

struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

impl ReleaseInfo {
    fn new_from_json_object(json: &JsonValue) -> Option<Self> {
        if let JsonValue::Object(obj) = json {
            let tag_name = obj.get("tag_name")?.as_str()?.into();
            let published_at_str = obj.get("published_at")?.as_str()?;
            let published_at = DateTime::<Utc>::from_str(published_at_str).ok()?;
            let assets = ReleaseAsset::new_from_json_array(obj.get("assets")?)?;
            let body = obj.get("body")?.as_str()?.into();
            Some(ReleaseInfo {
                tag_name,
                published_at,
                assets,
                body,
            })
        } else {
            None
        }
    }
}

impl ReleaseAsset {
    fn new_from_json_array(json: &JsonValue) -> Option<Vec<Self>> {
        if let JsonValue::Array(vec_json_obj) = json {
            Some(
                vec_json_obj
                    .into_iter()
                    .map_while(|asset_json_value_object| {
                        ReleaseAsset::new_from_json_object(asset_json_value_object)
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    fn new_from_json_object(json: &JsonValue) -> Option<Self> {
        if let JsonValue::Object(obj) = json {
            let name = obj.get("name")?.as_str()?.into();
            let browser_download_url = obj.get("browser_download_url")?.as_str()?.into();
            Some(ReleaseAsset {
                name,
                browser_download_url,
            })
        } else {
            None
        }
    }
}

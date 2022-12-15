use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ReleaseInfoSourceType {
    TelosGithub,
    TelosWebApi,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuConfigJson {
    release_api: String,
    release_info_source_type: ReleaseInfoSourceType,
}

#[cfg(test)]
mod test {
    use super::{AuConfigJson, ReleaseInfoSourceType};

    #[test]
    fn test_au_config() {
        let c = AuConfigJson {
            release_api: String::from("api.github.com/xxx"),
            release_info_source_type: ReleaseInfoSourceType::TelosGithub,
        };
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            String::from(
                r#"{"release_api":"api.github.com/xxx","release_info_source_type":"TelosGithub"}"#
            )
        );

        let from_str = String::from(
            r#"{"release_api":"api.github.com/xxx","release_info_source_type":"TelosGithub"}"#,
        );
        let to_c: AuConfigJson = serde_json::from_str(&from_str).unwrap();
        assert_eq!(to_c.release_api, c.release_api);
        assert_eq!(to_c.release_info_source_type, c.release_info_source_type);
    }
}

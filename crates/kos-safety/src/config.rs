// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Jun-young Han <jyhan@katech.re.kr>
// KATECH SDV Platform Research Center

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SafetyConfig {
    #[serde(default)]
    pub never_reclaim: Vec<String>,
    #[serde(default)]
    pub never_cpu_fallback: bool,
    #[serde(default)]
    pub max_swap_duration_ms: Option<u64>,
    #[serde(default)]
    pub rollback_timeout_ms: Option<u64>,
    #[serde(default)]
    pub require_integrity_check: Option<bool>,
    #[serde(default)]
    pub require_core_binding: Option<bool>,
    #[serde(default)]
    pub max_cores_per_model: Option<u32>,
    #[serde(default)]
    pub enforce_zone_isolation: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_defaults() {
        let yaml = "{}";
        let config: SafetyConfig = serde_yaml::from_str(yaml).unwrap();
        assert!(config.never_reclaim.is_empty());
        assert!(!config.never_cpu_fallback);
        assert!(config.max_swap_duration_ms.is_none());
    }

    #[test]
    fn deserialize_full() {
        let yaml = r#"
never_reclaim: [adas_npu]
never_cpu_fallback: true
max_swap_duration_ms: 500
rollback_timeout_ms: 1000
"#;
        let config: SafetyConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.never_reclaim, vec!["adas_npu"]);
        assert!(config.never_cpu_fallback);
        assert_eq!(config.max_swap_duration_ms, Some(500));
        assert_eq!(config.rollback_timeout_ms, Some(1000));
    }
}

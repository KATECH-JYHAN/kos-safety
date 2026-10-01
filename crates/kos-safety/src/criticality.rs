// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Jun-young Han <jyhan@katech.re.kr>
// KATECH SDV Platform Research Center

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Criticality {
    NonCritical,
    Critical,
    SafetyCritical,
}

impl PartialOrd for Criticality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Criticality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}

impl Criticality {
    fn rank(self) -> u8 {
        match self {
            Self::NonCritical => 0,
            Self::Critical => 1,
            Self::SafetyCritical => 2,
        }
    }

    pub fn to_asil(self) -> crate::AsilLevel {
        match self {
            Self::NonCritical => crate::AsilLevel::QM,
            Self::Critical => crate::AsilLevel::AsilB,
            Self::SafetyCritical => crate::AsilLevel::AsilD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(Criticality::NonCritical < Criticality::Critical);
        assert!(Criticality::Critical < Criticality::SafetyCritical);
    }

    #[test]
    fn equality() {
        assert_eq!(Criticality::NonCritical, Criticality::NonCritical);
    }

    #[test]
    fn serde_roundtrip() {
        let yaml = serde_yaml::to_string(&Criticality::SafetyCritical).unwrap();
        assert!(yaml.contains("safety_critical"));
    }

    #[test]
    fn to_asil_mapping() {
        assert_eq!(Criticality::NonCritical.to_asil(), crate::AsilLevel::QM);
        assert_eq!(Criticality::SafetyCritical.to_asil(), crate::AsilLevel::AsilD);
    }
}

// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Jun-young Han <jyhan@katech.re.kr>
// KATECH SDV Platform Research Center

use crate::AsilLevel;
use crate::Criticality;

pub fn can_access(from: AsilLevel, to: AsilLevel, write: bool) -> bool {
    if from >= to {
        true
    } else {
        !write
    }
}

pub fn can_reclaim_zone(criticality: Criticality, reclaimable: bool) -> bool {
    if criticality == Criticality::SafetyCritical {
        false
    } else {
        reclaimable
    }
}

pub fn validate_criticality_reclaimable(
    criticality: Criticality,
    reclaimable: bool,
) -> Result<(), String> {
    if criticality == Criticality::SafetyCritical && reclaimable {
        Err("safety_critical zone must not be reclaimable".into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_same_level() {
        assert!(can_access(AsilLevel::AsilB, AsilLevel::AsilB, true));
        assert!(can_access(AsilLevel::AsilB, AsilLevel::AsilB, false));
    }

    #[test]
    fn access_higher_to_lower() {
        assert!(can_access(AsilLevel::AsilD, AsilLevel::QM, true));
        assert!(can_access(AsilLevel::AsilD, AsilLevel::QM, false));
    }

    #[test]
    fn access_lower_to_higher_read_ok() {
        assert!(can_access(AsilLevel::QM, AsilLevel::AsilD, false));
    }

    #[test]
    fn access_lower_to_higher_write_denied() {
        assert!(!can_access(AsilLevel::QM, AsilLevel::AsilD, true));
    }

    #[test]
    fn reclaim_safety_critical_denied() {
        assert!(!can_reclaim_zone(Criticality::SafetyCritical, true));
        assert!(!can_reclaim_zone(Criticality::SafetyCritical, false));
    }

    #[test]
    fn reclaim_non_critical_depends_on_flag() {
        assert!(can_reclaim_zone(Criticality::NonCritical, true));
        assert!(!can_reclaim_zone(Criticality::NonCritical, false));
    }

    #[test]
    fn validate_safety_critical_reclaimable_fails() {
        assert!(validate_criticality_reclaimable(Criticality::SafetyCritical, true).is_err());
    }

    #[test]
    fn validate_non_critical_reclaimable_ok() {
        assert!(validate_criticality_reclaimable(Criticality::NonCritical, true).is_ok());
    }
}

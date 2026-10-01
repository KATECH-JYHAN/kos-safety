// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Jun-young Han <jyhan@katech.re.kr>
// KATECH SDV Platform Research Center

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AsilLevel {
    QM,
    AsilA,
    AsilB,
    AsilC,
    AsilD,
}

impl AsilLevel {
    pub fn can_write_to(self, target: Self) -> bool {
        self >= target
    }

    pub fn can_read_from(self, _target: Self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(AsilLevel::QM < AsilLevel::AsilA);
        assert!(AsilLevel::AsilA < AsilLevel::AsilB);
        assert!(AsilLevel::AsilB < AsilLevel::AsilC);
        assert!(AsilLevel::AsilC < AsilLevel::AsilD);
    }

    #[test]
    fn qm_cannot_write_to_d() {
        assert!(!AsilLevel::QM.can_write_to(AsilLevel::AsilD));
    }

    #[test]
    fn d_can_write_to_qm() {
        assert!(AsilLevel::AsilD.can_write_to(AsilLevel::QM));
    }

    #[test]
    fn same_level_write_ok() {
        assert!(AsilLevel::AsilB.can_write_to(AsilLevel::AsilB));
    }

    #[test]
    fn read_always_ok() {
        assert!(AsilLevel::QM.can_read_from(AsilLevel::AsilD));
        assert!(AsilLevel::AsilD.can_read_from(AsilLevel::QM));
    }
}

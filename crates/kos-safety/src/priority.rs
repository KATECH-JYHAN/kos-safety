// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Jun-young Han <jyhan@katech.re.kr>
// KATECH SDV Platform Research Center

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Priority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

impl Priority {
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Self::Critical,
            "high" => Self::High,
            "low" => Self::Low,
            _ => Self::Normal,
        }
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Critical),
            1 => Some(Self::High),
            2 => Some(Self::Normal),
            3 => Some(Self::Low),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(Priority::Critical < Priority::High);
        assert!(Priority::High < Priority::Normal);
        assert!(Priority::Normal < Priority::Low);
    }

    #[test]
    fn from_str_lossy_critical() {
        assert_eq!(Priority::from_str_lossy("critical"), Priority::Critical);
        assert_eq!(Priority::from_str_lossy("CRITICAL"), Priority::Critical);
    }

    #[test]
    fn from_str_lossy_default_normal() {
        assert_eq!(Priority::from_str_lossy("unknown"), Priority::Normal);
    }

    #[test]
    fn from_u8_valid() {
        assert_eq!(Priority::from_u8(0), Some(Priority::Critical));
        assert_eq!(Priority::from_u8(3), Some(Priority::Low));
    }

    #[test]
    fn from_u8_invalid() {
        assert_eq!(Priority::from_u8(4), None);
    }

    #[test]
    fn repr_u8() {
        assert_eq!(Priority::Critical as u8, 0);
        assert_eq!(Priority::Low as u8, 3);
    }
}

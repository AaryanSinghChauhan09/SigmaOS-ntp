// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ntp: ntp component model.
//!
//! Real implementation: ntp_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

/// NTP client module.
pub mod sigmantp;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtpItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NtpError {
    EmptyNtpItemName,
    DuplicateNtpItem(String),
    EmptyEventSubject,
}

impl fmt::Display for NtpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNtpItemName => write!(f, "ntp_item name must not be empty"),
            Self::DuplicateNtpItem(id) => write!(f, "duplicate ntp_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NtpError {}

pub struct Ntp {
    pub name: String,
    ntp_items: BTreeMap<String, NtpItem>,
    events: BTreeMap<String, Event>,
}

impl Ntp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ntp_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_ntp_item(&mut self, item: NtpItem) -> Result<(), NtpError> {
        if item.name.trim().is_empty() {
            return Err(NtpError::EmptyNtpItemName);
        }
        let id = item.id.clone();
        if self.ntp_items.contains_key(&id) {
            return Err(NtpError::DuplicateNtpItem(id.clone()));
        }
        self.ntp_items.insert(id, item);
        Ok(())
    }

    pub fn ntp_item_count(&self) -> usize {
        self.ntp_items.len()
    }

    pub fn get_ntp_item(&self, id: &str) -> Option<&NtpItem> {
        self.ntp_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NtpError> {
        if event.subject.trim().is_empty() {
            return Err(NtpError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ntp_item(id: &str, name: &str) -> NtpItem {
        NtpItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Ntp::new("Ntp");
        assert_eq!(m.name, "Ntp");
        assert_eq!(m.ntp_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_ntp_item() {
        let mut m = Ntp::new("Ntp");
        m.add_ntp_item(sample_ntp_item("a1", "Alpha")).unwrap();
        assert_eq!(m.ntp_item_count(), 1);
        assert!(m.get_ntp_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Ntp::new("Ntp");
        assert!(m.add_ntp_item(sample_ntp_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Ntp::new("Ntp");
        m.add_ntp_item(sample_ntp_item("x", "One")).unwrap();
        assert!(m.add_ntp_item(sample_ntp_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Ntp::new("Ntp");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

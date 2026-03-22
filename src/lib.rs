use serde::{Deserialize, Serialize};

/// Current state of the bridge daemon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeState {
    pub connected_devices: Vec<String>,
    pub clipboard_synced: bool,
    pub notifications_mirrored: bool,
}

/// A single clipboard history entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub content: String,
    pub source: String,
    pub timestamp: String,
}

/// Merge local and remote clipboard histories, deduplicating by content.
///
/// When duplicates exist (same content), the entry with the later timestamp
/// is kept. The result is sorted by timestamp in ascending order.
#[must_use]
pub fn merge_clipboard_history(
    local: &[ClipboardEntry],
    remote: &[ClipboardEntry],
) -> Vec<ClipboardEntry> {
    use std::collections::HashMap;

    let mut by_content: HashMap<String, ClipboardEntry> = HashMap::new();

    for entry in local.iter().chain(remote.iter()) {
        by_content
            .entry(entry.content.clone())
            .and_modify(|existing| {
                if entry.timestamp > existing.timestamp {
                    *existing = entry.clone();
                }
            })
            .or_insert_with(|| entry.clone());
    }

    let mut merged: Vec<ClipboardEntry> = by_content.into_values().collect();
    merged.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_empty_histories() {
        let result = merge_clipboard_history(&[], &[]);
        assert!(result.is_empty());
    }

    #[test]
    fn merge_local_only() {
        let local = vec![ClipboardEntry {
            content: "hello".to_string(),
            source: "mac".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }];
        let result = merge_clipboard_history(&local, &[]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, "hello");
    }

    #[test]
    fn merge_remote_only() {
        let remote = vec![ClipboardEntry {
            content: "world".to_string(),
            source: "android".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }];
        let result = merge_clipboard_history(&[], &remote);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].content, "world");
    }

    #[test]
    fn merge_deduplicates_by_content() {
        let local = vec![ClipboardEntry {
            content: "same".to_string(),
            source: "mac".to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }];
        let remote = vec![ClipboardEntry {
            content: "same".to_string(),
            source: "android".to_string(),
            timestamp: "2026-01-02T00:00:00Z".to_string(),
        }];
        let result = merge_clipboard_history(&local, &remote);
        assert_eq!(result.len(), 1);
        // The remote entry has a later timestamp
        assert_eq!(result[0].source, "android");
    }

    #[test]
    fn merge_orders_by_timestamp() {
        let local = vec![ClipboardEntry {
            content: "later".to_string(),
            source: "mac".to_string(),
            timestamp: "2026-01-03T00:00:00Z".to_string(),
        }];
        let remote = vec![
            ClipboardEntry {
                content: "first".to_string(),
                source: "android".to_string(),
                timestamp: "2026-01-01T00:00:00Z".to_string(),
            },
            ClipboardEntry {
                content: "middle".to_string(),
                source: "android".to_string(),
                timestamp: "2026-01-02T00:00:00Z".to_string(),
            },
        ];
        let result = merge_clipboard_history(&local, &remote);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].content, "first");
        assert_eq!(result[1].content, "middle");
        assert_eq!(result[2].content, "later");
    }
}

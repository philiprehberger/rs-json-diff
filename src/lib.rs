use serde_json::Value;
use std::fmt;

/// The type of change detected between two JSON values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

impl fmt::Display for ChangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChangeType::Added => write!(f, "added"),
            ChangeType::Removed => write!(f, "removed"),
            ChangeType::Modified => write!(f, "modified"),
        }
    }
}

/// A single change between two JSON values.
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    pub path: String,
    pub change_type: ChangeType,
    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.change_type {
            ChangeType::Added => {
                write!(f, "+ {}: {}", self.path, self.new_value.as_ref().unwrap())
            }
            ChangeType::Removed => {
                write!(f, "- {}: {}", self.path, self.old_value.as_ref().unwrap())
            }
            ChangeType::Modified => {
                write!(
                    f,
                    "~ {}: {} -> {}",
                    self.path,
                    self.old_value.as_ref().unwrap(),
                    self.new_value.as_ref().unwrap()
                )
            }
        }
    }
}

/// Summary counts of changes by type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffSummary {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
}

/// Compute a structural diff between two JSON values.
///
/// Returns a list of changes with path tracking. Paths use dot notation
/// for object keys and bracket notation for array indices.
pub fn diff(a: &Value, b: &Value) -> Vec<Change> {
    let mut changes = Vec::new();
    diff_values(a, b, "", &mut changes);
    changes
}

/// Summarize a list of changes by counting each type.
pub fn diff_summary(changes: &[Change]) -> DiffSummary {
    let mut added = 0;
    let mut removed = 0;
    let mut modified = 0;

    for change in changes {
        match change.change_type {
            ChangeType::Added => added += 1,
            ChangeType::Removed => removed += 1,
            ChangeType::Modified => modified += 1,
        }
    }

    DiffSummary {
        added,
        removed,
        modified,
    }
}

fn build_path(prefix: &str, key: &str) -> String {
    if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{}.{}", prefix, key)
    }
}

fn build_array_path(prefix: &str, index: usize) -> String {
    format!("{}[{}]", prefix, index)
}

fn diff_values(a: &Value, b: &Value, path: &str, changes: &mut Vec<Change>) {
    match (a, b) {
        (Value::Object(map_a), Value::Object(map_b)) => {
            // Keys in a
            for (key, val_a) in map_a {
                let child_path = build_path(path, key);
                match map_b.get(key) {
                    Some(val_b) => diff_values(val_a, val_b, &child_path, changes),
                    None => changes.push(Change {
                        path: child_path,
                        change_type: ChangeType::Removed,
                        old_value: Some(val_a.clone()),
                        new_value: None,
                    }),
                }
            }
            // Keys only in b
            for (key, val_b) in map_b {
                if !map_a.contains_key(key) {
                    let child_path = build_path(path, key);
                    changes.push(Change {
                        path: child_path,
                        change_type: ChangeType::Added,
                        old_value: None,
                        new_value: Some(val_b.clone()),
                    });
                }
            }
        }
        (Value::Array(arr_a), Value::Array(arr_b)) => {
            let max_len = arr_a.len().max(arr_b.len());
            for i in 0..max_len {
                let child_path = build_array_path(path, i);
                match (arr_a.get(i), arr_b.get(i)) {
                    (Some(val_a), Some(val_b)) => {
                        diff_values(val_a, val_b, &child_path, changes);
                    }
                    (Some(val_a), None) => {
                        changes.push(Change {
                            path: child_path,
                            change_type: ChangeType::Removed,
                            old_value: Some(val_a.clone()),
                            new_value: None,
                        });
                    }
                    (None, Some(val_b)) => {
                        changes.push(Change {
                            path: child_path,
                            change_type: ChangeType::Added,
                            old_value: None,
                            new_value: Some(val_b.clone()),
                        });
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        _ => {
            if a != b {
                changes.push(Change {
                    path: path.to_string(),
                    change_type: ChangeType::Modified,
                    old_value: Some(a.clone()),
                    new_value: Some(b.clone()),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn no_changes() {
        let a = json!({"name": "Alice", "age": 30});
        let b = json!({"name": "Alice", "age": 30});
        let changes = diff(&a, &b);
        assert!(changes.is_empty());
    }

    #[test]
    fn added_key() {
        let a = json!({"name": "Alice"});
        let b = json!({"name": "Alice", "age": 30});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "age");
        assert_eq!(changes[0].change_type, ChangeType::Added);
        assert_eq!(changes[0].new_value, Some(json!(30)));
        assert_eq!(changes[0].old_value, None);
    }

    #[test]
    fn removed_key() {
        let a = json!({"name": "Alice", "age": 30});
        let b = json!({"name": "Alice"});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "age");
        assert_eq!(changes[0].change_type, ChangeType::Removed);
        assert_eq!(changes[0].old_value, Some(json!(30)));
        assert_eq!(changes[0].new_value, None);
    }

    #[test]
    fn modified_value() {
        let a = json!({"name": "Alice"});
        let b = json!({"name": "Bob"});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "name");
        assert_eq!(changes[0].change_type, ChangeType::Modified);
        assert_eq!(changes[0].old_value, Some(json!("Alice")));
        assert_eq!(changes[0].new_value, Some(json!("Bob")));
    }

    #[test]
    fn nested_diff() {
        let a = json!({"user": {"name": "Alice", "age": 30}});
        let b = json!({"user": {"name": "Alice", "age": 31}});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "user.age");
        assert_eq!(changes[0].change_type, ChangeType::Modified);
    }

    #[test]
    fn array_diff() {
        let a = json!({"tags": ["rust", "dev"]});
        let b = json!({"tags": ["rust", "senior"]});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "tags[1]");
        assert_eq!(changes[0].change_type, ChangeType::Modified);
    }

    #[test]
    fn array_added() {
        let a = json!({"items": [1, 2]});
        let b = json!({"items": [1, 2, 3]});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "items[2]");
        assert_eq!(changes[0].change_type, ChangeType::Added);
        assert_eq!(changes[0].new_value, Some(json!(3)));
    }

    #[test]
    fn array_removed() {
        let a = json!({"items": [1, 2, 3]});
        let b = json!({"items": [1, 2]});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "items[2]");
        assert_eq!(changes[0].change_type, ChangeType::Removed);
        assert_eq!(changes[0].old_value, Some(json!(3)));
    }

    #[test]
    fn type_change() {
        let a = json!({"value": "hello"});
        let b = json!({"value": 42});
        let changes = diff(&a, &b);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].path, "value");
        assert_eq!(changes[0].change_type, ChangeType::Modified);
        assert_eq!(changes[0].old_value, Some(json!("hello")));
        assert_eq!(changes[0].new_value, Some(json!(42)));
    }

    #[test]
    fn summary() {
        let a = json!({"a": 1, "b": 2, "c": 3});
        let b = json!({"a": 1, "b": 5, "d": 4});
        let changes = diff(&a, &b);
        let summary = diff_summary(&changes);
        assert_eq!(summary.added, 1);
        assert_eq!(summary.removed, 1);
        assert_eq!(summary.modified, 1);
    }

    #[test]
    fn display_change_type() {
        assert_eq!(format!("{}", ChangeType::Added), "added");
        assert_eq!(format!("{}", ChangeType::Removed), "removed");
        assert_eq!(format!("{}", ChangeType::Modified), "modified");
    }

    #[test]
    fn display_change() {
        let added = Change {
            path: "name".to_string(),
            change_type: ChangeType::Added,
            old_value: None,
            new_value: Some(json!("Alice")),
        };
        assert_eq!(format!("{}", added), "+ name: \"Alice\"");

        let removed = Change {
            path: "age".to_string(),
            change_type: ChangeType::Removed,
            old_value: Some(json!(30)),
            new_value: None,
        };
        assert_eq!(format!("{}", removed), "- age: 30");

        let modified = Change {
            path: "score".to_string(),
            change_type: ChangeType::Modified,
            old_value: Some(json!(10)),
            new_value: Some(json!(20)),
        };
        assert_eq!(format!("{}", modified), "~ score: 10 -> 20");
    }

    #[test]
    fn complex_nested() {
        let a = json!({
            "users": [
                {"name": "Alice", "roles": ["admin"]},
                {"name": "Bob", "roles": ["user"]}
            ],
            "config": {
                "debug": false,
                "version": "1.0"
            }
        });
        let b = json!({
            "users": [
                {"name": "Alice", "roles": ["admin", "super"]},
                {"name": "Charlie", "roles": ["user"]}
            ],
            "config": {
                "debug": true,
                "version": "1.0",
                "env": "prod"
            }
        });
        let changes = diff(&a, &b);
        let summary = diff_summary(&changes);

        // roles[1] added, name modified (Bob->Charlie), debug modified, env added
        assert!(summary.added >= 2);
        assert!(summary.modified >= 2);

        let paths: Vec<&str> = changes.iter().map(|c| c.path.as_str()).collect();
        assert!(paths.contains(&"users[0].roles[1]"));
        assert!(paths.contains(&"users[1].name"));
        assert!(paths.contains(&"config.debug"));
        assert!(paths.contains(&"config.env"));
    }
}

//! Git log integration: commit frequency, contributors, and lines changed.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use chrono::Datelike;

/// Git history summary for a repository.
#[derive(Debug, Clone, Default)]
pub(crate) struct GitHistory {
    /// Total number of commits.
    pub(crate) total_commits: usize,
    /// Unique contributor emails.
    pub(crate) contributors: Vec<String>,
    /// Per-week activity stats (last 12 weeks).
    pub(crate) weekly_activity: Vec<WeekStats>,
}

/// Activity stats for a single week.
#[derive(Debug, Clone)]
pub(crate) struct WeekStats {
    /// Week identifier (e.g., "2024-W03").
    pub(crate) week: String,
    /// Number of commits.
    pub(crate) commits: usize,
    /// Lines added.
    pub(crate) lines_added: usize,
    /// Lines removed.
    pub(crate) lines_removed: usize,
}

/// Collect git history for a repository.
///
/// Returns `None` if git is not available or the directory is not a git repo.
pub(crate) fn collect_git_history(dir: &Path) -> Option<GitHistory> {
    let log_output = Command::new("git")
        .args(["log", "--format=%ae|%aI", "--since=12 weeks ago"])
        .current_dir(dir)
        .output()
        .ok()?;

    if !log_output.status.success() {
        return None;
    }

    let log_text = String::from_utf8_lossy(&log_output.stdout);

    // Count total commits (all time)
    let total_commits = count_total_commits(dir);

    // Parse recent commits for contributors and weekly breakdown
    let mut contributors_set = std::collections::BTreeSet::new();
    let mut week_commits: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();

    for line in log_text.lines() {
        let parts: Vec<&str> = line.splitn(2, '|').collect();
        if parts.len() < 2 {
            continue;
        }
        let email = parts[0].trim();
        let date_str = parts[1].trim();

        contributors_set.insert(email.to_string());

        if let Some(week) = iso_week_from_date(date_str) {
            let entry = week_commits.entry(week).or_insert((0, 0, 0));
            entry.0 += 1;
        }
    }

    // Get numstat for lines changed per week
    if let Some(numstat) = collect_numstat(dir) {
        for (week, added, removed) in numstat {
            let entry = week_commits.entry(week).or_insert((0, 0, 0));
            entry.1 += added;
            entry.2 += removed;
        }
    }

    let weekly_activity: Vec<WeekStats> = week_commits
        .into_iter()
        .map(|(week, (commits, added, removed))| WeekStats {
            week,
            commits,
            lines_added: added,
            lines_removed: removed,
        })
        .collect();

    Some(GitHistory {
        total_commits,
        contributors: contributors_set.into_iter().collect(),
        weekly_activity,
    })
}

/// Count total commits in the repository.
fn count_total_commits(dir: &Path) -> usize {
    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(dir)
        .output()
        .ok();

    output
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8_lossy(&o.stdout)
                    .trim()
                    .parse::<usize>()
                    .ok()
            } else {
                None
            }
        })
        .unwrap_or(0)
}

/// Collect numstat data grouped by week.
fn collect_numstat(dir: &Path) -> Option<Vec<(String, usize, usize)>> {
    let output = Command::new("git")
        .args(["log", "--format=%aI", "--numstat", "--since=12 weeks ago"])
        .current_dir(dir)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8_lossy(&output.stdout);
    let mut result: Vec<(String, usize, usize)> = Vec::new();
    let mut current_week = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Date lines start with a year (4 digits)
        if trimmed.len() >= 10 && trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            if let Some(week) = iso_week_from_date(trimmed) {
                current_week = week;
            }
            continue;
        }

        // Numstat lines: "added\tremoved\tfilename"
        if !current_week.is_empty() {
            let parts: Vec<&str> = trimmed.split('\t').collect();
            if parts.len() >= 2 {
                let added = parts[0].parse::<usize>().unwrap_or(0);
                let removed = parts[1].parse::<usize>().unwrap_or(0);
                result.push((current_week.clone(), added, removed));
            }
        }
    }

    Some(result)
}

/// Extract ISO week string from a date string.
///
/// Accepts ISO 8601 format like "2024-03-20T14:30:00+00:00".
fn iso_week_from_date(date_str: &str) -> Option<String> {
    // Parse just the date part (first 10 chars)
    let date_part = date_str.get(..10)?;
    let parsed = chrono::NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()?;
    let iso_week = parsed.iso_week();
    Some(format!("{}-W{:02}", iso_week.year(), iso_week.week()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso_week_from_valid_date() {
        let week = iso_week_from_date("2024-03-20T14:30:00+00:00");
        assert!(week.is_some());
        assert!(week.as_ref().is_some_and(|w| w.starts_with("2024-W")));
    }

    #[test]
    fn iso_week_from_date_only() {
        let week = iso_week_from_date("2024-01-01");
        assert!(week.is_some());
        assert!(week.as_ref().is_some_and(|w| w.starts_with("2024-W")));
    }

    #[test]
    fn iso_week_from_invalid_date() {
        assert!(iso_week_from_date("not-a-date").is_none());
        assert!(iso_week_from_date("").is_none());
    }

    #[test]
    fn collect_git_history_works_in_git_repo() {
        // Run in the current repo (which is a git repo)
        let history = collect_git_history(Path::new("."));
        // Should succeed since we're in a git repo
        assert!(history.is_some());
        let h = history.unwrap();
        assert!(h.total_commits > 0);
        assert!(!h.contributors.is_empty());
    }

    #[test]
    fn collect_git_history_returns_none_for_non_git() {
        let dir = tempfile::TempDir::new().unwrap();
        let history = collect_git_history(dir.path());
        assert!(history.is_none());
    }
}

# Git history for inactive repositories

## Problem

`GitHistory.total_commits` is all-time, but contributor emails are currently
derived from the 12-week activity window. A valid repository with no recent
commit therefore reports commits while reporting zero contributors, and its
own verification gate becomes date-dependent.

## Requirements

1. Contributor emails represent all reachable commits at `HEAD`.
2. Weekly commit and line-change activity remains limited to the most recent
   12 weeks.
3. A repository whose only commit is older than 12 weeks reports one total
   commit, its contributor, and no recent weekly activity.
4. Non-Git directories continue to return no Git history.
5. The existing format, Clippy, and test gates remain green.

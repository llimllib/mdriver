---
name: pr-transcript
description: Export the current Pi session transcript to an HTML file for inclusion in a pull request. Use when the user is ready to submit a PR and wants to include an AI session transcript.
---

# PR Transcript Export

This skill exports the current Pi session to an HTML file for PR documentation.

## Workflow

The PR number isn't known until the PR is created, so follow this order:

1. **Commit and push** the code changes
2. **Export transcript** with a temporary name (e.g., `transcripts/temp-<description>.html`)
3. **Commit and push** the transcript
4. **Create the PR** with `gh pr create` (now we have the PR number)
5. **Rename the transcript** to include the PR number (e.g., `transcripts/<pr-number>-<description>.html`)
6. **Commit and push** the rename
7. **Update the PR body** with `gh pr edit` to fix the transcript link

## Finding the Session File

```bash
# Session files are stored based on the working directory
SESSION_DIR="$HOME/.pi/agent/sessions/--$(pwd | tr '/' '-' | sed 's/^-//')--"

# Get the most recent session file
ls -t "$SESSION_DIR"/*.jsonl | head -1
```

## Exporting the Transcript

```bash
mkdir -p transcripts
pi --export <session_file> transcripts/temp-<description>.html
```

## Complete Example

User: "commit it and file a PR, including a transcript"

```bash
# 1. Commit and push code changes
git add -A && git commit -m "Add feature X" && git push origin my-branch

# 2. Find session and export transcript with temp name
SESSION_FILE=$(ls -t "$HOME/.pi/agent/sessions/--$(pwd | tr '/' '-' | sed 's/^-//')--"/*.jsonl | head -1)
mkdir -p transcripts
pi --export "$SESSION_FILE" transcripts/temp-feature-x.html

# 3. Commit and push transcript
git add transcripts/temp-feature-x.html
git commit -m "Add session transcript for PR"
git push origin my-branch

# 4. Create PR (captures the PR number)
gh pr create --title "Add feature X" --body "..." 
# Output: https://github.com/user/repo/pull/42

# 5. Rename transcript with PR number
mv transcripts/temp-feature-x.html transcripts/42-feature-x.html
git add -A && git commit -m "Rename transcript with PR number"
git push origin my-branch

# 6. Update PR body with correct transcript link
gh pr edit 42 --body "...
## Session Transcript
[View transcript](https://github.com/user/repo/blob/my-branch/transcripts/42-feature-x.html)
"
```

## Notes

- The session directory is named after the current working directory with slashes replaced by dashes
- Multiple session files may exist; use the most recent one (sorted by timestamp in filename)
- Create `transcripts/` directory if it doesn't exist
- The description should be kebab-case (lowercase with hyphens)
- Always include a "Session Transcript" section in the PR body with a link to the file

#!/usr/bin/env bash
set -euo pipefail

# Automated release workflow: bumps version, commits, and tags
# Usage: ./tools/release.sh
#
# GoReleaser handles changelog generation from git commits,
# so we no longer manually update CHANGELOG.md

# Extract current version from Cargo.toml
CURRENT=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT"
read -rp "Bump type (major/minor/patch): " BUMP

# Parse current version into components
MAJOR=$(echo "$CURRENT" | cut -d. -f1)
MINOR=$(echo "$CURRENT" | cut -d. -f2)
PATCH=$(echo "$CURRENT" | cut -d. -f3)

# Calculate new version based on bump type
case $BUMP in
	major) NEW_VERSION="$((MAJOR + 1)).0.0" ;;
	minor) NEW_VERSION="$MAJOR.$((MINOR + 1)).0" ;;
	patch) NEW_VERSION="$MAJOR.$MINOR.$((PATCH + 1))" ;;
	*) echo "Invalid bump type. Use major, minor, or patch."; exit 1 ;;
esac

echo "New version: $NEW_VERSION"
echo ""

# Show commits since last tag
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -n "$LATEST_TAG" ]; then
	echo "=== Commits since $LATEST_TAG ==="
	git log --pretty=format:"- %s" "$LATEST_TAG"..HEAD --no-merges
	echo ""
	echo ""
fi

# Update version in Cargo.toml
sed -i.bak "s/^version = \"$CURRENT\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Run checks before committing
echo "Running cargo fmt..."
cargo fmt

echo "Running cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running cargo test..."
cargo test

# Commit and tag
read -rp "Enter release message (or press enter for default): " MESSAGE
MESSAGE="${MESSAGE:-Release v$NEW_VERSION}"

git add Cargo.toml
git commit -m "chore: bump version to $NEW_VERSION"
git pull --rebase
git tag -a "v$NEW_VERSION" -m "$MESSAGE"
git push
git push --tags

echo ""
echo "✓ Tagged version v$NEW_VERSION"
echo "✓ GitHub Actions will now build and release via GoReleaser"

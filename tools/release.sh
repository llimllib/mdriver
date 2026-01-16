#!/usr/bin/env bash
set -euo pipefail

# Automated release workflow: bumps version, updates changelog, commits, and tags
# Usage: ./tools/release.sh
# Interactive prompts:
#   1. Bump type (major/minor/patch) - calculates new semantic version
#   2. Optional CHANGELOG.md edit
#   3. Release/tag message

# Extract current version from Cargo.toml and prompt for bump type
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

# Get the latest tag
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

# Get commit messages since last tag (or all commits if no tag exists)
if [ -n "$LATEST_TAG" ]; then
	COMMITS=$(git log --pretty=format:"- %s" "$LATEST_TAG"..HEAD --no-merges)
else
	COMMITS=$(git log --pretty=format:"- %s" --no-merges)
fi

if [ -z "$COMMITS" ]; then
	echo "No new commits since $LATEST_TAG"
	exit 1
fi

echo "=== Changes to be added to changelog ==="
echo "$COMMITS"
echo ""

# Insert commits into CHANGELOG.md after the [Unreleased] header
# Create a temp file with the new content
{
	sed -n '1,/^## \[Unreleased\]/p' CHANGELOG.md
	echo ""
	echo "$COMMITS"
	sed -n '/^## \[Unreleased\]/,${/^## \[Unreleased\]/d;p}' CHANGELOG.md
} > CHANGELOG.md.tmp
mv CHANGELOG.md.tmp CHANGELOG.md

# Update CHANGELOG.md: add new version header and update comparison links
DATE=$(date +%Y-%m-%d)
sed -i.bak "s/^## \[Unreleased\]/## [Unreleased]\n\n## [$NEW_VERSION] - $DATE/" CHANGELOG.md
sed -i.bak "s|\[Unreleased\]: .*/compare/v.*\.\.\.HEAD|[Unreleased]: https://github.com/llimllib/mdriver/compare/v$NEW_VERSION...HEAD\n[$NEW_VERSION]: https://github.com/llimllib/mdriver/compare/v$CURRENT...v$NEW_VERSION|" CHANGELOG.md
rm CHANGELOG.md.bak

# Update version in Cargo.toml
sed -i.bak "s/^version = \"$CURRENT\"/version = \"$NEW_VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Commit, tag, and push
read -rp "Enter release/tag message: " MESSAGE
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to $NEW_VERSION"
git pull
git tag -a "v$NEW_VERSION" -m "$MESSAGE" && git push && git push --tags

echo ""
echo "✓ Released version $NEW_VERSION"

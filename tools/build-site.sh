#!/bin/bash
set -euo pipefail

# Build the documentation site for GitHub Pages
# Outputs to _site/ directory

# Check for pandoc
if ! command -v pandoc &> /dev/null; then
    echo "Error: pandoc is required but not installed" >&2
    echo "Install with: brew install pandoc (macOS) or apt install pandoc (Linux)" >&2
    exit 1
fi

# Get the project root (where this script lives is scripts/, go up one level)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# GitHub icon SVG
GITHUB_ICON='<svg height="16" width="16" viewBox="0 0 16 16" style="vertical-align: text-bottom; margin-right: 4px;"><path fill="currentColor" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>'

# Clean and create output directory
rm -rf _site
mkdir -p _site/transcripts

# Build index.html from README
cat > _site/index.html << HTMLEOF
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>mdriver - streaming markdown viewer</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
      line-height: 1.6;
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem;
      color: #333;
    }
    pre {
      background: #f4f4f4;
      padding: 1rem;
      overflow-x: auto;
      border-radius: 4px;
    }
    code {
      background: #f4f4f4;
      padding: 0.2rem 0.4rem;
      border-radius: 3px;
    }
    pre code {
      background: none;
      padding: 0;
    }
    table {
      border-collapse: collapse;
      width: 100%;
      margin: 1rem 0;
    }
    th, td {
      border: 1px solid #ddd;
      padding: 0.5rem;
      text-align: left;
    }
    th {
      background: #f4f4f4;
    }
    a {
      color: #0366d6;
    }
    img {
      max-width: 100%;
      height: auto;
    }
    h1, h2, h3 {
      margin-top: 1.5rem;
    }
    nav {
      background: #f8f9fa;
      padding: 1rem;
      margin-bottom: 2rem;
      border-radius: 4px;
    }
    nav a {
      margin-right: 1rem;
    }
  </style>
</head>
<body>
  <nav>
    <a href="/">Home</a>
    <a href="./transcripts/">Transcripts</a>
    <a href="https://github.com/llimllib/mdriver">${GITHUB_ICON}GitHub</a>
  </nav>
HTMLEOF

pandoc README.md -f markdown -t html >> _site/index.html

echo '</body></html>' >> _site/index.html

# Copy docs (images, etc.)
if [ -d docs ] && [ "$(ls -A docs)" ]; then
    mkdir -p _site/docs
    cp -r docs/* _site/docs/
fi

# Copy transcripts
if [ -d transcripts ] && [ "$(ls -A transcripts)" ]; then
    cp -r transcripts/* _site/transcripts/
fi

# Create transcripts index
cat > _site/transcripts/index.html << HTMLEOF
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Transcripts - mdriver</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
      line-height: 1.6;
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem;
      color: #333;
    }
    a {
      color: #0366d6;
    }
    nav {
      background: #f8f9fa;
      padding: 1rem;
      margin-bottom: 2rem;
      border-radius: 4px;
    }
    nav a {
      margin-right: 1rem;
    }
    ul {
      padding-left: 1.5rem;
    }
    li {
      padding: 0.5rem 0;
      border-bottom: 1px solid #eee;
    }
    .pr-link {
      margin-left: 0.5rem;
      font-size: 0.9em;
    }
  </style>
</head>
<body>
  <nav>
    <a href="/">Home</a>
    <a href="/transcripts/">Transcripts</a>
    <a href="https://github.com/llimllib/mdriver">${GITHUB_ICON}GitHub</a>
  </nav>
  <h1>Development Transcripts</h1>
  <p>AI coding session transcripts for the mdriver project.</p>
  <ul>
HTMLEOF

# List all HTML files in transcripts
for f in transcripts/*.html; do
    if [ -f "$f" ]; then
        basename=$(basename "$f")
        # Extract PR number (everything before the first dash)
        pr_number=$(echo "$basename" | sed 's/-.*//')
        # Extract a readable name from the filename (everything after the PR number)
        name=$(echo "$basename" | sed 's/\.html$//' | sed 's/^[0-9]*-//' | sed 's/-/ /g')
        # Capitalize first letter
        name="$(echo "${name:0:1}" | tr '[:lower:]' '[:upper:]')${name:1}"
        echo "    <li><a href=\"$basename\">$name</a><a class=\"pr-link\" href=\"https://github.com/llimllib/mdriver/pull/$pr_number\">${GITHUB_ICON}PR #$pr_number</a></li>" >> _site/transcripts/index.html
    fi
done

cat >> _site/transcripts/index.html << 'HTMLEOF'
  </ul>
</body>
</html>
HTMLEOF

echo "Site built in _site/"

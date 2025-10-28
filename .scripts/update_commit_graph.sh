#!/bin/bash

echo "Running update_commit_graph.sh"

# This script calculates the commit activity for the last 7 days and updates the Readme.md file with a graph.

# Get the absolute path to the project root
PROJECT_ROOT=$(git rev-parse --show-toplevel)
README_PATH="$PROJECT_ROOT/Readme.md"

# Check the last commit message
LAST_COMMIT_MESSAGE=$(git log -1 --pretty=%B)
if [[ "$LAST_COMMIT_MESSAGE" == "docs: Update commit graph in Readme.md" ]]; then
  echo "Skipping graph update because the last commit was a graph update."
  exit 0
fi

# Initialize arrays to store dates and counts
days=()
counts=()

# Initialize counts for the last 7 days to 0
for i in {0..6}; do
  day=$(date -v-${i}d +%Y-%m-%d)
  days+=("$day")
  counts+=(0)
done

# Get commit dates for the last 7 days
commit_dates=$(git log --since="1 week ago" --pretty=format:%cs)

# Count commits for each day
for cdate in $commit_dates; do
  for i in "${!days[@]}"; do
    if [[ "${days[$i]}" == "$cdate" ]]; then
      counts[$i]=$((${counts[$i]} + 1))
      break
    fi
  done
done

# Generate the visualization
graph=""
for i in {0..6}; do
  day=${days[$i]}
  # macOS date command to get day name from date string
  day_name=$(date -j -f "%Y-%m-%d" "$day" +%a)
  count=${counts[$i]}
  bar=""
  if [[ $count -gt 0 ]]; then
    for ((j=0; j<$count; j++)); do
      bar+="█"
    done
    graph="${graph}${day_name} (${day}): ${count} commits ${bar}  "$'
'
  fi
done

# Update Readme.md
# Use a temporary file to avoid issues with in-place editing
TMP_FILE=$(mktemp)

export graph_for_awk="$graph"
awk '
  BEGIN {p=1; graph=ENVIRON["graph_for_awk"]}
  /<!-- START_COMMIT_GRAPH -->/ {print; print graph; p=0}
  /<!-- END_COMMIT_GRAPH -->/ {p=1}
  p {print}
' "$README_PATH" > "$TMP_FILE" && mv "$TMP_FILE" "$README_PATH"

echo "Attempting to commit readme changes"
# Stage and commit the changes
git add "$README_PATH"
git commit -m "docs: Update commit graph in Readme.md"
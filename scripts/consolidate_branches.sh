#!/bin/bash
# Script to consolidate all branches into master and delete them
# Usage: ./scripts/consolidate_branches.sh

set -e

echo "Branch Consolidation Script"
echo "============================"
echo ""

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Error: Not in a git repository"
    exit 1
fi

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
echo "Current branch: $CURRENT_BRANCH"

# Switch to master branch
echo "Switching to master branch..."
git checkout master || git checkout main || {
    echo "Error: Could not checkout master or main branch"
    exit 1
}

# Update master
echo "Pulling latest changes from master..."
git pull origin master || git pull origin main || true

# Get list of all branches except master/main
echo ""
echo "Fetching all branches..."
git fetch --all --prune

echo ""
echo "Branches to merge:"
BRANCHES=$(git branch -r | grep -v 'HEAD' | grep -v 'master' | grep -v 'main' | sed 's/origin\///' | sed 's/^[[:space:]]*//')

if [ -z "$BRANCHES" ]; then
    echo "No branches to merge."
    exit 0
fi

echo "$BRANCHES"
echo ""

# Merge each branch
for BRANCH in $BRANCHES; do
    echo "Processing branch: $BRANCH"
    
    # Try to merge the branch
    if git merge --no-ff "origin/$BRANCH" -m "Merge branch '$BRANCH' into master"; then
        echo "  ✓ Successfully merged $BRANCH"
    else
        echo "  ✗ Conflict merging $BRANCH - requires manual resolution"
        echo "  Aborting merge..."
        git merge --abort
        echo "  Please manually merge $BRANCH"
        continue
    fi
done

echo ""
echo "Merging complete. Pushing to master..."
git push origin master || git push origin main

echo ""
echo "Deleting merged branches..."
for BRANCH in $BRANCHES; do
    echo "Deleting remote branch: $BRANCH"
    git push origin --delete "$BRANCH" || echo "  Could not delete $BRANCH (may require permissions)"
done

echo ""
echo "Branch consolidation complete!"
echo ""
echo "Summary:"
echo "- All branches have been merged into master"
echo "- Remote branches have been deleted"
echo ""
echo "Note: You may need to manually delete local branches with:"
echo "  git branch -D <branch-name>"

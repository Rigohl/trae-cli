# Script to consolidate all branches into master and delete them
# Usage: .\scripts\consolidate_branches.ps1

$ErrorActionPreference = "Stop"

Write-Host "Branch Consolidation Script" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in a git repository
try {
    git rev-parse --git-dir | Out-Null
} catch {
    Write-Host "Error: Not in a git repository" -ForegroundColor Red
    exit 1
}

# Get current branch
$currentBranch = git branch --show-current
Write-Host "Current branch: $currentBranch"

# Switch to master branch
Write-Host "Switching to master branch..."
try {
    git checkout master
    $mainBranch = "master"
} catch {
    try {
        git checkout main
        $mainBranch = "main"
    } catch {
        Write-Host "Error: Could not checkout master or main branch" -ForegroundColor Red
        exit 1
    }
}

# Update master
Write-Host "Pulling latest changes from $mainBranch..."
try {
    git pull origin $mainBranch
} catch {
    Write-Host "Warning: Could not pull from origin" -ForegroundColor Yellow
}

# Get list of all branches except master/main
Write-Host ""
Write-Host "Fetching all branches..."
git fetch --all --prune

Write-Host ""
Write-Host "Branches to merge:" -ForegroundColor Cyan
$remoteBranches = git branch -r | Where-Object { 
    $_ -notmatch 'HEAD' -and 
    $_ -notmatch 'master' -and 
    $_ -notmatch 'main' 
} | ForEach-Object { 
    $_.Trim() -replace 'origin/', '' 
}

if ($remoteBranches.Count -eq 0) {
    Write-Host "No branches to merge." -ForegroundColor Yellow
    exit 0
}

$remoteBranches | ForEach-Object { Write-Host "  - $_" }
Write-Host ""

# Merge each branch
$mergedBranches = @()
$failedBranches = @()

foreach ($branch in $remoteBranches) {
    Write-Host "Processing branch: $branch" -ForegroundColor Yellow
    
    # Try to merge the branch
    try {
        git merge --no-ff "origin/$branch" -m "Merge branch '$branch' into $mainBranch"
        Write-Host "  ✓ Successfully merged $branch" -ForegroundColor Green
        $mergedBranches += $branch
    } catch {
        Write-Host "  ✗ Conflict merging $branch - requires manual resolution" -ForegroundColor Red
        Write-Host "  Aborting merge..."
        git merge --abort
        Write-Host "  Please manually merge $branch"
        $failedBranches += $branch
    }
}

Write-Host ""
Write-Host "Merging complete. Pushing to $mainBranch..."
try {
    git push origin $mainBranch
} catch {
    Write-Host "Error: Could not push to $mainBranch" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Deleting merged branches..." -ForegroundColor Cyan
foreach ($branch in $mergedBranches) {
    Write-Host "Deleting remote branch: $branch"
    try {
        git push origin --delete $branch
        Write-Host "  ✓ Deleted $branch" -ForegroundColor Green
    } catch {
        Write-Host "  Could not delete $branch (may require permissions)" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Branch consolidation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Summary:" -ForegroundColor Cyan
Write-Host "- Successfully merged $($mergedBranches.Count) branches into $mainBranch"
if ($failedBranches.Count -gt 0) {
    Write-Host "- Failed to merge $($failedBranches.Count) branches (conflicts):" -ForegroundColor Red
    $failedBranches | ForEach-Object { Write-Host "    - $_" -ForegroundColor Red }
}
Write-Host "- Remote branches have been deleted"
Write-Host ""
Write-Host "Note: You may need to manually delete local branches with:"
Write-Host "  git branch -D <branch-name>"

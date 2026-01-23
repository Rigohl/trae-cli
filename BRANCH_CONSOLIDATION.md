# Branch Consolidation Guide

This guide explains how to consolidate all branches into the main branch (master) and clean up the repository.

## Problem

When a repository accumulates many feature branches over time, it can become difficult to manage. This guide helps you merge all branches into the main branch and delete them.

## Scripts

Two scripts are provided for branch consolidation:

1. **`scripts/consolidate_branches.sh`** - For Linux/macOS users
2. **`scripts/consolidate_branches.ps1`** - For Windows users

## What These Scripts Do

1. **Checkout the main branch** (master or main)
2. **Pull latest changes** from the remote repository
3. **Fetch all remote branches**
4. **Merge each branch** into master using `--no-ff` (no fast-forward) to preserve history
5. **Push the merged changes** to the remote repository
6. **Delete all remote branches** that were successfully merged

## Prerequisites

- Git installed and configured
- Write access to the repository
- Permissions to delete remote branches

## Usage

### Linux/macOS

```bash
cd /path/to/trae-cli
./scripts/consolidate_branches.sh
```

### Windows (PowerShell)

```powershell
cd C:\path\to\trae-cli
.\scripts\consolidate_branches.ps1
```

## What to Expect

The script will:

1. Display the current branch
2. Switch to master/main branch
3. List all branches that will be merged
4. Merge each branch one by one
   - ✓ Success: Branch merged without conflicts
   - ✗ Conflict: Branch has conflicts and needs manual resolution
5. Push all merged changes to master
6. Delete all successfully merged remote branches
7. Show a summary of the operation

## Handling Merge Conflicts

If a branch has conflicts, the script will:

1. Abort the conflicting merge
2. Continue with other branches
3. Report which branches had conflicts

To manually resolve conflicts:

```bash
# Switch to master
git checkout master

# Merge the problematic branch
git merge origin/<branch-name>

# Resolve conflicts in your editor
# After resolving, stage the changes
git add .

# Complete the merge
git commit

# Push to remote
git push origin master

# Delete the branch
git push origin --delete <branch-name>
```

## Cleaning Up Local Branches

After running the consolidation script, you may have local branches that are no longer needed:

```bash
# List local branches
git branch

# Delete a local branch
git branch -D <branch-name>

# Delete all local branches except master
git branch | grep -v "master" | xargs git branch -D
```

## Safety Notes

- **Backup your repository** before running these scripts
- The scripts use `--no-ff` merges to preserve branch history
- Failed merges are aborted automatically - no changes are made
- You can always recover branches from the remote if needed (before deletion)

## Alternative: Manual Consolidation

If you prefer to merge branches manually:

```bash
# Switch to master
git checkout master

# Update master
git pull origin master

# Merge a specific branch
git merge --no-ff origin/<branch-name>

# Push changes
git push origin master

# Delete the remote branch
git push origin --delete <branch-name>

# Delete local branch
git branch -D <branch-name>
```

## Verifying the Consolidation

After running the script:

```bash
# Check remaining branches
git branch -r

# Verify all changes are in master
git log --graph --oneline --all

# Check the repository status
git status
```

## Troubleshooting

### "Permission denied" errors

You may need administrator/owner permissions to delete remote branches. Contact your repository administrator.

### "Could not checkout master"

Your repository might use `main` instead of `master`. The script tries both automatically.

### Script execution errors

On Linux/macOS, ensure the script is executable:
```bash
chmod +x scripts/consolidate_branches.sh
```

On Windows, you may need to enable script execution:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## Post-Consolidation

After consolidating all branches:

1. Update any documentation that references old branches
2. Update CI/CD pipelines if they reference specific branches
3. Notify team members about the consolidation
4. Consider implementing a branch naming/cleanup policy

## GitHub Actions Alternative

For automated branch cleanup, consider creating a GitHub Actions workflow that periodically identifies and suggests merging stale branches.

## Related Issues

This consolidation addresses the requirement: "Todo traelo a rama main a principal, despues elimina todas las ramas" (Bring everything to main/principal branch, then eliminate all branches).

# Setup Instructions

## Step 1: Create GitHub Repository

Since the GitHub CLI is not installed, create the repository manually:

1. Go to [GitHub](https://github.com/new)
2. Fill in the repository details:
   - **Repository name**: `claude-config`
   - **Description**: Claude Code configuration files synced across computers
   - **Visibility**: **Private**
   - **Do NOT** initialize with README, .gitignore, or license (we already have these)
3. Click "Create repository"

## Step 2: Push to GitHub

After creating the repository, GitHub will show you commands. Run these from the repository directory:

```bash
cd ~/Dropbox/GitHub/claude-config
git remote add origin git@github.com:YOUR_USERNAME/claude-config.git
git push -u origin main
```

Replace `YOUR_USERNAME` with your GitHub username.

If you use HTTPS instead of SSH:
```bash
git remote add origin https://github.com/YOUR_USERNAME/claude-config.git
git push -u origin main
```

## Step 3: Set Up Symlinks on This Computer

Now that the files are in the repo, set up the symlinks:

```bash
cd ~/Dropbox/GitHub/claude-config
./setup.sh
```

This will create a symlink from `~/.claude/settings.json` to the repo's `settings.json`.

## Step 4: Set Up on Your Second Computer

On your other computer:

1. Ensure Dropbox is installed and synced
2. Wait for the `~/Dropbox/GitHub/claude-config` folder to sync
3. Run the setup script:
   ```bash
   cd ~/Dropbox/GitHub/claude-config
   ./setup.sh
   ```

## Optional: Install GitHub CLI

To make future GitHub operations easier, install the GitHub CLI:

```bash
brew install gh
gh auth login
```

## Sharing with Coworkers

To share with coworkers:

1. Go to your repository on GitHub
2. Click "Settings" → "Collaborators"
3. Add their GitHub usernames
4. They can then clone:
   ```bash
   git clone git@github.com:YOUR_USERNAME/claude-config.git ~/claude-config
   cd ~/claude-config
   ./setup.sh
   ```

Note: If sharing, they should clone to a location that works for them (doesn't have to be in Dropbox).

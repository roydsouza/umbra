---
description: Session sync ritual for Project Umbra with upstream software checks
---

# Umbra Sync Ritual

This workflow ensures **Project Umbra** stays synchronized and that all agents (AntiGravity, Gemini Conductor, Claude Code) are aware of upstream software updates for Tor and Arti.

---

## 🚀 Session Start (Ritual)

Whenever an agent enters the `umbra` directory, they MUST perform these steps immediately:

// turbo
1. **Pull Latest Changes (Parent Repository):**
   ```bash
   cd ~/antigravity/umbra && git pull
   ```

2. **Check Upstream Software Updates (Submodules):**
   For each submodule (`tor` and `arti`), check if there are new commits upstream:
   
   // turbo
   - **Tor**:
     ```bash
     cd ~/antigravity/umbra/tor && git fetch origin && git log HEAD..origin/master --oneline
     ```
   
   // turbo
   - **Arti**:
     ```bash
     cd ~/antigravity/umbra/arti && git fetch origin && git log HEAD..origin/main --oneline
     ```
   
   > [!IMPORTANT]
   > If any new commits are found upstream, **Notify the User immediately** with a summary of the new changes before proceeding with other work.

3. **Read Context Files:**
   - **Read [SYNC_LOG.md](../SYNC_LOG.md)**: Reconcile internal state with the latest session results.
   - **Read [TASKS.md](../TASKS.md)**: Identify the current priority.

---

## 💾 Checkpoint (Session End)

When requested to "checkpoint" or end the session:

1. **Update [SYNC_LOG.md](../SYNC_LOG.md):**
   Append a new entry with the current date and detailed summary of changes.

2. **Commit and Push Submodule Changes (if any):**
   If you made changes *inside* `tor/` or `arti/` (rare), commit and push those first.

// turbo
3. **Stage and Commit Parent Repository:**
   ```bash
   cd ~/antigravity/umbra
   git add -A
   git commit -m "[Detailed Comment summarizing the engineering work]"
   ```

// turbo
4. **Push to GitHub:**
   ```bash
   git push
   ```

---

## 🛠️ Quick Reference

| Action | Command |
|--------|---------|
| Start Sync | `git pull && git submodule update --init` |
| Check Upstream | `git submodule update --remote --dry-run` |
| Push Changes | `git push` |

---
name: Umbra Operations
description: Operational rituals for the Umbra workspace, including Arti status checks.
---

# Umbra Operations Skill

This skill defines the operational rituals for the `umbra` context.

## 🟢 Entry Ritual (Automatic)

**Trigger**: When the user enters or opens `umbra`.

**Action**:
1. Run `cd arti && python3 scripts/check_status.py`
2. Report the output to the user.
3. If an update is available, ask: *"Would you like to upgrade to {latest}?"*

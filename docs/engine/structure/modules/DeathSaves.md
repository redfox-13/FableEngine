# DeathSaves Module

The **DeathSaves Module** tracks an [entity](../entities/README.md)’s status when they are at 0 HP and on the brink of death. This mechanic is used to determine whether the entity stabilizes and survives or succumbs and dies.

This system is composed of **two counters**:

- **Successes** – Count successful death saving throws
- **Failures** – Count failed death saving throws

When either counter reaches `3`, the outcome is resolved:

- **3 Successes**: The entity stabilizes (remains at 0 HP but is no longer dying)
- **3 Failures**: The entity dies permanently

## What it includes:

- **Successes** – Number of successful death saves.
- **Failures** – Number of failed death saves.
- **Keep Failures** _(optional)_ –  A toggle for whether failure points are kept after succeeding (some systems or house rules allow lingering consequences).

> This module is typically used when the entity reaches 0 HP and is not instantly killed.


***
![List of all modules](README.md#List%20of%20Modules)
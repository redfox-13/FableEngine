# DicePool Module

The **DicePool Module** is used by an [entity](../entities/README.md) to track how many [dice](../../world/mechanics/Dice.md) from a referenced **DiceSet** are currently available for use. This is ideal for systems like Hit Dice, limited-use magic, or resource-based mechanics.

## What it includes:

- **Linked DiceSet** – A reference to the [dice set mechanic](../../world/mechanics/DiceSet.md) definition (e.g. `2d6`).
- **Remaining Quantity** – How many dice are left on this pool.
- **Label** (*optional*) – A name or purpose for the pool (e.g. `"Hit Dice"`, `"Rage Uses"`, `"Healing Pool"`).

> Example: A character has a `4d8` Hit Dice pool, and has spent `2`, so they have `2d8` remaining.

By separating dice logic from entity tracking, this module stays lightweight while still supporting complex systems like short rest recovery, recharge effects, or pooled dice usage.


***
![List of all modules](README.md#List%20of%20Modules)
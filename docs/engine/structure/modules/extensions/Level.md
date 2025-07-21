# Level Extension
The **Level Extension** tracks experience points (XP) and manages level progression for an [entity](../../entities/README.md). It normally works as an extension of a [Stat Module](../Stat.md) (usually named `"Level"`) to represent the entity’s current experience.

This module handles XP accumulation and leveling logic, while the actual level value is stored in a separate module. This separation makes it flexible: you can still manually control or override the level if needed.
## What it includes:

- **Current XP** – The total experience the entity has earned.
- **Linked Module** – The name of the **Module** that represents the entity’s level (e.g. `"Level"`).
- **Auto Level Up** – A toggle that determines whether the module automatically updates a linked module when XP milestones are reached.
- **XP Table** _(optional)_ – A reference table that defines how much XP is needed for each level.
	- This is **required** if Auto Level Up is enabled.
	- Optional if levels are managed manually, but can still be used for reference.

> Note: When Auto Level Up is disabled, a **manual Level Up** is required to increment the level. This provides players or DMs control over leveling while still making it feel smooth and intentional.

This design lets you support both hands-on XP systems (where players level up manually) and fully automated leveling logic. This setup also allows the XP system to be entirely optional. If your game doesn't use XP, you could, for example, just use a static `"Level"` [stat](../Stat.md) with no **level extension** attached.

## Possible parent modules:
- [Stat Module](../Stat.md)

***
![List of all extensions](README.md#List%20of%20Extensions)
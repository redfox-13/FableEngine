# Grouping Module
The **Grouping Module** is a utility module that lets you bundle other modules together into a single named group. It doesn’t store new data or change the behaviour of the modules it contains — it simply provides a convenient way to organize and access related modules as a set.

Use it to group anything: stats, skills, combat traits, custom sets — whatever makes sense for you.

## What it includes:

- **Group Name** – A label for the group (e.g. `"Abilities"`, `"Skills"`, `"Combat Info"`).
- **Modules List** – A list of modules (e.g. `Health`, `AC`, `Speed`) grouped under this label.
- **Display Type** _(Optional)_ – Any UI or formatting hints for how the group should be shown (`grid`, `list`, `collapsible`, etc.). Defaults to `list`.

> Example: A `Grouping Module` labeled `"Ability Scores"` could include 6 Stat Modules for Strength, Dexterity, Constitution, Intelligence, Wisdom, and Charisma.

This module helps with clarity, organization, and visualisation without interfering with how the contained modules function individually.


***
![List of all modules](README.md#List%20of%20Modules)
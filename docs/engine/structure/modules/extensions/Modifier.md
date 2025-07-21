# Modifier Extension

The **Modifier Extension** enhances a **module** by either reading from it to produce a derived modifier, or by applying a bonus to it based on some rule or value.

This could be used for ability score modifiers, racial bonuses, feature or trait effects, or anything else that needs to dynamically adjust or derive from a value.

## What it includes:

- **Mode** – read, add or both
    - **read**: calculates a modifier *from* an existing module
    - **add**: adds a modifier _to_ an existing module
    - **both**: reads *from* a module and adds *to* another
- **Linked Module** – The name of the **Module** that represents the value being read from or modified (e.g. `"Dexterity"`)
- **Formula** _(only for "read")_ – Optional formula for calculating a modifier from a stat (defaults to `floor((value - 10) / 2)`)
- **Value** _(only for "add")_ – The number to be added to the linked module (e.g. `+1`, `-2`)

> Example A (read mode): `Dexterity = 18 → Mod = +4`  
> Example B (add mode): `+1 to Strength` from a trait → final stat = 17

## Possible parent modules:
- [Stat Module](../Stat.md)

***
![List of all extensions](README.md#List%20of%20Extensions)
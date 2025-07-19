# Extensions

## What are Extensions?

**Extensions** are optional add-ons that expand the functionality of existing [Modules](../README.md). Where **modules** define the core parts of an entity (like health, stats, or inventory), extensions **plug into those modules** to add deeper mechanics or optional systems.

They’re not standalone—they **depend on a specific module** to function, and enhance or react to that module’s behaviour.

Think of them as modular upgrades or rule add-ons.

### Why Use Extensions?

- **Customization**: Need a way to track Experience Points for Character Level? See a value as a modifier? Use an extension.
- **Minimalism by Default**: Keep base modules clean and focused. Only add complexity where it’s needed.
- **Flexibility**: Extensions allow you to layer on systems without rewriting core mechanics.

This keeps your simulator lean, while still offering depth when and where you want it.

### How Do Extensions Work?

An extension:
1. Hooks into one existing module.
2. Awaits for changes in the main module.
3. Creates or adds data in response to changes.

> Extensions follow the same modular philosophy as core modules: they’re opt-in, composable, and designed to interact cleanly with the rest of the system.

## List of Extensions
- [Level](Level.md)

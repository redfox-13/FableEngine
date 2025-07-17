# Entities

### What is an Entity?

An **Entity** is a core building block used to represent simple or complex characters in your game. It is composed of one or more [**modules**](#Entity%20Modularity), each responsible for defining specific traits, behaviours, or capabilities.

Think of an entity as a customizable container: the **modules** you include define what the entity _is_ and what it can _do_ and _have_.

### Entity Modularity

Entities are designed to be **modular**—meaning they are composed of multiple **modules**. Each **module** adds specific information or functionality to the entity, allowing for a semi-complete character customization.

For example, an **entity** might include:

- **Appearance**, **Description** – to define identity

- **Health**, **Skills**, **Spells** – to keep track of ongoing changeable information

- **Currency**, **Inventory** – to support interactions with the world

> See the [Overview of Modules](../modules/README.md) for more information and a complete list of available modules.
### Types of Entities

You’ll find some pages in this documentation detailing different types of **entities** you can create, such as:

- [Players](Players.md) – characters that players will roleplay during gameplay
  
- [Non-Playable](Non-Playable.md) – non-player characters managed by the DM
  
- [Companions](Companions.md) – companion characters that follow the players
  
- [Creatures](Creatures.md) – creatures controlled by the DM

### World Interactions

An **entity** can also interact with the world around it, using modules that have **external components** like:

- Items in an inventory (e.g. potions, tools)
  
- Spell books or magic wands for casting spells
  
- Weapons for combat
  
- Currency used in the world
  
- Mounts for the character
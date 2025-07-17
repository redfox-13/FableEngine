# Health Module
The **Health Module** represents a character’s life points. It tracks how much health an [entity](../entities/README.md) currently has, how much they can have at their best, and any extra temporary health they've gained through any means.

### What it includes:
- **Max Health** – The total health the character can have when fully healed.
- **Current Health** – The character's current HP. This number goes down as they take damage and can be restored with healing or other methods.
- **Temporary Health** – Extra HP that acts as a buffer. It’s not part of the max health and is usually gained from abilities or spells. When taking damage, temporary health is lost first.


***
![List of all modules](README.md#List%20of%20Modules)
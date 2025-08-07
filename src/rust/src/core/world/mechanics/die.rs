use rand::Rng;

pub enum Die {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
    Custom(CustomDie),
}

impl Die {
    pub fn sides(&self) -> u16 {
        match self {
            Die::D2 => 2,
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
            Die::Custom(die) => die.sides(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Die::D2 => "d2",
            Die::D4 => "d4",
            Die::D6 => "d6",
            Die::D8 => "d8",
            Die::D10 => "d10",
            Die::D12 => "d12",
            Die::D20 => "d20",
            Die::D100 => "d100",
            Die::Custom(die) => die.name(),
        }
    }

    pub fn roll(&self) -> u16 {
        rand::rng().random_range(1..=self.sides())
    }

    pub fn roll_multiple(&self, number_of_rolls: u16) -> Vec<u16> {
        (0..number_of_rolls.max(1)).map(|_| self.roll()).collect()
    }

    pub fn new_custom_with_name(sides: u16, name: String) -> Option<Self> {
        if sides % 2 == 1 || sides == 0 {
            ()
        }
        Some(Die::Custom(CustomDie::new(sides, name)))
    }

    pub fn new_custom(sides: u16) -> Option<Self> {
        Die::new_custom_with_name(sides, format!("d{}", sides))
    }
}

pub struct CustomDie {
    sides: u16,
    name: String,
}

impl CustomDie {
    fn new(sides: u16, name: String) -> Self {
        Self { sides: sides, name: name }
    }

    fn sides(&self) -> u16 {
        self.sides
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_sides_and_name() {
        let die: Die = Die::D10;
        assert_eq!(die.name(), "d10");
        assert_eq!(die.sides(), 10);
    }

    #[test]
    fn rolls_follow_bounds() {
        let die: Die = Die::D10;
        let mut roll: u16;
        for _ in 0..1000 {
            roll = die.roll_multiple(1)[0];
            assert!(
                (1..=10).contains(&roll),
                "Roll out of bounds [1, 10]: got {}",
                roll
            );
        }
    }

    #[test]
    fn can_get_sides_and_name_from_custom() {
        if let Some(die) = Die::new_custom(30) {
            assert_eq!(die.name(), "d30");
            assert_eq!(die.sides(), 30);
        } else {
            panic!("Failed to create custom die with 30 sides");
        }
    }

    #[test]
    fn custom_rolls_follow_bounds() {
        if let Some(die) = Die::new_custom(30) {
            let mut roll: u16;
            for _ in 0..1000 {
                roll = die.roll_multiple(1)[0];
                assert!(
                    (1..=30).contains(&roll),
                    "Roll out of bounds [1, 30]: got {}",
                    roll
                );
            }
        } else {
            panic!("Failed to create custom die with 30 sides");
        }
    }
}


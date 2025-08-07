use super::die::Die;

pub struct DiceSet {
    die_type: Die,
    quantity: u16,
}

impl DiceSet {
    pub fn new(die_type: Die, quantity: u16) -> Option<Self> {
        Some(Self { die_type: die_type, quantity: quantity.max(1) })
    }

    pub fn die_type(&self) -> &Die {
        &self.die_type
    }

    pub fn set_die_type(&mut self, die_type: Die) {
        self.die_type = die_type;
    }

    pub fn quantity(&self) -> u16 {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: u16) {
        self.quantity = quantity.max(1);
    }

    pub fn dice_set_name(&self) -> String {
        format!("{}{}", self.quantity, self.die_type.name())
    }

    pub fn roll_one(&self) -> u16 {
        self.die_type.roll()
    }

    pub fn roll_all(&self) -> Vec<u16> {
        self.die_type.roll_multiple(self.quantity)
    }
}


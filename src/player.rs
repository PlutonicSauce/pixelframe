// "src/player.rs"

// A module that represents the Player in the game.

pub struct Player {
    // Player's unique identifier
    pub id: u32,
    // Player's name
    pub name: String,
    // Player's current health points
    pub health: u32,
}

impl Player {
    // Creates a new Player instance with default values
    pub fn new(id: u32, name: String) -> Player {
        Player { id, name, health: 100 }
    }

    // Method to deal damage to the player
    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.health {
            self.health = 0; // Player is dead
        } else {
            self.health -= amount; // Reduce health by damage amount
        }
    }

    // Method to heal the player
    pub fn heal(&mut self, amount: u32) {
        self.health += amount; // Increase health by healing amount
    }

    // Method to display player's information
    pub fn display_info(&self) {
        println!("Player ID: {}, Name: {}, Health: {}", self.id, self.name, self.health);
    }
}
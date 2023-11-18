// milliseconds between updates
pub const PLAYER_UPDATE_MS: u32 = 50;
pub const RAT_UPDATE_MS: u32 = 100;
pub const BRAT_UPDATE_MS: u32 = 75;
pub const FACTORY_UPDATE_MS: u32 = 250;
pub const BULLET_UPDATE_MS: u32 = 10;

// player fire rate in nanoseconds
pub const PLAYER_FIRE_RATE_NS: u32 = 1_000_000_000 / 8;

// scoring
pub const RAT_KILL: usize = 50;
pub const BRAT_KILL: usize = 25;
pub const FACTORY_KILL: usize = 250;

// spawn rate
pub const RAT_SPAWN_SECONDS: u64 = 20;
pub const BRAT_SPAWN_SECONDS: u64 = 45;
pub const RATS_PER_FACTORY: f32 = 2.5;

// length of super boom in frames
pub const SUPER_BOOM_FRAMES: usize = 60;

// how long bullet is harmless to player after being fired (in update cycles)
//
// This is done to prevent the situation where the player happens to be moving
// in the same direction they are firing and the update cycles overlap in such a
// way that the player briefly occupies the same position as the bullet; which
// causes the game to treat it as a player kill. They bullet will still kill the
// player if the maze is sparse enough so that the bullet can wrap around and
// hit the player from the opposite direction.
pub const BULLET_HARMLESS_LIFETIME: u32 = 10;

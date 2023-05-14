use candid::Principal;
use config::Config;
use ic_exports::stable_structures::memory_manager::MemoryId;

mod config;

pub const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(80);

/// State of a minter canister.
#[derive(Default)]
pub struct State {
    /// Minter canister configuration.
    pub config: Config,
}

impl State {
    /// Clear the state and set initial data from settings.
    pub fn reset(&mut self, settings: Settings) {
        self.config.reset(settings);
    }
}

/// State settings.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub owner: Principal,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
        }
    }
}

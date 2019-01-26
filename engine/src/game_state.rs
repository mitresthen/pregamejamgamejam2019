use std::fmt;

// Game as a whole can be in those states:
//              | gameplay_running | presents_menu | gameplay_displayed
// Title Screen :                N |             N |                  N
// Main Menu    :                N |             Y |                  N
// Gameplay     :                Y |             N |                  Y
// Pause Menu   :                N |             Y |                  Y

pub struct GameState {
    pub gameplay_running:   bool,
    pub presents_menu:      bool,
    pub gameplay_displayed: bool,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameState(gr: {}, pm:{}, gd:{})", self.gameplay_running, self.presents_menu, self.gameplay_displayed)
    }
}

impl GameState {
    pub fn is_on(&self, ogs: GameState) -> bool {
        // ogs - other game state
        self.gameplay_running   == ogs.gameplay_running   &&
        self.presents_menu      == ogs.presents_menu      &&
        self.gameplay_displayed == ogs.gameplay_displayed
    }

    pub fn go_to(&mut self, ogs: GameState)
    {
        self.gameplay_running   = ogs.gameplay_running;
        self.presents_menu      = ogs.presents_menu;
        self.gameplay_displayed = ogs.gameplay_displayed;
    }

    pub fn invert_paused_state(&mut self)
    {
        self.invert_gameplay_running();
        self.invert_presents_menu();
    }

    fn invert_gameplay_running(&mut self)
    {
        self.gameplay_running = !self.gameplay_running;
    }
    fn invert_presents_menu(&mut self)
    {
        self.presents_menu = !self.presents_menu;
    }
    fn invert_gameplay_displayed(&mut self)
    {
        self.gameplay_displayed = !self.gameplay_displayed;
    }
}

pub const TITLE_STATE      : GameState = GameState { gameplay_running: false, presents_menu: false, gameplay_displayed: false };
pub const MAIN_MENU_STATE  : GameState = GameState { gameplay_running: false, presents_menu: true , gameplay_displayed: false };
pub const GAMEPLAY_STATE   : GameState = GameState { gameplay_running: true , presents_menu: false, gameplay_displayed: true  };
pub const PAUSE_MENU_STATE : GameState = GameState { gameplay_running: false, presents_menu: true , gameplay_displayed: true  };

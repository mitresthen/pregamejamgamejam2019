use CHANGE_TIME;
use std::fmt;

// Game as a whole can be in those states:
//              | gameplay_running | presents_menu | gameplay_displayed | exit
// Title Screen :                N |             N |                  N |    N
// Main Menu    :                N |             Y |                  N |    N
// Gameplay     :                Y |             N |                  Y |    N
// Pause Menu   :                N |             Y |                  Y |    N
// Exit         :                N |             N |                  N |    Y

#[derive(Clone)]
pub struct GameState {
    pub gameplay_running:   bool,
    pub presents_menu:      bool,
    pub gameplay_displayed: bool,
    pub exit:               bool,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameState(gr: {}, pm:{}, gd:{}, e:{})", self.gameplay_running, self.presents_menu, self.gameplay_displayed, self.exit)
    }
}

impl GameState {
    pub fn is_on(&self, ogs: GameState) -> bool {
        // ogs - other game state
        self.gameplay_running   == ogs.gameplay_running   &&
        self.presents_menu      == ogs.presents_menu      &&
        self.gameplay_displayed == ogs.gameplay_displayed &&
        self.exit               == ogs.exit
    }

    pub fn go_to(&mut self, ogs: GameState, dt: f32) -> bool
    {
        let mut relative_dt = dt;
        if ogs.is_on(PAUSE_MENU_STATE)
        {
            relative_dt = 100.0;
        }
        if relative_dt >= CHANGE_TIME
        {
            self.gameplay_running   = ogs.gameplay_running;
            self.presents_menu      = ogs.presents_menu;
            self.gameplay_displayed = ogs.gameplay_displayed;
            return true;
        }
        false
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
}

pub const TITLE_STATE      : GameState = GameState { gameplay_running: false, presents_menu: false, gameplay_displayed: false, exit: false };
pub const MAIN_MENU_STATE  : GameState = GameState { gameplay_running: false, presents_menu: true , gameplay_displayed: false, exit: false };
pub const GAMEPLAY_STATE   : GameState = GameState { gameplay_running: true , presents_menu: false, gameplay_displayed: true , exit: false };
pub const PAUSE_MENU_STATE : GameState = GameState { gameplay_running: false, presents_menu: true , gameplay_displayed: true , exit: false };
pub const EXIT_STATE       : GameState = GameState { gameplay_running: false, presents_menu: false, gameplay_displayed: false, exit: true  };
// TODO: Think of a better way of having middle state for resetting the game than fake one with wrong params
pub const RESET_GAME       : GameState = GameState { gameplay_running: false, presents_menu: false, gameplay_displayed: true , exit: false };

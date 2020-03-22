use crate::cli::smart_cli;
use crate::cli::raw_cli;
use std::borrow::Cow;

pub enum MainMenuAction {
    HostGame,
    JoinGame(/* GameId */ String),
    ReadRules,
}

const MAIN_MENU_PROMPT: &str = "\
Welcome to the Lost Cities game! \
 \
What would you like to do? (press one of the following keys) \
h => [h]ost new game \
j => [j]oin existing game \
r => [r]ead the rules \
";

pub fn handle_main_menu() -> MainMenuAction {
    loop {
        match prompt_for_main_menu_action() {
            Ok(action) => return action,
            Err(msg) => println!("{}", msg),
        }
    }
}

fn prompt_for_main_menu_action() -> smart_cli::PromptResult<MainMenuAction> {
    let cli_host_or_join = raw_cli::prompt_for_input(MAIN_MENU_PROMPT);
    match cli_host_or_join.to_lowercase().as_str() {
        "h" => Ok(MainMenuAction::HostGame),
        "j" => Ok(MainMenuAction::JoinGame(raw_cli::prompt_for_input("Please enter the Game ID you'd like to join: "))),
        "r" => Ok(MainMenuAction::ReadRules),
        _ => Err(Cow::from("Please press either 'h', 'j', or 'r'.")),
    }
}


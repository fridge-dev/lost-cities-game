use crate::cli::smart_cli;
use crate::cli::raw_cli;
use std::borrow::Cow;

pub enum MainMenuAction {
    HostGame,
    JoinGame(/* GameId */ String),
    ReadRules,
}

const MAIN_MENU_PROMPT: &str = "\n\
What would you like to do? (press one of the following keys)\n\
h => [h]ost new game\n\
j => [j]oin existing game\n\
r => [r]ead the rules\n\
";

pub fn handle_main_menu() -> MainMenuAction {
    println!();
    println!("Welcome to the Lost Cities game!");

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

#[cfg(test)]
mod tests {
    use crate::screens::main_menu::MAIN_MENU_PROMPT;

    #[test]
    fn multiline_print() {
        println!("{}", MAIN_MENU_PROMPT);
    }
}

use inquire::Confirm;
use once_cell::sync::Lazy;
use std::sync::RwLock;
pub static NO_CONFIRM: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));

#[derive(Default, Debug)]
pub struct ConfirmProps {
    pub message: String,
    pub default: Option<bool>,
    pub help: Option<String>,
}

pub fn confirm(props: ConfirmProps) -> Result<bool, inquire::InquireError> {
    if *NO_CONFIRM
        .read()
        .expect("Could not read lazy global NO_CONFIRM")
    {
        Ok(true)
    } else {
        Confirm::new(&props.message)
            .with_default(props.default.unwrap_or(false))
            .with_help_message(&props.help.unwrap_or("".to_string()))
            .prompt()
    }
}

use crate::GLOBAL_CMD_MATCHES;
use inquire::Confirm;

#[derive(Default, Debug)]
pub struct ConfirmProps {
    pub message: String,
    pub default: Option<bool>,
    pub help: Option<String>,
}

pub fn confirm(props: ConfirmProps) -> Result<bool, inquire::InquireError> {
    if GLOBAL_CMD_MATCHES
        .lock()
        .expect("Could not read GLOBAL_CMD_MATCHES")
        .get_flag("no-confirm")
    {
        Ok(true)
    } else {
        Confirm::new(&props.message)
            .with_default(props.default.unwrap_or(false))
            .with_help_message(&props.help.unwrap_or("".to_string()))
            .prompt()
    }
}

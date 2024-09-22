use crate::get_options;
use inquire::Confirm;

#[derive(Default, Debug)]
pub struct ConfirmProps {
    pub message: String,
    pub default: Option<bool>,
    pub help: Option<String>,
}
pub fn confirm(props: ConfirmProps) -> Result<bool, inquire::InquireError> {
    if get_options().no_confirm {
        Ok(true)
    } else {
        // Interactive confirmation:
        #[cfg_attr(coverage_nightly, coverage(off))]
        Confirm::new(&props.message)
            .with_default(props.default.unwrap_or(false))
            .with_help_message(&props.help.unwrap_or("".to_string()))
            .prompt()
    }
}

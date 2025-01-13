use clap::Args;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TerminalShell {
    /// Bourne Again `SHell` (bash)
    Bash,

    /// Elvish shell (elvish)
    Elvish,

    /// Friendly Interactive `SHell` (fish)
    Fish,

    /// `Nushell` (nushell)
    Nushell,

    /// `PowerShell` (powershell)
    PowerShell,

    /// Z `SHell` (zsh)
    Zsh,
}

impl clap::ValueEnum for TerminalShell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Bash,
            Self::Elvish,
            Self::Fish,
            Self::Nushell,
            Self::PowerShell,
            Self::Zsh,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Bash => clap::builder::PossibleValue::new("bash"),
            Self::Elvish => clap::builder::PossibleValue::new("elvish"),
            Self::Fish => clap::builder::PossibleValue::new("fish"),
            Self::Nushell => clap::builder::PossibleValue::new("nushell"),
            Self::PowerShell => clap::builder::PossibleValue::new("powershell"),
            Self::Zsh => clap::builder::PossibleValue::new("zsh"),
        })
    }
}

#[derive(Args, Debug)]
pub struct CompletionsCommandArguments {
    pub shell: TerminalShell,
}

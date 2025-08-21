use clap::{ArgAction, CommandFactory, FromArgMatches, Parser};

// An intermediary struct which lets us remove the helper fields _no_* in the ultimate `Args`
#[derive(Parser, Debug, Clone, Default)]
struct ArgsInternal {
    /// Name of the player for which to show the information
    pub player_name: String,

    // NB: the `_no_missing` and `missing`s clap annotations are swapped to support a default of
    // `true` which gets reset by the `--no-missing` flag. See https://jwodder.github.io/kbits/posts/clap-bool-negate/
    #[arg(short = 'm', long = "missing", overrides_with = "missing")]
    #[doc(hidden)]
    /// Show items which are not fully researched
    _no_missing: bool,
    #[arg(short = 'M', long = "no-missing", action = ArgAction::SetFalse)]
    pub missing: bool,

    /// Show items which are fully researched
    #[arg(short, long)]
    pub complete: bool,
    #[arg(short = 'C', long = "no-complete")]
    #[doc(hidden)]
    _no_complete: bool,
}

#[derive(Debug, Clone)]
pub struct Args {
    /// Name of the player for which to show the information
    pub player_name: String,
    /// Show items which are not fully researched
    pub missing: bool,
    /// Show items which are fully researched
    pub complete: bool,
}

impl Parser for Args {}
impl FromArgMatches for Args {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        ArgsInternal::from_arg_matches(matches).map(Into::into)
    }
    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        let mut ai: ArgsInternal = self.clone().into();
        ai.update_from_arg_matches(matches)?;
        *self = ai.into();
        Ok(())
    }
}

impl CommandFactory for Args {
    fn command() -> clap::Command {
        ArgsInternal::command()
    }

    fn command_for_update() -> clap::Command {
        ArgsInternal::command_for_update()
    }
}

impl From<ArgsInternal> for Args {
    fn from(
        ArgsInternal {
            player_name,
            missing,
            complete,
            ..
        }: ArgsInternal,
    ) -> Self {
        Self {
            player_name,
            missing,
            complete,
        }
    }
}

impl From<Args> for ArgsInternal {
    fn from(
        Args {
            player_name,
            missing,
            complete,
            ..
        }: Args,
    ) -> Self {
        Self {
            player_name,
            missing,
            complete,
            ..Default::default()
        }
    }
}

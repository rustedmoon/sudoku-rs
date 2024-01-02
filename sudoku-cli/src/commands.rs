use clap::{Parser, Subcommand, ValueEnum};
use sudoku_engine::{Difficulty, BoardSize};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate sudoku
    Generate {
        /// Difficulty
        #[arg(value_enum, default_value_t = CliDifficulty::Medium)]
        difficulty: CliDifficulty,
        /// BoardSize
        #[arg(value_enum, default_value_t = CliBoardSize::Nine)]
        size: CliBoardSize,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum CliDifficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
    Extreme,
    Impossible,
}

impl From<CliDifficulty> for Difficulty {
    fn from(value: CliDifficulty) -> Self {
        match value {
            CliDifficulty::Beginner => Self::Beginner,
            CliDifficulty::Easy => Self::Easy,
            CliDifficulty::Medium => Self::Medium,
            CliDifficulty::Hard => Self::Hard,
            CliDifficulty::Extreme => Self::Extreme,
            CliDifficulty::Impossible => Self::Impossible,
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum CliBoardSize {
    #[clap(name = "4")]
    Four,
    #[clap(name = "9")]
    Nine,
    #[clap(name = "16")]
    Sixteen,
}

impl From<CliBoardSize> for BoardSize {
    fn from(value: CliBoardSize) -> Self {
        match value {
            CliBoardSize::Four => Self::Four,
            CliBoardSize::Nine => Self::Nine,
            CliBoardSize::Sixteen => Self::Sixteen,
        }
    }
}
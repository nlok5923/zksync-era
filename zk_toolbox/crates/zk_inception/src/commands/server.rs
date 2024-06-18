use anyhow::Context;
use clap::Parser;
use common::{cmd::Cmd, config::global_config, logger, spinner::Spinner};
use config::{ChainConfig, EcosystemConfig};
use serde::{Deserialize, Serialize};
use xshell::{cmd, Shell};

use crate::{
    messages::{
        MSG_BUILDING_L1_CONTRACTS, MSG_CHAIN_NOT_INITIALIZED, MSG_SERVER_ADDITIONAL_ARGS_HELP,
        MSG_SERVER_COMPONENTS_HELP, MSG_SERVER_GENESIS_HELP, MSG_STARTING_SERVER,
    },
    server::{RunServer, ServerMode},
};

#[derive(Debug, Serialize, Deserialize, Parser)]
pub struct RunServerArgs {
    #[clap(long, help = MSG_SERVER_COMPONENTS_HELP)]
    pub components: Option<Vec<String>>,
    #[clap(long, help = MSG_SERVER_GENESIS_HELP)]
    pub genesis: bool,
    #[clap(long, short)]
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = false, help = MSG_SERVER_ADDITIONAL_ARGS_HELP)]
    additional_args: Vec<String>,
}

pub fn run(shell: &Shell, args: RunServerArgs) -> anyhow::Result<()> {
    let ecosystem_config = EcosystemConfig::from_file(shell)?;

    let chain = global_config().chain_name.clone();
    let chain_config = ecosystem_config
        .load_chain(chain)
        .context(MSG_CHAIN_NOT_INITIALIZED)?;

    logger::info(MSG_STARTING_SERVER);

    build_l1_contracts(shell, &ecosystem_config)?;
    run_server(args, &chain_config, shell)?;

    Ok(())
}

fn build_l1_contracts(shell: &Shell, ecosystem_config: &EcosystemConfig) -> anyhow::Result<()> {
    let _dir_guard = shell.push_dir(ecosystem_config.path_to_foundry());
    let spinner = Spinner::new(MSG_BUILDING_L1_CONTRACTS);
    Cmd::new(cmd!(shell, "yarn build")).run()?;
    spinner.finish();
    Ok(())
}

fn run_server(
    args: RunServerArgs,
    chain_config: &ChainConfig,
    shell: &Shell,
) -> anyhow::Result<()> {
    let server = RunServer::new(args.components.clone(), chain_config);
    let mode = if args.genesis {
        ServerMode::Genesis
    } else {
        ServerMode::Normal
    };
    server.run(shell, mode)
}

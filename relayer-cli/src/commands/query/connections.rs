use alloc::sync::Arc;

use abscissa_core::clap::Parser;
use abscissa_core::Runnable;
use tokio::runtime::Runtime as TokioRuntime;

use ibc::core::ics24_host::identifier::{ChainId, ConnectionId};
use ibc_proto::ibc::core::connection::v1::QueryConnectionsRequest;
use ibc_relayer::chain::{ChainEndpoint, CosmosSdkChain, SubstrateChain};

use crate::conclude::{exit_with_unrecoverable_error, Output};
use crate::prelude::*;

#[derive(Clone, Command, Debug, Parser)]
pub struct QueryConnectionsCmd {
    #[clap(required = true, help = "identifier of the chain to query")]
    chain_id: ChainId,
}

// hermes query connections ibc-0
impl Runnable for QueryConnectionsCmd {
    fn run(&self) {
        let config = app_config();

        let chain_config = match config.find_chain(&self.chain_id) {
            None => Output::error(format!(
                "chain '{}' not found in configuration file",
                self.chain_id
            ))
            .exit(),
            Some(chain_config) => chain_config,
        };

        debug!("Options: {:?}", self);

        let rt = Arc::new(TokioRuntime::new().unwrap());
        let chain_type = chain_config.account_prefix.clone();
        match chain_type.as_str() {
            "cosmos" => {
                let chain = CosmosSdkChain::bootstrap(chain_config.clone(), rt).unwrap_or_else(exit_with_unrecoverable_error);


                let req = QueryConnectionsRequest {
                    pagination: ibc_proto::cosmos::base::query::pagination::all(),
                };

                let res = chain.query_connections(req);

                match res {
                    Ok(connections) => {
                        let ids: Vec<ConnectionId> = connections
                            .into_iter()
                            .map(|identified_connection| identified_connection.connection_id)
                            .collect();

                        Output::success(ids).exit()
                    }
                    Err(e) => Output::error(format!("{}", e)).exit(),
                }
            }
            "substrate" => {
                let chain = SubstrateChain::bootstrap(chain_config.clone(), rt).unwrap();

                let req = QueryConnectionsRequest {
                    pagination: ibc_proto::cosmos::base::query::pagination::all(),
                };

                let res = chain.query_connections(req);

                match res {
                    Ok(connections) => {
                        let ids: Vec<ConnectionId> = connections
                            .into_iter()
                            .map(|identified_connection| identified_connection.connection_id)
                            .collect();

                        Output::success(ids).exit()
                    }
                    Err(e) => Output::error(format!("{}", e)).exit(),
                }
            }
            _ => panic!("Unknown chain type"),
        }
    }
}

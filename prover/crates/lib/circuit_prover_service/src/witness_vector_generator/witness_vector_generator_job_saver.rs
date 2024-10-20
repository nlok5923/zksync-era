use anyhow::Context;
use async_trait::async_trait;
use zksync_prover_dal::{ConnectionPool, Prover};
use zksync_prover_job_processor::JobSaver;
use zksync_types::prover_dal::FriProverJobMetadata;

use crate::{
    types::witness_vector_generator_execution_output::WitnessVectorGeneratorExecutionOutput,
    witness_vector_generator::WitnessVectorGeneratorExecutor,
};

pub struct WitnessVectorGeneratorJobSaver {
    connection_pool: ConnectionPool<Prover>,
    sender:
        tokio::sync::mpsc::Sender<(WitnessVectorGeneratorExecutionOutput, FriProverJobMetadata)>,
}

impl WitnessVectorGeneratorJobSaver {
    pub fn new(
        connection_pool: ConnectionPool<Prover>,
        sender: tokio::sync::mpsc::Sender<(
            WitnessVectorGeneratorExecutionOutput,
            FriProverJobMetadata,
        )>,
    ) -> Self {
        Self {
            connection_pool,
            sender,
        }
    }
}

#[async_trait]
impl JobSaver for WitnessVectorGeneratorJobSaver {
    type ExecutorType = WitnessVectorGeneratorExecutor;

    async fn save_result(
        &self,
        data: (
            anyhow::Result<WitnessVectorGeneratorExecutionOutput>,
            FriProverJobMetadata,
        ),
    ) -> anyhow::Result<()> {
        let (result, metadata) = data;
        match result {
            Ok(payload) => {
                // let WitnessVectorGeneratorExecutionOutput { circuit, witness_vector } = output;
                // let prover_job = ProverJob::new(metadata.block_number, metadata.id, circuit_wrapper, ProverServiceDataKey { circuit_id: metadata.circuit_id, round: metadata.aggregation_round });
                // let output = WitnessVectorArtifactsTemp::new(
                //     witness_vector,
                //     prover_job,
                //     Instant::now(),
                // );
                self.sender
                    .send((payload, metadata))
                    .await
                    .context("failed to send witness vector to prover")?;
            }
            Err(err) => {}
        }
        Ok(())
    }
}

use std::{collections::HashMap, fmt::Debug};

use colored::*;
use strum::{Display, EnumString};
use zksync_basic_types::{basic_fri_types::AggregationRound, prover_dal::JobCountStatistics};
use zksync_config::PostgresConfig;
use zksync_env_config::FromEnv;
use zksync_types::L1BatchNumber;

pub fn postgres_config() -> anyhow::Result<PostgresConfig> {
    Ok(PostgresConfig::from_env()?)
}

pub struct BatchData {
    pub batch_number: L1BatchNumber,
    pub basic_witness_generator: Task,
    pub leaf_witness_generator: Task,
    pub node_witness_generator: Task,
    pub recursion_tip: Task,
    pub scheduler: Task,
    pub compressor: Task,
}

impl Debug for BatchData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "== {} ==",
            format!("Batch {} Status", self.batch_number).bold()
        )?;
        writeln!(f)?;
        writeln!(f, "= {} =", format!("Proving Stages").bold())?;
        writeln!(f, "{:?}", self.basic_witness_generator)?;
        writeln!(f, "{:?}", self.leaf_witness_generator)?;
        writeln!(f, "{:?}", self.node_witness_generator)?;
        writeln!(f, "{:?}", self.recursion_tip)?;
        writeln!(f, "{:?}", self.scheduler)?;
        writeln!(f, "{:?}", self.compressor)
    }
}

impl Default for BatchData {
    fn default() -> Self {
        BatchData {
            batch_number: L1BatchNumber::default(),
            basic_witness_generator: Task::BasicWitnessGenerator(TaskStatus::Stuck),
            leaf_witness_generator: Task::LeafWitnessGenerator {
                status: TaskStatus::WaitingForProofs,
                aggregation_round_0_prover_jobs_data: ProverJobsData::default(),
            },
            node_witness_generator: Task::NodeWitnessGenerator {
                status: TaskStatus::WaitingForProofs,
                aggregation_round_1_prover_jobs_data: ProverJobsData::default(),
            },
            recursion_tip: Task::RecursionTip {
                status: TaskStatus::WaitingForProofs,
                aggregation_round_2_prover_jobs_data: ProverJobsData::default(),
            },
            scheduler: Task::Scheduler(TaskStatus::WaitingForProofs),
            compressor: Task::Compressor(TaskStatus::WaitingForProofs),
        }
    }
}

#[derive(Debug, EnumString, Clone, Display)]
pub enum TaskStatus {
    /// A task is considered queued when all of its jobs is queued.
    #[strum(to_string = "Queued 📥")]
    Queued,
    /// A task is considered in progress when at least one of its jobs differs in its status.
    #[strum(to_string = "In Progress ⌛️")]
    InProgress,
    /// A task is considered successful when all of its jobs were processed successfully.
    #[strum(to_string = "Successful ✅")]
    Successful,
    /// A task is considered waiting for proofs when all of its jobs are waiting for proofs.
    #[strum(to_string = "Waiting for Proof ⏱️")]
    WaitingForProofs,
    /// A task is considered stuck when at least one of its jobs is stuck.
    #[strum(to_string = "Stuck 🛑")]
    Stuck,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Queued
    }
}

impl Copy for TaskStatus {}

type ProverJobsData = HashMap<(L1BatchNumber, AggregationRound), JobCountStatistics>;

#[derive(EnumString, Clone, Display)]
pub enum Task {
    #[strum(to_string = "Basic Witness Generator")]
    BasicWitnessGenerator(TaskStatus),
    #[strum(to_string = "Leaf Witness Generator")]
    LeafWitnessGenerator {
        status: TaskStatus,
        aggregation_round_0_prover_jobs_data: ProverJobsData,
    },
    #[strum(to_string = "Node Witness Generator")]
    NodeWitnessGenerator {
        status: TaskStatus,
        aggregation_round_1_prover_jobs_data: ProverJobsData,
    },
    #[strum(to_string = "Recursion Tip")]
    RecursionTip {
        status: TaskStatus,
        aggregation_round_2_prover_jobs_data: ProverJobsData,
    },
    #[strum(to_string = "Scheduler")]
    Scheduler(TaskStatus),
    #[strum(to_string = "Compressor")]
    Compressor(TaskStatus),
}

impl Task {
    fn status(&self) -> TaskStatus {
        match self {
            Task::BasicWitnessGenerator(status)
            | Task::LeafWitnessGenerator { status, .. }
            | Task::NodeWitnessGenerator { status, .. }
            | Task::RecursionTip { status, .. }
            | Task::Scheduler(status)
            | Task::Compressor(status) => *status,
        }
    }
}

impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-- {} --", self.to_string().bold())?;
        writeln!(f, "> {}", self.status().to_string())
    }
}

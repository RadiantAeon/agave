//! Implementation defined RPC server errors
use {
    crate::response::RpcSimulateTransactionResult,
    jsonrpsee_types::{ErrorObject, ErrorObjectOwned},
    serde::{Deserialize, Serialize},
    solana_clock::Slot,
    solana_transaction_status_client_types::EncodeError,
    thiserror::Error,
};

// Keep in sync with https://github.com/solana-labs/solana-web3.js/blob/master/src/errors.ts
pub const JSON_RPC_SERVER_ERROR_BLOCK_CLEANED_UP: i64 = -32001;
pub const JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE: i64 = -32002;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_VERIFICATION_FAILURE: i64 = -32003;
pub const JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE: i64 = -32004;
pub const JSON_RPC_SERVER_ERROR_NODE_UNHEALTHY: i64 = -32005;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_PRECOMPILE_VERIFICATION_FAILURE: i64 = -32006;
pub const JSON_RPC_SERVER_ERROR_SLOT_SKIPPED: i64 = -32007;
pub const JSON_RPC_SERVER_ERROR_NO_SNAPSHOT: i64 = -32008;
pub const JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_SLOT_SKIPPED: i64 = -32009;
pub const JSON_RPC_SERVER_ERROR_KEY_EXCLUDED_FROM_SECONDARY_INDEX: i64 = -32010;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_HISTORY_NOT_AVAILABLE: i64 = -32011;
pub const JSON_RPC_SCAN_ERROR: i64 = -32012;
pub const JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_LEN_MISMATCH: i64 = -32013;
pub const JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET: i64 = -32014;
pub const JSON_RPC_SERVER_ERROR_UNSUPPORTED_TRANSACTION_VERSION: i64 = -32015;
pub const JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED: i64 = -32016;
pub const JSON_RPC_SERVER_ERROR_EPOCH_REWARDS_PERIOD_ACTIVE: i64 = -32017;
pub const JSON_RPC_SERVER_ERROR_SLOT_NOT_EPOCH_BOUNDARY: i64 = -32018;
pub const JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_UNREACHABLE: i64 = -32019;

#[derive(Error, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum RpcCustomError {
    #[error("BlockCleanedUp")]
    BlockCleanedUp {
        slot: Slot,
        first_available_block: Slot,
    },
    #[error("SendTransactionPreflightFailure")]
    SendTransactionPreflightFailure {
        message: String,
        result: RpcSimulateTransactionResult,
    },
    #[error("TransactionSignatureVerificationFailure")]
    TransactionSignatureVerificationFailure,
    #[error("BlockNotAvailable")]
    BlockNotAvailable { slot: Slot },
    #[error("NodeUnhealthy")]
    NodeUnhealthy { num_slots_behind: Option<Slot> },
    #[error("TransactionPrecompileVerificationFailure")]
    TransactionPrecompileVerificationFailure(solana_transaction_error::TransactionError),
    #[error("SlotSkipped")]
    SlotSkipped { slot: Slot },
    #[error("NoSnapshot")]
    NoSnapshot,
    #[error("LongTermStorageSlotSkipped")]
    LongTermStorageSlotSkipped { slot: Slot },
    #[error("KeyExcludedFromSecondaryIndex")]
    KeyExcludedFromSecondaryIndex { index_key: String },
    #[error("TransactionHistoryNotAvailable")]
    TransactionHistoryNotAvailable,
    #[error("ScanError")]
    ScanError { message: String },
    #[error("TransactionSignatureLenMismatch")]
    TransactionSignatureLenMismatch,
    #[error("BlockStatusNotAvailableYet")]
    BlockStatusNotAvailableYet { slot: Slot },
    #[error("UnsupportedTransactionVersion")]
    UnsupportedTransactionVersion(u8),
    #[error("MinContextSlotNotReached")]
    MinContextSlotNotReached { context_slot: Slot },
    #[error("EpochRewardsPeriodActive")]
    EpochRewardsPeriodActive {
        slot: Slot,
        current_block_height: u64,
        rewards_complete_block_height: u64,
    },
    #[error("SlotNotEpochBoundary")]
    SlotNotEpochBoundary { slot: Slot },
    #[error("LongTermStorageUnreachable")]
    LongTermStorageUnreachable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeUnhealthyErrorData {
    pub num_slots_behind: Option<Slot>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinContextSlotNotReachedErrorData {
    pub context_slot: Slot,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpochRewardsPeriodActiveErrorData {
    pub current_block_height: u64,
    pub rewards_complete_block_height: u64,
    pub slot: Option<u64>,
}

impl From<EncodeError> for RpcCustomError {
    fn from(err: EncodeError) -> Self {
        match err {
            EncodeError::UnsupportedTransactionVersion(version) => {
                Self::UnsupportedTransactionVersion(version)
            }
        }
    }
}

impl From<RpcCustomError> for ErrorObjectOwned {
    fn from(e: RpcCustomError) -> Self {
        match e {
            RpcCustomError::BlockCleanedUp {
                slot,
                first_available_block,
            } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_BLOCK_CLEANED_UP as i32,
                format!(
                    "Block {slot} cleaned up, does not exist on node. First available block: \
                     {first_available_block}",
                ),
                None::<()>,
            ),
            RpcCustomError::SendTransactionPreflightFailure { message, result } => {
                ErrorObject::owned(
                    JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE as i32,
                    message,
                    Some(serde_json::json!(result)),
                )
            }
            RpcCustomError::TransactionSignatureVerificationFailure => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_VERIFICATION_FAILURE as i32,
                "Transaction signature verification failure".to_string(),
                None::<()>,
            ),
            RpcCustomError::BlockNotAvailable { slot } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE as i32,
                format!("Block not available for slot {slot}"),
                None::<()>,
            ),
            RpcCustomError::NodeUnhealthy { num_slots_behind } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_NODE_UNHEALTHY as i32,
                if let Some(num_slots_behind) = num_slots_behind {
                    format!("Node is behind by {num_slots_behind} slots")
                } else {
                    "Node is unhealthy".to_string()
                },
                Some(serde_json::json!(NodeUnhealthyErrorData {
                    num_slots_behind
                })),
            ),
            RpcCustomError::TransactionPrecompileVerificationFailure(e) => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_TRANSACTION_PRECOMPILE_VERIFICATION_FAILURE as i32,
                format!("Transaction precompile verification failure {e:?}"),
                None::<()>,
            ),
            RpcCustomError::SlotSkipped { slot } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_SLOT_SKIPPED as i32,
                format!(
                    "Slot {slot} was skipped, or missing due to ledger jump to recent snapshot"
                ),
                None::<()>,
            ),
            RpcCustomError::NoSnapshot => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_NO_SNAPSHOT as i32,
                "No snapshot".to_string(),
                None::<()>,
            ),
            RpcCustomError::LongTermStorageSlotSkipped { slot } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_SLOT_SKIPPED as i32,
                format!("Slot {slot} was skipped, or missing in long-term storage"),
                None::<()>,
            ),
            RpcCustomError::KeyExcludedFromSecondaryIndex { index_key } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_KEY_EXCLUDED_FROM_SECONDARY_INDEX as i32,
                format!(
                    "{index_key} excluded from account secondary indexes; this RPC method \
                     unavailable for key"
                ),
                None::<()>,
            ),
            RpcCustomError::TransactionHistoryNotAvailable => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_TRANSACTION_HISTORY_NOT_AVAILABLE as i32,
                "Transaction history is not available from this node".to_string(),
                None::<()>,
            ),
            RpcCustomError::ScanError { message } => {
                ErrorObject::owned(JSON_RPC_SCAN_ERROR as i32, message, None::<()>)
            }
            RpcCustomError::TransactionSignatureLenMismatch => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_LEN_MISMATCH as i32,
                "Transaction signature length mismatch".to_string(),
                None::<()>,
            ),
            RpcCustomError::BlockStatusNotAvailableYet { slot } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET as i32,
                format!("Block status not yet available for slot {slot}"),
                None::<()>,
            ),
            RpcCustomError::UnsupportedTransactionVersion(version) => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_UNSUPPORTED_TRANSACTION_VERSION as i32,
                format!(
                    "Transaction version ({version}) is not supported by the requesting client. \
                     Please try the request again with the following configuration parameter: \
                     \"maxSupportedTransactionVersion\": {version}"
                ),
                None::<()>,
            ),
            RpcCustomError::MinContextSlotNotReached { context_slot } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED as i32,
                "Minimum context slot has not been reached".to_string(),
                Some(serde_json::json!(MinContextSlotNotReachedErrorData {
                    context_slot,
                })),
            ),
            RpcCustomError::EpochRewardsPeriodActive {
                slot,
                current_block_height,
                rewards_complete_block_height,
            } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_EPOCH_REWARDS_PERIOD_ACTIVE as i32,
                format!("Epoch rewards period still active at slot {slot}"),
                Some(serde_json::json!(EpochRewardsPeriodActiveErrorData {
                    current_block_height,
                    rewards_complete_block_height,
                    slot: Some(slot),
                })),
            ),
            RpcCustomError::SlotNotEpochBoundary { slot } => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_SLOT_NOT_EPOCH_BOUNDARY as i32,
                format!(
                    "Rewards cannot be found because slot {slot} is not the epoch boundary. This \
                     may be due to gap in the queried node's local ledger or long-term storage"
                ),
                Some(serde_json::json!({
                    "slot": slot,
                })),
            ),
            RpcCustomError::LongTermStorageUnreachable => ErrorObject::owned(
                JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_UNREACHABLE as i32,
                "Failed to query long-term storage; please try again".to_string(),
                None::<()>,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::custom_error::EpochRewardsPeriodActiveErrorData, serde_json::Value,
        test_case::test_case,
    };

    #[test_case(serde_json::json!({
            "currentBlockHeight": 123,
            "rewardsCompleteBlockHeight": 456
        }); "Pre-3.0 schema")]
    #[test_case(serde_json::json!({
            "currentBlockHeight": 123,
            "rewardsCompleteBlockHeight": 456,
            "slot": 789
        }); "3.0+ schema")]
    fn test_deseriailze_epoch_rewards_period_active_error_data(serialized_data: Value) {
        let expected_current_block_height = serialized_data
            .get("currentBlockHeight")
            .map(|v| v.as_u64().unwrap())
            .unwrap();
        let expected_rewards_complete_block_height = serialized_data
            .get("rewardsCompleteBlockHeight")
            .map(|v| v.as_u64().unwrap())
            .unwrap();
        let expected_slot: Option<u64> = serialized_data.get("slot").map(|v| v.as_u64().unwrap());
        let actual: EpochRewardsPeriodActiveErrorData =
            serde_json::from_value(serialized_data).expect("Failed to deserialize test fixture");
        assert_eq!(
            actual,
            EpochRewardsPeriodActiveErrorData {
                current_block_height: expected_current_block_height,
                rewards_complete_block_height: expected_rewards_complete_block_height,
                slot: expected_slot,
            }
        );
    }
}

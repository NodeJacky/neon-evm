use super::{e, Rpc};
use crate::types::{ChDbConfig, TracerDb, TxParams};
use solana_client::{
    client_error::Result as ClientResult,
    client_error::{ClientError, ClientErrorKind},
    rpc_config::{RpcSendTransactionConfig, RpcTransactionConfig},
    rpc_response::{Response, RpcResponseContext, RpcResult},
};
use solana_sdk::{
    account::Account,
    clock::{Slot, UnixTimestamp},
    commitment_config::CommitmentConfig,
    hash::Hash,
    pubkey::Pubkey,
    signature::Signature,
    transaction::Transaction,
};
use solana_transaction_status::{
    EncodedConfirmedBlock, EncodedConfirmedTransactionWithStatusMeta, TransactionStatus,
};
use std::any::Any;

pub struct CallDbClient {
    pub slot: u64,
    tracer_db: TracerDb,
}

impl CallDbClient {
    pub fn new(config: &ChDbConfig, slot: u64) -> Self {
        let db = TracerDb::new(config);
        Self {
            slot,
            tracer_db: db,
        }
    }
}

impl Rpc for CallDbClient {
    fn commitment(&self) -> CommitmentConfig {
        CommitmentConfig::default()
    }

    fn confirm_transaction_with_spinner(
        &self,
        _signature: &Signature,
        _recent_blockhash: &Hash,
        _commitment_config: CommitmentConfig,
    ) -> ClientResult<()> {
        Err(e!(
            "confirm_transaction_with_spinner() not implemented for db_call_client"
        ))
    }

    fn get_account(&self, key: &Pubkey) -> ClientResult<Account> {
        self.tracer_db
            .get_account_at(key, self.slot)
            .map_err(|e| e!("load account error", key, e))?
            .ok_or_else(|| e!("account not found", key))
    }

    fn get_account_with_commitment(
        &self,
        key: &Pubkey,
        _: CommitmentConfig,
    ) -> RpcResult<Option<Account>> {
        let account = self
            .tracer_db
            .get_account_at(key, self.slot)
            .map_err(|e| e!("load account error", key, e))?;

        let context = RpcResponseContext {
            slot: self.slot,
            api_version: None,
        };
        Ok(Response {
            context,
            value: account,
        })
    }

    fn get_multiple_accounts(&self, pubkeys: &[Pubkey]) -> ClientResult<Vec<Option<Account>>> {
        let mut result = Vec::new();
        for key in pubkeys {
            let account = self
                .tracer_db
                .get_account_at(key, self.slot)
                .map_err(|e| e!("load account error", key, e))?;
            result.push(account);
        }
        Ok(result)
    }

    fn get_account_data(&self, key: &Pubkey) -> ClientResult<Vec<u8>> {
        Ok(self.get_account(key)?.data)
    }

    fn get_block(&self, _slot: Slot) -> ClientResult<EncodedConfirmedBlock> {
        Err(e!("get_block() not implemented for db_call_client"))
    }

    fn get_block_time(&self, slot: Slot) -> ClientResult<UnixTimestamp> {
        self.tracer_db
            .get_block_time(slot)
            .map_err(|e| e!("get_block_time error", slot, e))
    }

    fn get_latest_blockhash(&self) -> ClientResult<Hash> {
        Err(e!(
            "get_latest_blockhash() not implemented for db_call_client"
        ))
    }

    fn get_minimum_balance_for_rent_exemption(&self, _data_len: usize) -> ClientResult<u64> {
        Err(e!(
            "get_minimum_balance_for_rent_exemption() not implemented for db_call_client"
        ))
    }

    fn get_slot(&self) -> ClientResult<Slot> {
        Ok(self.slot)
    }

    fn get_signature_statuses(
        &self,
        _signatures: &[Signature],
    ) -> RpcResult<Vec<Option<TransactionStatus>>> {
        Err(e!(
            "get_signature_statuses() not implemented for db_call_client"
        ))
    }

    fn get_transaction_with_config(
        &self,
        _signature: &Signature,
        _config: RpcTransactionConfig,
    ) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta> {
        Err(e!(
            "get_transaction_with_config() not implemented for db_call_client"
        ))
    }

    fn send_transaction(&self, _transaction: &Transaction) -> ClientResult<Signature> {
        Err(e!("send_transaction() not implemented for db_call_client"))
    }

    fn send_and_confirm_transaction_with_spinner(
        &self,
        _transaction: &Transaction,
    ) -> ClientResult<Signature> {
        Err(e!(
            "send_and_confirm_transaction_with_spinner() not implemented for db_call_client"
        ))
    }

    fn send_and_confirm_transaction_with_spinner_and_commitment(
        &self,
        _transaction: &Transaction,
        _commitment: CommitmentConfig,
    ) -> ClientResult<Signature> {
        Err(e!("send_and_confirm_transaction_with_spinner_and_commitment() not implemented for db_call_client"))
    }

    fn send_and_confirm_transaction_with_spinner_and_config(
        &self,
        _transaction: &Transaction,
        _commitment: CommitmentConfig,
        _config: RpcSendTransactionConfig,
    ) -> ClientResult<Signature> {
        Err(e!("send_and_confirm_transaction_with_spinner_and_config() not implemented for db_call_client"))
    }

    fn get_latest_blockhash_with_commitment(
        &self,
        _commitment: CommitmentConfig,
    ) -> ClientResult<(Hash, u64)> {
        Err(e!(
            "get_latest_blockhash_with_commitment() not implemented for db_call_client"
        ))
    }

    fn get_transaction_data(&self) -> ClientResult<TxParams> {
        Err(e!(
            "get_transaction_data() not implemented for db_call_client"
        ))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

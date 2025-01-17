use crate::{
    commands::get_neon_elf::read_elf_parameters_from_account, errors::NeonCliError, Config,
    Context, NeonCliResult,
};
use evm_loader::account::{MainTreasury, Treasury};
use log::{info, warn};
use solana_cli::checks::check_account_for_fee;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    system_program,
    transaction::Transaction,
};
use spl_token::instruction::sync_native;

pub fn execute(config: &Config, context: &Context) -> NeonCliResult {
    let neon_params = read_elf_parameters_from_account(config, context)?;

    let pool_count: u32 = neon_params
        .get("NEON_POOL_COUNT")
        .and_then(|value| value.parse().ok())
        .ok_or(NeonCliError::IncorrectProgram(config.evm_loader))?;

    let main_balance_address = MainTreasury::address(&config.evm_loader).0;

    info!("Main pool balance: {}", main_balance_address);

    let client = context
        .rpc_client
        .as_any()
        .downcast_ref::<RpcClient>()
        .expect("cast to solana_client::rpc_client::RpcClient error");

    for i in 0..pool_count {
        let (aux_balance_address, _) = Treasury::address(&config.evm_loader, i);

        if let Some(aux_balance_account) = context
            .rpc_client
            .get_account_with_commitment(&aux_balance_address, config.commitment)?
            .value
        {
            let minimal_balance = context
                .rpc_client
                .get_minimum_balance_for_rent_exemption(aux_balance_account.data.len())?;
            let available_lamports = aux_balance_account.lamports.saturating_sub(minimal_balance);
            if available_lamports > 0 {
                info!(
                    "{:4}: collect {} lamports from {}",
                    i, available_lamports, aux_balance_address
                );
                let mut message = Message::new(
                    &[Instruction::new_with_bincode(
                        config.evm_loader,
                        &(30_u8, i),
                        vec![
                            AccountMeta::new(main_balance_address, false),
                            AccountMeta::new(aux_balance_address, false),
                            AccountMeta::new_readonly(system_program::id(), false),
                        ],
                    )],
                    Some(&context.signer.pubkey()),
                );
                let blockhash = context.rpc_client.get_latest_blockhash()?;
                message.recent_blockhash = blockhash;

                check_account_for_fee(client, &context.signer.pubkey(), &message)?;

                let mut trx = Transaction::new_unsigned(message);
                trx.try_sign(&[&*context.signer], blockhash)?;
                context
                    .rpc_client
                    .send_and_confirm_transaction_with_spinner(&trx)?;
            } else {
                info!("{:4}: skip account {}", i, aux_balance_address);
            }
        } else {
            warn!("{:4}: not found account {}", i, aux_balance_address);
        }
    }
    let mut message = Message::new(
        &[sync_native(&spl_token::id(), &main_balance_address)?],
        Some(&context.signer.pubkey()),
    );
    let blockhash = context.rpc_client.get_latest_blockhash()?;
    message.recent_blockhash = blockhash;

    check_account_for_fee(client, &context.signer.pubkey(), &message)?;

    let mut trx = Transaction::new_unsigned(message);
    trx.try_sign(&[&*context.signer], blockhash)?;
    context
        .rpc_client
        .send_and_confirm_transaction_with_spinner(&trx)?;

    let main_balance_account = context.rpc_client.get_account(&main_balance_address)?;
    Ok(serde_json::json!({
        "pool_address": main_balance_address.to_string(),
        "balance": main_balance_account.lamports
    }))
}

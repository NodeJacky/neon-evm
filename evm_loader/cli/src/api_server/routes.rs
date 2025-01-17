// use evm_loader::types::Address;
use crate::api_server::handlers::{
    emulate::emulate, emulate_hash::emulate_hash, get_ether_account_data::get_ether_account_data,
    get_storage_at::get_storage_at, trace::trace, trace_hash::trace_hash,
};

use crate::api_server::state::State;

pub fn register(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/emulate").post(emulate);
    api.at("/emulate_hash").post(emulate_hash);
    api.at("/get-storage-at").get(get_storage_at);
    api.at("/get-ether-account-data")
        .get(get_ether_account_data);
    api.at("/trace").post(trace);
    api.at("/trace_hash").post(trace_hash);

    api
}

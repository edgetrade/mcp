pub mod envelope_game;
pub mod game_state;
pub mod intents_game;
pub mod verification;

use crate::client::{IrisClient, proof_game};
use crate::generated::routes::requests::agent_proof_game::{
    ProofGameRequest, ProofGameRequestOrdersItem, ProofGameResponse,
};
use crate::wallet::types::WalletError;

/// Execute the prove game with a list of intents.
///
/// This helper converts the intents into a proper ProofGameRequest
/// and calls the proof_game API endpoint.
pub async fn proof_game_with_intents(
    _session_id: String,
    intents: Vec<ProofGameRequestOrdersItem>,
    client: &IrisClient,
) -> Result<ProofGameResponse, WalletError> {
    // Get the wallet from game state
    let wallet = game_state::get_or_create_wallet(false).map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Create the encrypted wallet blob (empty for prove game)
    let encrypted_wallet_blob = String::new();

    let request = ProofGameRequest {
        chain_id: erato::models::ChainId::ETHEREUM.to_string(),
        encrypted_wallet_blob,
        orders: intents,
        wallet_address: wallet.address,
    };

    proof_game(&request, client).await
}

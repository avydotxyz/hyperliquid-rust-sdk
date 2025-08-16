use hyperliquid_rust_sdk::{
    BaseUrl, InfoClient, HashGenerator, PerpDexSchemaInput, Error,
};

// Set to true to register a new perp dex.
const REGISTER_PERP_DEX: bool = false;
const DUMMY_DEX: &str = "test";

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    // Initialize info client for testnet
    let info_client = InfoClient::new(BaseUrl::Testnet);

    // Step 1: Get perp deploy auction status which includes auction start and gas information
    let perp_deploy_auction_status = info_client.query_perp_deploy_auction_status().await?;
    println!("Perp deploy auction status: {:#}", perp_deploy_auction_status);

    // Step 2: Registering a Perp Dex and Assets
    //
    // Takes part in the perp deploy auction and if successful, registers asset "TEST0".
    // The max gas is 10k HYPE and represents the max amount to be paid for the perp deploy auction.
    // Registering an asset can be done multiple times.
    let perp_dex_schema_input = if REGISTER_PERP_DEX {
        Some(PerpDexSchemaInput {
            full_name: "test dex".to_string(),
            collateral_token: 0,
            oracle_updater: Some("0x0000000000000000000000000000000000000000".to_string()), // Replace with actual address
        })
    } else {
        None
    };

    let register_asset_result = HashGenerator::perp_deploy_register_asset(
        DUMMY_DEX.to_string(),
        Some(1000000000000), // max_gas
        format!("{}:TEST0", DUMMY_DEX),
        2, // sz_decimals
        "10.0".to_string(), // oracle_px
        10, // margin_table_id
        false, // only_isolated
        perp_dex_schema_input,
    ).await?;
    
    println!("Register asset result: {:#?}", register_asset_result);
    // If registration is successful, the "dex" that was used can serve as the index into this clearinghouse for later asset
    // registrations and oracle updates.

    // Step 3: Set the Oracle Prices
    //
    // Oracle updates can be sent multiple times
    let oracle_pxs = vec![
        (format!("{}:TEST0", DUMMY_DEX), "12.0".to_string()),
        (format!("{}:TEST1", DUMMY_DEX), "1.0".to_string()),
    ];
    
    let mark_pxs = vec![
        vec![
            (format!("{}:TEST1", DUMMY_DEX), "3.0".to_string()),
            (format!("{}:TEST0", DUMMY_DEX), "14.0".to_string()),
        ]
    ];

    let set_oracle_result = HashGenerator::perp_deploy_set_oracle(
        DUMMY_DEX.to_string(),
        oracle_pxs,
        mark_pxs,
    ).await?;
    
    println!("Set oracle result: {:#?}", set_oracle_result);

    // Step 4: Get DUMMY_DEX meta
    let dummy_dex_meta = info_client.meta(Some(DUMMY_DEX.to_string())).await?;
    println!("Dummy dex meta: {:#}", dummy_dex_meta);

    Ok(())
}

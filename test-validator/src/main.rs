use clap::Parser;
use solana_sdk::signer::Signer;
use solana_streamer::socket::SocketAddrSpace;
use solana_test_validator::TestValidatorGenesis;
use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::signal;
use solana_core::consensus::tower_storage::FileTowerStorage;
use solana_sdk::signature::{Keypair, read_keypair, write_keypair};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Parser, Debug)]
#[command(
    name = "solana-test-validator",
    about = "Solana Test Validator",
    long_about = "A standalone Solana test validator for local development and testing."
)]
struct Args {
    /// Path to the ledger directory (if not specified, uses ~/.solana-test-validator/ledger)
    #[arg(long, value_name = "PATH")]
    ledger_path: Option<PathBuf>,

    /// RPC port for the validator
    #[arg(long, value_name = "PORT", default_value_t = 0)]
    rpc_port: u16,

    /// Faucet address (host:port)
    #[arg(long, value_name = "HOST:PORT")]
    faucet_addr: Option<String>,

    /// Gossip host IP address
    #[arg(long, value_name = "IP", default_value = "127.0.0.1")]
    gossip_host: String,

    /// Gossip port
    #[arg(long, value_name = "PORT", default_value_t = 0)]
    gossip_port: u16,

    /// Enable UDP for TPU
    #[arg(long)]
    tpu_enable_udp: bool,

    /// Deletes existing ledger at path before starting
    #[arg(long, help = "Deletes existing ledger at path before starting")]
    reset_ledger: bool,

    /// Optional output path to save mint keypair
    #[arg(long, value_name = "PATH", help = "Optional output path to save mint keypair")]
    mint_output: Option<PathBuf>,

    /// Optional input path to load mint keypair from
    #[arg(long, value_name = "PATH", help = "Optional input path to load mint keypair from")]
    mint_input: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reduce logging verbosity and disable metrics
    solana_logger::setup_with_default("solana=warn");
    std::env::set_var("SOLANA_METRICS_CONFIG", "");

    let args = Args::parse();

    // Initialize TestValidatorGenesis with default configuration
    let mut genesis = TestValidatorGenesis::default();

    // Apply CLI configurations
    let default_ledger_path = std::path::PathBuf::from(
        std::env::home_dir().unwrap().join(".solana-test-validator/ledger"),
    );
    let ledger_path = args.ledger_path.unwrap_or(default_ledger_path);

    // Handle --reset-ledger
    if args.reset_ledger && ledger_path.exists() {
        println!("Resetting ledger at {}", ledger_path.display());
        fs::remove_dir_all(&ledger_path)?;
    }
    fs::create_dir_all(&ledger_path)?;
    genesis.ledger_path(&ledger_path);

    // Configure file-based tower storage
    let tower_path = ledger_path.join("tower");
    genesis.tower_storage(Arc::new(FileTowerStorage::new(tower_path)));

    if args.rpc_port != 0 {
        genesis.rpc_port(args.rpc_port);
    }

    if let Some(faucet_addr) = args.faucet_addr {
        let socket_addr = SocketAddr::from_str(&faucet_addr)?;
        genesis.faucet_addr(Some(socket_addr));
    }

    let gossip_host = IpAddr::from_str(&args.gossip_host)?;
    genesis.gossip_host(gossip_host);

    if args.gossip_port != 0 {
        genesis.gossip_port(args.gossip_port);
    }

    // Force UDP to reduce QUIC-related metrics
    genesis.tpu_enable_udp(true);
    // Reduce ledger resource usage
    genesis.max_ledger_shreds = Some(1000);

    // Use default RPC config for test
    genesis.rpc_config(solana_rpc::rpc::JsonRpcConfig::default_for_test());

    // Load or generate the mint keypair
    let mint_keypair = if let Some(mint_input) = &args.mint_input {
        println!("Loading mint keypair from {}", mint_input.display());
        let file = File::open(mint_input).map_err(|e| {
            format!(
                "Failed to open mint keypair file {}: {}",
                mint_input.display(),
                e
            )
        })?;
        read_keypair(&mut BufReader::new(file)).map_err(|e| {
            format!(
                "Failed to read mint keypair from {}: {}",
                mint_input.display(),
                e
            )
        })?
    } else {
        println!("Generating new mint keypair");
        Keypair::new()
    };

    // Save the mint keypair to the specified or default output path
    let mint_output_path = args
        .mint_output
        .clone()
        .unwrap_or_else(|| ledger_path.join("mint-keypair.json"));
    if let Some(parent) = mint_output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(&mint_output_path).map_err(|e| {
        format!(
            "Failed to create mint keypair file {}: {}",
            mint_output_path.display(),
            e
        )
    })?;
    write_keypair(&mint_keypair, &mut BufWriter::new(file)).map_err(|e| {
        format!(
            "Failed to write mint keypair to {}: {}",
            mint_output_path.display(),
            e
        )
    })?;
    println!("Mint keypair saved to: {}", mint_output_path.display());

    let mint_address = mint_keypair.pubkey();

    // Start the test validator
    println!("Starting Solana test validator with mint address: {}", mint_address);
    let (test_validator, _mint_keypair) = genesis
        .start_async_with_socket_addr_space(SocketAddrSpace::new(true))
        .await;

    // Log validator details
    println!("Validator started successfully!");
    println!("RPC URL: {}", test_validator.rpc_url());
    println!("RPC PubSub URL: {}", test_validator.rpc_pubsub_url());
    println!("TPU Address: {}", test_validator.tpu());
    println!("Gossip Address: {}", test_validator.gossip());
    println!("Vote Account Address: {}", test_validator.vote_account_address());

    // Wait for Ctrl+C to shut down
    signal::ctrl_c().await?;
    println!("Received Ctrl+C, shutting down validator...");

    // Explicitly join the validator to ensure clean shutdown
    test_validator.join();

    Ok(())
}
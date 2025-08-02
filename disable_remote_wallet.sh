#!/data/data/com.termux/files/usr/bin/bash

# Disable solana_remote_wallet and RemoteWalletManager in CLI code
echo "🔧 Disabling remote wallet integration for Termux build..."

FILES=$(grep -rl "RemoteWalletManager" cli/src)

for file in $FILES; do
  echo "Patching $file..."

  # Remove 'use solana_remote_wallet...' lines
  sed -i '/solana_remote_wallet/d' "$file"

  # Comment lines using RemoteWalletManager in function signatures or logic
  sed -i 's/RemoteWalletManager/REMOVED_RemoteWalletManager/g' "$file"
done

# Remove crate dependency in Cargo.toml
echo "🔧 Updating Cargo.toml..."
sed -i '/solana-remote-wallet/d' cli/Cargo.toml

echo "✅ Remote wallet logic removed. You can now build safely in Termux!"
echo ""
echo "👉 Run:"
echo "cargo build --release -p solana-cli"
echo "cargo build --release -p solana-test-validator"

#!/data/data/com.termux/files/usr/bin/bash

echo "🛠️  Stripping RemoteWalletManager and solana_remote_wallet references..."

# Step 1: Remove all import lines of solana_remote_wallet
grep -rl "solana_remote_wallet" cli/src | while read file; do
  echo "Cleaning import in $file"
  sed -i '/solana_remote_wallet/d' "$file"
done

# Step 2: Remove all function parameters or types using RemoteWalletManager
grep -rl "RemoteWalletManager" cli/src | while read file; do
  echo "Patching $file"

  # Remove any line with the type RemoteWalletManager in function signatures
  sed -i '/RemoteWalletManager/d' "$file"

  # Remove generics like <RemoteWalletManager> and Rc<RemoteWalletManager>
  sed -i 's/<[^>]*RemoteWalletManager[^>]*>//g' "$file"
  sed -i 's/Rc<RemoteWalletManager>//g' "$file"
  sed -i 's/Option<Rc<RemoteWalletManager>>//g' "$file"
done

# Step 3: Clean Cargo.toml
echo "📦 Cleaning Cargo.toml..."
sed -i '/solana-remote-wallet/d' cli/Cargo.toml

echo "✅ Done! Try rebuilding with:"
echo "   cargo build --release -p solana-cli"

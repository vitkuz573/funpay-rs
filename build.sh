#!/bin/bash
set -e
cd "$(dirname "$0")"

# Generate from spec using specgen
cargo run --manifest-path ../specgen/Cargo.toml -- generate \
    --spec ../funpay-spec/spec/funpay.yaml \
    --target rust \
    --output .

# Append hand-written module declarations to lib.rs
for mod in cookies middleware retry auth search ua monitor stream export ws; do
    grep -q "pub mod ${mod};" src/lib.rs 2>/dev/null || \
        echo "pub mod ${mod};" >> src/lib.rs
done

# Add extra dependencies needed by hand-written modules
if ! grep -q 'rand = "0.8"' Cargo.toml; then
    sed -i '/^\[dependencies\]/a rand = "0.8"\nrust_decimal = "1"\nfutures-util = "0.3"\ntokio-tungstenite = { version = "0.24", features = ["native-tls"] }\nasync-stream = "0.3"\nfutures = "0.3"\ncsv = "1"' Cargo.toml
fi

# Verify hand-written modules exist
for f in cookies middleware retry auth search ua monitor stream export ws; do
    [ -f src/${f}.rs ] || echo "Warning: src/${f}.rs not found"
done

echo "✅ Generated SDK ready"


#!/bin/bash

if [ -z "$1" ]; then
  echo "❌ Please provide a base name (e.g., todo)"
  exit 1
fi

BASE_NAME="$1"

# Get current working directory (assumed to be ecommerce_backend root)
CURRENT_DIR="$(pwd)"

# Build path to struct file relative to current dir
STRUCT_FILE="$CURRENT_DIR/src/domain/models/${BASE_NAME}.rs"

if [ ! -f "$STRUCT_FILE" ]; then
  echo "❌ Struct file not found at $STRUCT_FILE"
  exit 1
fi

# Change to codegen directory relative to current dir
CODEGEN_DIR="$CURRENT_DIR/codegen"

if ! cd "$CODEGEN_DIR"; then
  echo "❌ Failed to enter codegen directory at $CODEGEN_DIR"
  exit 1
fi

cargo run -- "$BASE_NAME" --struct-file "$STRUCT_FILE"

sleep 1

# Return to ecommerce_backend root directory
cd "$CURRENT_DIR"

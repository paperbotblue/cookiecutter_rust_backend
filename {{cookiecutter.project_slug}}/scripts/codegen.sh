
#!/bin/bash

if [ -z "$1" ]; then
  echo "❌ Please provide a module name (e.g., auth)"
  exit 1
fi

if [ -z "$2" ]; then
  echo "❌ Please provide a base name (e.g., user)"
  exit 1
fi

read -p "Enter module name (lowercase): " MODULE_NAME
read -p "Enter model name (lowercase): " BASE_NAME

# Get current working directory 
CURRENT_DIR="$(pwd)"

# Build path to struct file relative to current dir
STRUCT_FILE="$CURRENT_DIR/src/modules/${MODULE_NAME}/src/domain/models/${BASE_NAME}.rs"
MODULE_DIR="$CURRENT_DIR/src/modules/${MODULE_NAME}"


if [ ! -d "$MODULE_DIR" ]; then
  echo "❌ MODULE DIR not found at $MODULE_DIR"
  exit 1
fi
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

cargo run -- --struct-file "$STRUCT_FILE" "$BASE_NAME" "$MODULE_NAME"

sleep 1

# Return to ecommerce_backend root directory
cd "$CURRENT_DIR"

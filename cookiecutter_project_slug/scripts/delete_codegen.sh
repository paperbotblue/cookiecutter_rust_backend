#!/bin/bash

# Get model name from user input
read -p "Enter model name (lowercase): " model_name

# Confirm deletion
read -p "This will delete all files related to $model_name. Continue? (y/n) " confirm
if [[ $confirm != "y" && $confirm != "Y" ]]; then
    echo "Aborted."
    exit 1
fi

# Delete files
echo "Deleting files..."
rm -v src/domain/errors/${model_name}_errors.rs
rm -v src/domain/services/${model_name}.rs
rm -v src/domain/repositories/${model_name}.rs
rm -v src/infrastructure/repositories/${model_name}.rs
rm -v src/infrastructure/models/${model_name}.rs
rm -v src/services/${model_name}.rs
rm -v src/api/controllers/${model_name}_handler.rs
rm -v src/api/dto/${model_name}.rs
rm -v src/api/dto/validators/${model_name}.rs
rm -v src/api/mappers/${model_name}.rs
rm -v src/api/routes/${model_name}.rs

# Clean up mod.rs files
echo "Cleaning up mod.rs references..."
sed -i "/pub mod ${model_name}_errors;/d" src/domain/errors/mod.rs
sed -i "/pub mod ${model_name};/d" src/domain/services/mod.rs
sed -i "/pub mod ${model_name};/d" src/domain/repositories/mod.rs
sed -i "/pub mod ${model_name};/d" src/infrastructure/repositories/mod.rs
sed -i "/pub mod ${model_name};/d" src/infrastructure/models/mod.rs
sed -i "/pub mod ${model_name};/d" src/services/mod.rs
sed -i "/pub mod ${model_name}_handler;/d" src/api/controllers/mod.rs
sed -i "/pub mod ${model_name};/d" src/api/dto/mod.rs
sed -i "/pub mod ${model_name};/d" src/api/dto/validators/mod.rs
sed -i "/pub mod ${model_name};/d" src/api/mappers/mod.rs
sed -i "/pub mod ${model_name};/d" src/api/routes/mod.rs

echo "Cleanup complete for model: $model_name"

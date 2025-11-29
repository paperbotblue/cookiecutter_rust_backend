#!/bin/bash

# Get model name from user input
read -p "Enter module name (lowercase): " module_name
read -p "Enter model name (lowercase): " model_name

# Confirm deletion
read -p "This will delete all files related to $model_name. Continue? (y/n) " confirm
if [[ $confirm != "y" && $confirm != "Y" ]]; then
    echo "Aborted."
    exit 1
fi

# Delete files
echo "Deleting files..."
rm -v src/modules/${module_name}/src/domain/errors/${model_name}_errors.rs
rm -v src/modules/${module_name}/src/domain/services/${model_name}.rs
rm -v src/modules/${module_name}/src/domain/repositories/${model_name}.rs
rm -v src/modules/${module_name}/src/infrastructure/repositories/${model_name}.rs
rm -v src/modules/${module_name}/src/infrastructure/services/${model_name}.rs
rm -v src/modules/${module_name}/src/api/controllers/${model_name}_handler.rs
rm -v src/modules/${module_name}/src/api/dto/${model_name}.rs
rm -v src/modules/${module_name}/src/api/mappers/${model_name}.rs
rm -v src/app/src/routers/${module_name}/${model_name}.rs

# Clean up mod.rs files
echo "Cleaning up mod.rs references..."
sed -i "/pub mod ${model_name}_errors;/d" src/modules/${module_name}/src/domain/errors/mod.rs
sed -i "/pub mod ${model_name};/d" src/modules/${module_name}/src/domain/services/mod.rs
sed -i "/pub mod ${model_name};/d" src/modules/${module_name}/src/domain/repositories/mod.rs
sed -i "/pub mod ${model_name};/d" src/modules/${module_name}/src/infrastructure/repositories/mod.rs
sed -i "/pub mod ${model_name};/d" src/modules/${module_name}/src/infrastructure/services/mod.rs
sed -i "/pub mod ${model_name}_handler;/d" src/modules/${module_name}/src/api/controllers/mod.rs
sed -i "/pub mod ${model_name};/d" src/modules/${module_name}/src/api/dto/mod.rs
sed -i "/pub mod ${model_name};/d" src/modules/${module_name}/src/api/mappers/mod.rs
sed -i "/pub mod ${model_name};/d" src/app/src/routers/${module_name}/mod.rs

echo "Cleanup complete for model: $model_name"

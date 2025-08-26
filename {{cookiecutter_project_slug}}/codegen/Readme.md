
# for using codegen
diesel migration generate create_<entity_name>
    -- create up.sql and down.sql
diesel migration run
diesel print-schema > src/infrastructure/schema.rs
    -- create <entity_name>.rs in domain/models/<entity_name>.rs
./scripts/codegen.sh <entity_name>
    -- create app changes

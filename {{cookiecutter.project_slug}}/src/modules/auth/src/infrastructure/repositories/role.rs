use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::role::RoleQueryParams;
use crate::domain::repositories::role::RoleRepository;
use base::result_paging::{QueryParams, RepositoryResult, ResultPaging};

pub struct RoleDieselRepository {
    pub pool: PgPool,
}

impl RoleDieselRepository {
    pub fn new(pool: PgPool) -> Self {
        RoleDieselRepository { pool }
    }
}

#[async_trait]
impl RoleRepository for RoleDieselRepository {
    async fn create(&self, new_item: &CreateRole) -> RepositoryResult<Role> {
        Ok(query_as!(
            Role,
            r#"
                INSERT INTO roles 
                    (id, name, description) 
                VALUES
                    ($1, $2, $3)
                RETURNING *
            "#,
            Uuid::new_v4(),
            new_item.name,
            new_item.description
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update(&self, new_item: &UpdateRole) -> RepositoryResult<Role> {
        Ok(query_as!(
            Role,
            r#"
                UPDATE roles
                SET 
                    name = $2, 
                    description = $3
                WHERE 
                    id = $1 
                RETURNING *
            "#,
            new_item.id,
            new_item.name,
            new_item.description
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn list(&self, params: RoleQueryParams) -> RepositoryResult<ResultPaging<Role>> {
        let result = query_as!(
            Role,
            r#"
                SELECT *
                FROM roles
                LIMIT $1 OFFSET $2
            "#,
            params.limit(),
            params.offset()
        )
        .fetch_all(&self.pool)
        .await?;

        let total = query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM roles
        "#
        )
        .fetch_one(&self.pool)
        .await?
        .count;

        Ok(ResultPaging {
            total,
            items: result,
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<Role>> {
        Ok(query_as!(
            Role,
            r#"
                SELECT * FROM roles 
                WHERE
                id = $1
            "#,
            item_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn get_by_role_name(&self, role_name: String) -> RepositoryResult<Option<Role>> {
        Ok(query_as!(
            Role,
            r#"
                SELECT * FROM roles 
                WHERE name = $1 
            "#,
            role_name
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn get_all_permissions_by_role_id(
        &self,
        role_id_val: Uuid,
    ) -> RepositoryResult<Vec<Permission>> {
        Ok(query_as!(
            Permission,
            r#"
                SELECT p.id, p.name, p.description 
                FROM permissions AS p 
                INNER JOIN role_permissions AS rp 
                    ON p.id = rp.permission_id
                WHERE rp.role_id = $1
            "#,
            role_id_val
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn is_role_authorized(
        &self,
        role_name: String,
        permission_name: String,
    ) -> RepositoryResult<bool> {
        let row = query!(
            r#"
            SELECT 
                roles.id              AS role_id,
                roles.name            AS role_name,
                roles.description     AS role_description,
                permissions.id        AS permission_id,
                permissions.name      AS permission_name,
                permissions.description AS permission_description
            FROM roles
            INNER JOIN role_permissions 
                ON role_permissions.role_id = roles.id
            INNER JOIN permissions
                ON permissions.id = role_permissions.permission_id
            WHERE 
                roles.name = $1
            AND
                permissions.name = $2
            LIMIT 1
        "#,
            role_name,
            permission_name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.is_some())
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
                DELETE FROM roles WHERE id = $1
            "#,
            item_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

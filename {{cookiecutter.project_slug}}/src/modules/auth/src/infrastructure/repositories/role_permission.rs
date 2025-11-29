use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::role_permission::RolePermissionQueryParams;
use crate::domain::repositories::role_permission::RolePermissionRepository;
use base::result_paging::{QueryParams, RepositoryResult, ResultPaging};

pub struct RolePermissionDieselRepository {
    pub pool: PgPool,
}

impl RolePermissionDieselRepository {
    pub fn new(pool: PgPool) -> Self {
        RolePermissionDieselRepository { pool }
    }
}

#[async_trait]
impl RolePermissionRepository for RolePermissionDieselRepository {
    async fn create(&self, new_item: &CreateRolePermission) -> RepositoryResult<RolePermission> {
        Ok(query_as!(
            RolePermission,
            r#"
                INSERT INTO role_permissions 
                    (role_id, permission_id, description) 
                VALUES
                    ($1, $2, $3)
                RETURNING *
            "#,
            new_item.role_id,
            new_item.permission_id,
            new_item.description
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn list(
        &self,
        params: RolePermissionQueryParams,
    ) -> RepositoryResult<ResultPaging<RolePermission>> {
        let result = query_as!(
            RolePermission,
            r#"
                SELECT * FROM role_permissions LIMIT $1 OFFSET $2
            "#,
            params.limit(),
            params.offset()
        )
        .fetch_all(&self.pool)
        .await?;

        let total = query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM role_permissions
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

    async fn get(
        &self,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> RepositoryResult<Option<RolePermission>> {
        Ok(query_as!(
            RolePermission,
            r#"
                SELECT * FROM role_permissions 
                WHERE 
                role_id = $1 AND permission_id = $2
            "#,
            role_id,
            permission_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn delete(&self, role_id: Uuid, permission_id: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
            DELETE FROM role_permissions 
            WHERE 
            role_id = $1 AND permission_id = $2
        "#,
            role_id,
            permission_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

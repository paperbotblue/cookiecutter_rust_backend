use async_trait::async_trait;
use sqlx::{query, query_as, PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::repositories::permission::PermissionRepository;
use base::result_paging::{QueryParams, RepositoryResult, ResultPaging};

pub struct PermissionDbRepository {
    pub pool: PgPool,
}

impl PermissionDbRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PermissionDbRepository { pool }
    }
}

#[async_trait]
impl PermissionRepository for PermissionDbRepository {
    async fn create(&self, new_item: CreatePermission) -> RepositoryResult<Permission> {
        Ok(query_as!(
            Permission,
            r#"
                INSERT INTO permissions 
                    (id, name, description) 
                VALUES 
                    ($1, $2, $3) 
                RETURNING *;
            "#,
            Uuid::new_v4(),
            new_item.name,
            new_item.description
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn update(&self, new_item: UpdatePermission) -> RepositoryResult<Permission> {
        Ok(query_as!(
            Permission,
            r#"
                UPDATE permissions
                SET
                    name = $2,
                    description = $3
                WHERE 
                    id = $1
                RETURNING *;            
            "#,
            new_item.id,
            new_item.name,
            new_item.description
        )
        .fetch_one(&self.pool)
        .await?)
    }

    async fn list(
        &self,
        params: PermissionQueryParams,
    ) -> RepositoryResult<ResultPaging<Permission>> {
        let result = query_as!(
            Permission,
            r#"
                SELECT *
                FROM permissions
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
        FROM permissions
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

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<Permission>> {
        Ok(query_as!(
            Permission,
            r#"
                SELECT * FROM permissions WHERE id = $1
            "#,
            item_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn get_by_name(&self, permission_name: String) -> RepositoryResult<Option<Permission>> {
        Ok(query_as!(
            Permission,
            r#"
                SELECT * FROM permissions WHERE name = $1
            "#,
            permission_name
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        query!(
            r#"
            DELETE FROM permissions WHERE id = $1
        "#,
            item_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

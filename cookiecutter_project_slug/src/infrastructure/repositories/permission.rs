use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::permission::{CreatePermission, Permission, UpdatePermission};
use crate::domain::repositories::permission::PermissionQueryParams;
use crate::domain::repositories::permission::PermissionRepository;
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::permission::{
    CreatePermissionDiesel, PermissionDiesel, UpdatePermissionDiesel,
};

pub struct PermissionDieselRepository {
    pub pool: Arc<DBConn>,
}

impl PermissionDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        PermissionDieselRepository { pool: db }
    }
}

#[async_trait]
impl PermissionRepository for PermissionDieselRepository {
    async fn create(&self, new_item: CreatePermission) -> RepositoryResult<Permission> {
        use crate::infrastructure::schema::permissions::dsl::permissions;
        let new_item_diesel: CreatePermissionDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: PermissionDiesel = run(move || {
            diesel::insert_into(permissions)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: UpdatePermission) -> RepositoryResult<Permission> {
        use crate::infrastructure::schema::permissions::dsl::{id as target_id, permissions};
        let id_val: Uuid = new_item.id;
        let new_item_diesel: UpdatePermissionDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: PermissionDiesel = run(move || {
            diesel::update(permissions.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(
        &self,
        params: PermissionQueryParams,
    ) -> RepositoryResult<ResultPaging<Permission>> {
        use crate::infrastructure::schema::permissions::dsl::permissions;
        let pool = self.pool.clone();
        let builder = permissions.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<PermissionDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<Permission>> {
        use crate::infrastructure::schema::permissions::dsl::{id, permissions};
        let mut conn = self.pool.get().unwrap();

        run(move || {
            permissions
                .filter(id.eq(item_id))
                .first::<PermissionDiesel>(&mut conn)
                .optional() // 👈 converts NotFound into Ok(None)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into())) // map over Option
    }

    async fn get_by_name(&self, permission_name: String) -> RepositoryResult<Option<Permission>> {
        use crate::infrastructure::schema::permissions::dsl::{name, permissions};
        let mut conn = self.pool.get().unwrap();

        run(move || {
            permissions
                .filter(name.eq(permission_name))
                .first::<PermissionDiesel>(&mut conn)
                .optional() // 👈 converts NotFound into Ok(None)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into())) // map over Option
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::permissions::dsl::{id, permissions};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(permissions.filter(id.eq(item_id))).execute(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }
}

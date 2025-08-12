use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::role_permission::{CreateRolePermission, RolePermission};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::role_permission::RolePermissionQueryParams;
use crate::domain::repositories::role_permission::RolePermissionRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::role_permission::{
    CreateRolePermissionDiesel, RolePermissionDiesel,
};

pub struct RolePermissionDieselRepository {
    pub pool: Arc<DBConn>,
}

impl RolePermissionDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        RolePermissionDieselRepository { pool: db }
    }
}

#[async_trait]
impl RolePermissionRepository for RolePermissionDieselRepository {
    async fn create(&self, new_item: &CreateRolePermission) -> RepositoryResult<RolePermission> {
        use crate::infrastructure::schema::role_permissions::dsl::role_permissions;
        let new_item_diesel: CreateRolePermissionDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: RolePermissionDiesel = run(move || {
            diesel::insert_into(role_permissions)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(
        &self,
        params: RolePermissionQueryParams,
    ) -> RepositoryResult<ResultPaging<RolePermission>> {
        use crate::infrastructure::schema::role_permissions::dsl::role_permissions;
        let pool = self.pool.clone();
        let builder = role_permissions
            .limit(params.limit())
            .offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<RolePermissionDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id1: Uuid, item_id2: Uuid) -> RepositoryResult<RolePermission> {
        use crate::infrastructure::schema::role_permissions::dsl::{
            permission_id, role_id, role_permissions,
        };
        let mut conn = self.pool.get().unwrap();
        run(move || {
            role_permissions
                .filter(role_id.eq(item_id1))
                .filter(permission_id.eq(item_id2))
                .first::<RolePermissionDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|v| v.into())
    }

    async fn delete(&self, item_id1: Uuid, item_id2: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::role_permissions::dsl::{
            permission_id, role_id, role_permissions,
        };
        let mut conn = self.pool.get().unwrap();
        run(move || {
            diesel::delete(
                role_permissions
                    .filter(role_id.eq(item_id1))
                    .filter(permission_id.eq(item_id2)),
            )
            .execute(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }
}

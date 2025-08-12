use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::permission::Permission;
use crate::domain::models::role::{CreateRole, Role, UpdateRole};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::role::RoleQueryParams;
use crate::domain::repositories::role::RoleRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::permission::PermissionDiesel;
use crate::infrastructure::models::role::{CreateRoleDiesel, RoleDiesel, UpdateRoleDiesel};

pub struct RoleDieselRepository {
    pub pool: Arc<DBConn>,
}

impl RoleDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        RoleDieselRepository { pool: db }
    }
}

#[async_trait]
impl RoleRepository for RoleDieselRepository {
    async fn create(&self, new_item: &CreateRole) -> RepositoryResult<Role> {
        use crate::infrastructure::schema::roles::dsl::roles;
        let new_item_diesel: CreateRoleDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: RoleDiesel = run(move || {
            diesel::insert_into(roles)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: &UpdateRole) -> RepositoryResult<Role> {
        use crate::infrastructure::schema::roles::dsl::{id as target_id, roles};
        let new_item_diesel: UpdateRoleDiesel = new_item.into();
        let id_val: Uuid = new_item.id;
        let mut conn = self.pool.get().unwrap();
        let result: RoleDiesel = run(move || {
            diesel::update(roles.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: RoleQueryParams) -> RepositoryResult<ResultPaging<Role>> {
        use crate::infrastructure::schema::roles::dsl::roles;
        let pool = self.pool.clone();
        let builder = roles.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<RoleDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Role> {
        use crate::infrastructure::schema::roles::dsl::{id, roles};
        let mut conn = self.pool.get().unwrap();
        run(move || roles.filter(id.eq(item_id)).first::<RoleDiesel>(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())
            .map(|v| v.into())
    }

    async fn get_all_permissions_by_role_id(
        &self,
        role_id_val: Uuid,
    ) -> RepositoryResult<Vec<Permission>> {
        use crate::infrastructure::schema::{
            permissions::dsl::{id as permission_id, permissions as permissions_dsl},
            role_permissions::dsl::{
                permission_id as rp_permission_id, role_id as rp_role_id,
                role_permissions as rp_dsl,
            },
        };
        use diesel::prelude::*;
        let mut conn = self.pool.get().unwrap();

        run(move || {
            permissions_dsl
                .inner_join(rp_dsl.on(permission_id.eq(rp_permission_id)))
                .filter(rp_role_id.eq(role_id_val))
                .select(permissions_dsl::all_columns())
                .load::<PermissionDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|results| results.into_iter().map(|p| p.into()).collect())
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::roles::dsl::{id, roles};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(roles.filter(id.eq(item_id))).execute(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }

    async fn has_permission(&self, r_name: &str, p_name: &str) -> RepositoryResult<bool> {
        use crate::infrastructure::schema::{
            permissions::dsl::{
                id as permission_id, name as permission_name, permissions as permissions_dsl,
            },
            role_permissions::dsl::{
                permission_id as rp_permission_id, role_id as rp_role_id,
                role_permissions as rp_dsl,
            },
            roles::dsl::{id as role_id, name as role_name, roles as roles_dsl},
        };
        use diesel::prelude::*;
        let mut conn = self.pool.get().unwrap();

        // Clone inputs into owned Strings so they can be moved into the closure
        let r_name = r_name.to_string();
        let p_name = p_name.to_string();

        let result = run(move || {
            roles_dsl
                .inner_join(rp_dsl.on(role_id.eq(rp_role_id)))
                .inner_join(permissions_dsl.on(permission_id.eq(rp_permission_id)))
                .filter(role_name.eq(&r_name))
                .filter(permission_name.eq(&p_name))
                .select(role_id)
                .first::<uuid::Uuid>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(result.is_some())
    }
}

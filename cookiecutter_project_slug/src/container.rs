use crate::{
    domain::{
        repositories::{
            permission::PermissionRepository, role::RoleRepository,
            role_permission::RolePermissionRepository, token::TokenRepository,
        },
        services::{
            permission::PermissionService, role::RoleService,
            role_permission::RolePermissionService, token::TokenService,
        },
    },
    infrastructure::{
        databases::postgresql::db_pool,
        repositories::{
            permission::PermissionDieselRepository, role::RoleDieselRepository,
            role_permission::RolePermissionDieselRepository, token::TokenDieselRepository,
        },
    },
    services::{
        permission::PermissionServiceImpl, role::RoleServiceImpl,
        role_permission::RolePermissionServiceImpl, token::TokenServiceImpl,
    },
};
use std::sync::Arc;

pub struct Container {
    pub token_service: Arc<dyn TokenService>,
    pub role_service: Arc<dyn RoleService>,
    pub permission_service: Arc<dyn PermissionService>,
    pub role_permission_service: Arc<dyn RolePermissionService>,
}

impl Container {
    pub fn new() -> Self {
        let pool = Arc::new(db_pool());

        let token_service: Arc<dyn TokenRepository> =
            Arc::new(TokenDieselRepository::new(pool.clone()));
        let token_service = Arc::new(TokenServiceImpl {
            repository: token_service,
        });

        let permission_service: Arc<dyn PermissionRepository> =
            Arc::new(PermissionDieselRepository::new(pool.clone()));
        let permission_service = Arc::new(PermissionServiceImpl {
            repository: permission_service,
        });

        let role_service: Arc<dyn RoleRepository> =
            Arc::new(RoleDieselRepository::new(pool.clone()));
        let role_service = Arc::new(RoleServiceImpl {
            repository: role_service,
        });

        let role_permission_service: Arc<dyn RolePermissionRepository> =
            Arc::new(RolePermissionDieselRepository::new(pool.clone()));
        let role_permission_service = Arc::new(RolePermissionServiceImpl {
            repository: role_permission_service,
        });

        Container {
            role_permission_service,
            role_service,
            permission_service,
            token_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

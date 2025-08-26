use crate::{
    domain::{
        repositories::{
            permission::PermissionRepository, role::RoleRepository,
            role_permission::RolePermissionRepository, todo::TodoRepository, user::UserRepository,
        },
        services::{
            permission::PermissionService, role::RoleService,
            role_permission::RolePermissionService, todo::TodoService, user::UserService,
        },
    },
    infrastructure::{
        databases::postgresql::db_pool,
        repositories::{
            permission::PermissionDieselRepository, role::RoleDieselRepository,
            role_permission::RolePermissionDieselRepository, todo::TodoDieselRepository,
            user::UserDieselRepository,
        },
    },
    services::{
        permission::PermissionServiceImpl, role::RoleServiceImpl,
        role_permission::RolePermissionServiceImpl, todo::TodoServiceImpl, user::UserServiceImpl,
    },
};
use std::sync::Arc;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub role_service: Arc<dyn RoleService>,
    pub permission_service: Arc<dyn PermissionService>,
    pub role_permission_service: Arc<dyn RolePermissionService>,
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub fn new() -> Self {
        let pool = Arc::new(db_pool());

        let user_service: Arc<dyn UserRepository> =
            Arc::new(UserDieselRepository::new(pool.clone()));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_service,
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

        let todo_service: Arc<dyn TodoRepository> =
            Arc::new(TodoDieselRepository::new(pool.clone()));
        let todo_service = Arc::new(TodoServiceImpl {
            repository: todo_service,
        });

        Container {
            role_permission_service,
            role_service,
            permission_service,
            todo_service,
            user_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

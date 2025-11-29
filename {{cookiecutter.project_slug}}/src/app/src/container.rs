use std::sync::Arc;

use auth::{
    domain::services::{
        permission::PermissionService, refresh_token::RefreshTokenService, role::RoleService,
        role_permission::RolePermissionService, user::UserService,
    },
    infrastructure::services::{
        permission::PermissionServiceImpl, refresh_token::RefreshTokenServiceImpl,
        role::RoleServiceImpl, role_permission::RolePermissionServiceImpl, user::UserServiceImpl,
    },
};

pub struct Container {
    pub user_service: Arc<dyn UserService>,
    pub permission_service: Arc<dyn PermissionService>,
    pub role_service: Arc<dyn RoleService>,
    pub role_permission_service: Arc<dyn RolePermissionService>,
    pub refresh_token_service: Arc<dyn RefreshTokenService>,
    // pub school_service: Arc<dyn SchoolService>,
    // pub user_address_service: Arc<dyn UserAddressService>,
    // pub user_bank_detail_service: Arc<dyn UserBankDetailService>,
    // pub caste_service: Arc<dyn CasteService>,
    // pub religion_service: Arc<dyn ReligionService>,
    // pub department_service: Arc<dyn DepartmentService>,
    // pub designation_service: Arc<dyn DesignationService>,
    // pub staff_service: Arc<dyn StaffService>,
    // pub attendance_service: Arc<dyn AttendanceService>,
    // pub leave_type_service: Arc<dyn LeaveTypeService>,
    // pub leave_request_service: Arc<dyn LeaveRequestService>,
    // pub student_service: Arc<dyn StudentService>,
    // pub class_service: Arc<dyn ClassService>,
}

impl Container {
    pub async fn new() -> Self {
        let sqlx_pool = base::sqlx_init::db_conn().await;

        let permission_service = Arc::new(PermissionServiceImpl::new(sqlx_pool.clone()));
        let role_service = Arc::new(RoleServiceImpl::new(sqlx_pool.clone()));
        let role_permission_service = Arc::new(RolePermissionServiceImpl::new(sqlx_pool.clone()));
        let user_service = Arc::new(UserServiceImpl::new(sqlx_pool.clone()));
        let refresh_token_service = Arc::new(RefreshTokenServiceImpl::new(sqlx_pool.clone()));

        //
        // let caste_repo: Arc<dyn CasteRepository> =
        //     Arc::new(CasteDieselRepository::new(sqlx_pool.clone()));
        // let caste_service = Arc::new(CasteServiceImpl {
        //     repository: caste_repo,
        // });
        //
        // let department_repo: Arc<dyn DepartmentRepository> =
        //     Arc::new(DepartmentDieselRepository::new(sqlx_pool.clone()));
        // let department_service = Arc::new(DepartmentServiceImpl {
        //     repository: department_repo,
        // });
        //

        //
        // let class_repo: Arc<dyn ClassRepository> =
        //     Arc::new(ClassDieselRepository::new(sqlx_pool.clone()));
        // let class_service = Arc::new(ClassServiceImpl {
        //     repository: class_repo,
        // });
        //
        // let designation_repo: Arc<dyn DesignationRepository> =
        //     Arc::new(DesignationDieselRepository::new(sqlx_pool.clone()));
        // let designation_service = Arc::new(DesignationServiceImpl {
        //     repository: designation_repo,
        // });
        //
        // let school_repo: Arc<dyn SchoolRepository> =
        //     Arc::new(SchoolDieselRepository::new(sqlx_pool.clone()));
        // let school_service = Arc::new(SchoolServiceImpl {
        //     repository: school_repo,
        // });
        //
        // let user_address_repo: Arc<dyn UserAddressRepository> =
        //     Arc::new(UserAddressDieselRepository::new(sqlx_pool.clone()));
        // let user_address_service = Arc::new(UserAddressServiceImpl {
        //     repository: user_address_repo,
        // });
        //
        // let leave_type_repo: Arc<dyn LeaveTypeRepository> =
        //     Arc::new(LeaveTypeDieselRepository::new(sqlx_pool.clone()));
        // let leave_type_service = Arc::new(LeaveTypeServiceImpl {
        //     repository: leave_type_repo,
        // });
        //
        // let user_bank_detail_repo: Arc<dyn UserBankDetailRepository> =
        //     Arc::new(UserBankDetailDieselRepository::new(sqlx_pool.clone()));
        // let user_bank_detail_service = Arc::new(UserBankDetailServiceImpl {
        //     repository: user_bank_detail_repo,
        // });
        //
        // let religion_repo: Arc<dyn ReligionRepository> =
        //     Arc::new(ReligionDieselRepository::new(sqlx_pool.clone()));
        // let religion_service = Arc::new(ReligionServiceImpl {
        //     repository: religion_repo,
        // });
        //

        //
        // let staff_repo: Arc<dyn StaffRepository> =
        //     Arc::new(StaffDieselRepository::new(sqlx_pool.clone()));
        // let staff_service = Arc::new(StaffServiceImpl {
        //     repository: staff_repo,
        // });
        //
        // let attendance_repo: Arc<dyn AttendanceRepository> =
        //     Arc::new(AttendanceDieselRepository::new(sqlx_pool.clone()));
        // let attendance_service = Arc::new(AttendanceServiceImpl {
        //     repository: attendance_repo,
        // });
        //
        // let student_repo: Arc<dyn StudentRepository> =
        //     Arc::new(StudentDieselRepository::new(sqlx_pool.clone()));
        // let student_service = Arc::new(StudentServiceImpl {
        //     repository: student_repo,
        // });
        //
        // let leave_request_repo: Arc<dyn LeaveRequestRepository> =
        //     Arc::new(LeaveRequestDieselRepository::new(sqlx_pool.clone()));
        // let leave_request_service = Arc::new(LeaveRequestServiceImpl {
        //     repository: leave_request_repo,
        // });

        Container {
            permission_service,
            role_service,
            role_permission_service,
            refresh_token_service,
            user_service,
            // school_service,
            // user_address_service,
            // user_bank_detail_service,
            // caste_service,
            // religion_service,
            // department_service,
            // designation_service,
            // staff_service,
            // attendance_service,
            // leave_type_service,
            // leave_request_service,
            // student_service,
            // class_service,
        }
    }
}

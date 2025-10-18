use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, PaginatorTrait,
};
use uuid::Uuid;
use chrono::Utc;
use crate::entities::user::{Entity as UserEntity, Model as UserModel, ActiveModel as UserActiveModel};
use crate::models::user::{CreateUser, UpdateUser, UserResponse};
use crate::utils::password::PasswordUtil;
use crate::errors::{AppError, Result};

#[derive(Clone)]
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, user_data: CreateUser) -> Result<UserResponse> {
        // Check if username already exists
        if self.find_by_username(&user_data.username).await?.is_some() {
            return Err(AppError::Validation("Username already exists".to_string()));
        }

        // Check if email already exists
        if self.find_by_email(&user_data.email).await?.is_some() {
            return Err(AppError::Validation("Email already exists".to_string()));
        }

        let hashed_password = PasswordUtil::hash_password(&user_data.password)?;
        let now = Utc::now();

        let user = UserActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(user_data.username),
            name: Set(user_data.name),
            email: Set(user_data.email),
            password: Set(hashed_password),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            deleted_at: Set(None),
        };

        let user = user.insert(&self.db).await?;
        Ok(UserResponse::from(user))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserResponse>> {
        let user = UserEntity::find_by_id(id)
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;

        Ok(user.map(UserResponse::from))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserResponse>> {
        let user = UserEntity::find()
            .filter(crate::entities::user::Column::Username.eq(username))
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;

        Ok(user.map(UserResponse::from))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserResponse>> {
        let user = UserEntity::find()
            .filter(crate::entities::user::Column::Email.eq(email))
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;

        Ok(user.map(UserResponse::from))
    }

    pub async fn find_by_username_with_password(&self, username: &str) -> Result<Option<UserModel>> {
        let user = UserEntity::find()
            .filter(crate::entities::user::Column::Username.eq(username))
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email_with_password(&self, email: &str) -> Result<Option<UserModel>> {
        let user = UserEntity::find()
            .filter(crate::entities::user::Column::Email.eq(email))
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?;

        Ok(user)
    }

    pub async fn update(&self, id: Uuid, user_data: UpdateUser) -> Result<UserResponse> {
        let mut user: UserActiveModel = UserEntity::find_by_id(id)
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?
            .into();

        if let Some(username) = user_data.username {
            // Check if new username already exists
            if let Some(existing_user) = self.find_by_username(&username).await? {
                if existing_user.id != id {
                    return Err(AppError::Validation("Username already exists".to_string()));
                }
            }
            user.username = Set(username);
        }

        if let Some(email) = user_data.email {
            // Check if new email already exists
            if let Some(existing_user) = self.find_by_email(&email).await? {
                if existing_user.id != id {
                    return Err(AppError::Validation("Email already exists".to_string()));
                }
            }
            user.email = Set(email);
        }

        if let Some(name) = user_data.name {
            user.name = Set(name);
        }

        if let Some(password) = user_data.password {
            let hashed_password = PasswordUtil::hash_password(&password)?;
            user.password = Set(hashed_password);
        }

        if let Some(is_active) = user_data.is_active {
            user.is_active = Set(is_active);
        }

        user.updated_at = Set(Utc::now());

        let user = user.update(&self.db).await?;
        Ok(UserResponse::from(user))
    }

    pub async fn soft_delete(&self, id: Uuid) -> Result<()> {
        let mut user: UserActiveModel = UserEntity::find_by_id(id)
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?
            .into();

        user.deleted_at = Set(Some(Utc::now()));
        user.updated_at = Set(Utc::now());

        user.update(&self.db).await?;
        Ok(())
    }

    pub async fn restore(&self, id: Uuid) -> Result<UserResponse> {
        let mut user: UserActiveModel = UserEntity::find_by_id(id)
            .filter(crate::entities::user::Column::DeletedAt.is_not_null())
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found or not deleted".to_string()))?
            .into();

        user.deleted_at = Set(None);
        user.updated_at = Set(Utc::now());

        let user = user.update(&self.db).await?;
        Ok(UserResponse::from(user))
    }

    pub async fn list(&self, page: u64, per_page: u64) -> Result<Vec<UserResponse>> {
        let users = UserEntity::find()
            .filter(crate::entities::user::Column::DeletedAt.is_null())
            .paginate(&self.db, per_page)
            .fetch_page(page)
            .await?;

        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn verify_password(&self, user: &UserModel, password: &str) -> Result<bool> {
        PasswordUtil::verify_password(password, &user.password)
    }
}

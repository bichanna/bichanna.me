//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub text: String,
    pub is_published: bool,
    pub created: String,
    pub published: Option<String>,
    pub updated: Option<String>,
}

impl ActiveModel {
    fn new(title: String, text: String) -> Self {
        Self {
            title: ActiveValue::Set(title),
            text: ActiveValue::Set(text),
            is_published: ActiveValue::Set(false),
            created: ActiveValue::Set(chrono::Local::now().to_string()),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "User")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub user_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_from_steam::Entity")]
    UserFromSteam,
}

impl Related<super::user_from_steam::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserFromSteam.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "store")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub store_id: i32,
    pub manager_staff_id: i16,
    pub address_id: i32,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::address::Entity",
        from = "Column::AddressId",
        to = "super::address::Column::AddressId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Address,
    #[sea_orm(has_many = "super::customer::Entity")]
    Customer,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
    #[sea_orm(
        belongs_to = "super::staff::Entity",
        from = "Column::ManagerStaffId",
        to = "super::staff::Column::StaffId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Staff,
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::address::Entity")]
    Address,
    #[sea_orm(entity = "super::customer::Entity")]
    Customer,
    #[sea_orm(entity = "super::inventory::Entity")]
    Inventory,
    #[sea_orm(entity = "super::staff::Entity")]
    Staff,
}

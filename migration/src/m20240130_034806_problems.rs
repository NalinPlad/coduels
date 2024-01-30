use std::borrow::BorrowMut;

use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Problems::Table)
                    .col(pk_auto(Problems::Id).borrow_mut())
                    .col(string(Problems::Title).borrow_mut())
                    .col(text_null(Problems::Description).borrow_mut())
                    .col(json(Problems::Inputs).borrow_mut())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Problems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
    Title,
    Description,
    Inputs,
    
}



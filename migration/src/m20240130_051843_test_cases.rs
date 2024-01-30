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
                table_auto(TestCases::Table)
                    .col(pk_auto(TestCases::Id).borrow_mut())
                    .col(integer(TestCases::ProblemId).borrow_mut())
                    .col(bool(TestCases::Hidden).borrow_mut())
                    .col(json(TestCases::Inputs).borrow_mut())
                    .col(text(TestCases::Answer).borrow_mut())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-test_cases-problems")
                            .from(TestCases::Table, TestCases::ProblemId)
                            .to(Problems::Table, Problems::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestCases::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TestCases {
    Table,
    Id,
    ProblemId,
    Hidden,
    Inputs,
    Answer,
    
}


#[derive(DeriveIden)]
enum Problems {
    Table,
    Id,
}

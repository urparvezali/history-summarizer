use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(
                        big_integer(Links::Id)
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(big_integer(Links::UserId).not_null())
                    .col(text(Links::Url).not_null())
                    .col(string_len(Links::Keywords, 100).null())
                    .col(
                        timestamp(Links::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_links_user")
                            .from(Links::Table, Links::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Links {
    Table,
    Id,
    UserId,
    Url,
    Keywords,
    CreatedAt,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

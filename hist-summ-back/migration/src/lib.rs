pub use sea_orm_migration::prelude::*;

mod m20250323_131216_users;
mod m20250323_145821_links;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250323_131216_users::Migration),
            Box::new(m20250323_145821_links::Migration),
        ]
    }
}

use async_graphql::*;

#[cfg(test)]
mod tests {
    use sea_orm::{
        entity::prelude::*, entity::*, tests_cfg::*,
        DatabaseBackend, MockDatabase, MockExecResult, Transaction,
    };

    #[async_std::test]
    async fn test_test() -> Result<(), DbErr> {
        println!("Hello world!");
        assert!(1 == 2);
        Ok(())
    }

    #[async_std::test]
    async fn test_insert_cake() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![
                MockExecResult {
                    last_insert_id: 15,
                    rows_affected: 1,
                },
            ])
            .into_connection();

        // Prepare the ActiveModel
        let apple = cake::ActiveModel {
            name: Set("Apple Pie".to_owned()),
            ..Default::default()
        };

        // Insert the ActiveModel into MockDatabase
        let insert_result = apple.insert(&db).await?;

        // Checking last insert id
        // assert_eq!(insert_result.last_insert_id, 15);

        // Checking transaction log
        // assert_eq!(
        //     db.into_transaction_log(),
        //     vec![
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"INSERT INTO "cake" ("name") VALUES ($1)"#,
        //             vec!["Apple Pie".into()]
        //         ),
        //     ]
        // );

        Ok(())
    }
}


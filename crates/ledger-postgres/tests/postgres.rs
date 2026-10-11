use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};

use caudal_ledger::testing::{clock, op, run};
use caudal_ledger::{EntryQuery, Ledger, MemLedger};
use caudal_ledger_postgres::{PgLedger, migrate};
use proptest::prelude::*;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::{AssertSqlSafe, Executor};

struct TestDb {
    admin: PgPool,
    name: String,
    runtime: PgPool,
}

impl TestDb {
    async fn create() -> Option<Self> {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        let url = std::env::var("DATABASE_URL").ok()?;
        let admin = PgPool::connect(&url).await.expect("DATABASE_URL is reachable: run `just db`");
        let name = format!("caudal_test_{}_{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
        admin.execute(AssertSqlSafe(format!("CREATE DATABASE {name}"))).await.expect("can create databases");
        let options = PgConnectOptions::from_str(&url).expect("valid DATABASE_URL").database(&name);
        let owner = PgPool::connect_with(options.clone()).await.expect("connects");
        migrate(&owner).await.expect("migrations apply");
        owner.close().await;
        let runtime = PgPoolOptions::new()
            .max_connections(2)
            .after_connect(|conn, _| Box::pin(async move { conn.execute("SET ROLE ledger_runtime").await.map(drop) }))
            .connect_with(options)
            .await
            .expect("connects as ledger_runtime");
        Some(Self { admin, name, runtime })
    }

    async fn drop(self) {
        self.runtime.close().await;
        self.admin.execute(AssertSqlSafe(format!("DROP DATABASE {} WITH (FORCE)", self.name))).await.expect("drops");
    }
}

fn block_on<F: Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().expect("runtime").block_on(f)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn postgres_and_memory_agree(ops in prop::collection::vec(op(), 1..40)) {
        block_on(async {
            let Some(db) = TestDb::create().await else { return };
            let expected = run(&MemLedger::new(clock()), &ops).await;
            let actual = run(&PgLedger::new(db.runtime.clone(), clock()), &ops).await;
            db.drop().await;
            assert_eq!(actual, expected, "a divergence between backends fails the build");
        });
    }
}

#[test]
fn the_runtime_role_can_only_read_and_append() {
    block_on(async {
        let Some(db) = TestDb::create().await else { return };
        for table in ["ledger.entry", "ledger.leg"] {
            for (privilege, granted) in
                [("SELECT", true), ("INSERT", true), ("UPDATE", false), ("DELETE", false), ("TRUNCATE", false)]
            {
                let has: bool = sqlx::query_scalar("SELECT has_table_privilege('ledger_runtime', $1, $2)")
                    .bind(table)
                    .bind(privilege)
                    .fetch_one(&db.runtime)
                    .await
                    .expect("queryable");
                assert_eq!(has, granted, "{privilege} on {table}");
            }
        }
        let denied = db.runtime.execute("UPDATE ledger.entry SET kind = kind").await.expect_err("UPDATE is denied");
        assert_eq!(denied.as_database_error().and_then(|e| e.code()).as_deref(), Some("42501"));
        db.drop().await;
    });
}

#[test]
fn a_fresh_ledger_is_empty_and_readable() {
    block_on(async {
        let Some(db) = TestDb::create().await else { return };
        let ledger = PgLedger::new(db.runtime.clone(), clock());
        assert!(ledger.entries(EntryQuery::All).await.expect("readable").is_empty());
        db.drop().await;
    });
}

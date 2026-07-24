//! `MariaDB` container specification migrated from Quarkus test resources.

use cqrs_4_rust_test_support::TestHelper;

#[test]
fn preserves_mariadb_container_contract() {
    let resource = TestHelper::maria_db("11");
    assert_eq!(resource.database, "testdb");
    assert_eq!(resource.username, "mary");
}

use pgnation::{paginate, Pagination};

#[derive(sqlx::FromRow)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    age: i32,
    gender: String,
}

#[sqlx::test(migrations = "tests/migrations")]
async fn not_found(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users", 1, 10).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 0);
    assert_eq!(pagination.prev_page.is_none(), true);
    assert_eq!(pagination.next_page.is_none(), true);

    assert_eq!(users.len(), 0);
}

#[sqlx::test(migrations = "tests/migrations", fixtures("users"))]
async fn zero_size(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users", 1, 0).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 10);
    assert_eq!(pagination.prev_page.is_none(), true);
    assert_eq!(pagination.next_page.is_none(), true);

    assert_eq!(users.len(), 10);
}

#[sqlx::test(migrations = "tests/migrations", fixtures("users"))]
async fn prev_is_some_for_multiple_records(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users", 2, 10).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 10);
    assert_eq!(pagination.prev_page.is_some(), true);
    assert_eq!(pagination.next_page.is_none(), true);

    assert_eq!(users.len(), 0);
}

#[sqlx::test(migrations = "tests/migrations", fixtures("users"))]
async fn next_is_some_for_multiple_records(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users", 1, 2).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 10);
    assert_eq!(pagination.prev_page.is_none(), true);
    assert_eq!(pagination.next_page.is_some(), true);

    assert_eq!(users.len(), 2);
}

#[sqlx::test(migrations = "tests/migrations", fixtures("users"))]
async fn prev_is_some_for_single_record(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users WHERE id = 1", 2, 10).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 1);
    assert_eq!(pagination.prev_page.is_some(), true);
    assert_eq!(pagination.next_page.is_none(), true);

    assert_eq!(users.len(), 0);
}

#[sqlx::test(migrations = "tests/migrations", fixtures("users"))]
async fn next_is_none_for_single_record(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users WHERE id = 1", 1, 2).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 1);
    assert_eq!(pagination.prev_page.is_none(), true);
    assert_eq!(pagination.next_page.is_none(), true);

    assert_eq!(users.len(), 1);
}

#[sqlx::test(migrations = "tests/migrations", fixtures("users"))]
async fn prev_is_some_and_next_is_some(pool: sqlx::PgPool) {
    let result: Result<(Pagination, Vec<User>), sqlx::Error> =
        paginate(&pool, "SELECT * from users", 3, 2).await;

    assert_eq!(result.is_ok(), true);

    let (pagination, users) = result.unwrap();

    assert_eq!(pagination.total_count, 10);
    assert_eq!(pagination.prev_page.is_some(), true);
    assert_eq!(pagination.next_page.is_some(), true);

    assert_eq!(users.len(), 2);
}

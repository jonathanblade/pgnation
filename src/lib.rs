use sqlx::postgres::PgRow;
use sqlx::{Error, Executor, FromRow, Postgres, Row};

const DEFAULT_PAGE_SIZE: u32 = 100;

#[derive(Debug)]
pub struct Pagination {
    pub total_count: i64,
    pub prev_page: Option<u32>,
    pub next_page: Option<u32>,
}

pub async fn paginate<R>(
    executor: impl Executor<'_, Database = Postgres>,
    query: &str,
    page: u32,
    page_size: u32,
) -> Result<(Pagination, Vec<R>), Error>
where
    R: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let page = match page {
        0 => 1,
        _ => page,
    };
    let page_size = match page_size {
        0 => DEFAULT_PAGE_SIZE,
        _ => page_size,
    };
    let offset = match page {
        1 => 0,
        _ => page * page_size,
    };
    let records: Vec<PgRow> = sqlx::query(&format!(
        // https://stackoverflow.com/questions/28888375/run-a-query-with-a-limit-offset-and-also-get-the-total-number-of-rows
        "WITH cte AS ({query}) SELECT * FROM \
        (TABLE cte LIMIT {limit} OFFSET {offset}) sub \
        RIGHT JOIN (SELECT COUNT(*) FROM cte) c(total_count) ON true",
        query = query,
        limit = page_size,
        offset = offset
    ))
    .fetch_all(executor)
    .await?;
    let total_count = records
        .first()
        .ok_or(Error::RowNotFound)?
        .get::<i64, _>("total_count");
    let records = records
        .into_iter()
        .filter_map(|r| R::from_row(&r).ok())
        .collect();
    let prev_page = match page > 1 {
        true => Some(page - 1),
        false => None,
    };
    let next_page = match i64::from(offset + page_size) < total_count {
        true => Some(page + 1),
        false => None,
    };
    let pagination = Pagination {
        total_count,
        prev_page,
        next_page,
    };
    let result = (pagination, records);
    Ok(result)
}

#[macro_export]
macro_rules! fetch_public_items {
    ($db:expr, $table:expr, $struct:ty) => {{
        let items = sqlx::query(&format!("SELECT * FROM {}", $table))
            .fetch_all(&mut **$db)
            .await
            .map_err(|err| create_error!(DatabaseError {
                error: err.to_string()
            }))?;

        Ok(Json(
            items
                .iter()
                .filter_map(|row| {
                    let item: $struct = <$struct>::from_row(row).ok()?;
                    if let Visibility::Public = item.visibility {
                        Some(item)
                    } else {
                        None
                    }
                })
                .collect::<Vec<$struct>>(),
        ))
    }};
}
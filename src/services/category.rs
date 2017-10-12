use common::lazy_static::SQL_POOL;
use models::category::Category;

pub fn get_categories() -> Vec<Category> {

    let result = SQL_POOL.prep_exec("SELECT id, name FROM category", ()).unwrap();

    result.map(|row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {

            Category {
                id: row.get::<u8, _>(0).unwrap(),
                name: row.get::<String, _>(1).unwrap()
            }
        })
        .collect()
}


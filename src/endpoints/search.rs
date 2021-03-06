use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::{DbConn, models};
use crate::models::search::UserSearchResults;

#[get("/<searchname>")]
pub fn search_displayname(searchname: String, conn: DbConn) -> Json<models::search::UserSearchResults> {
    use crate::schema::users::dsl::*;

    let results = users.filter(displayname.eq(searchname))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error loading users");

    let mut search_data = UserSearchResults{
        results: vec![]
    };

    for result in results {
        search_data.results.push(result.id)
    }

    return Json(search_data);
}
use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::{DbConn, models};
use crate::models::timeline::UserTimeline;

#[get("/")]
pub fn get_timeline(conn: DbConn, key: crate::services::jwt::Claims) -> Json<models::timeline::UserTimeline>{
    use crate::schema::users::dsl::*;

    let mut user_timeline = UserTimeline{
        posts: vec![]
    };

    let mut user_following: Vec<String> = vec![];

    let following_array_results = users.filter(id.eq(&key.sub))
        .load::<crate::models::profile::Profile>(&*conn)
        .expect("Error loading user");


    for result in following_array_results {
        for following_result in result.following {
            user_following.push(following_result);
        }
    }

    user_following.iter()
        .for_each(|user_to_search| {
            // Can't find a pure SQL way to do this and there's no way to do in diesel.rs iirc
            // Not the best but whatever
            let user_following_result = users.filter(id.eq(user_to_search))
                .load::<crate::models::profile::Profile>(&*conn)
                .expect("Error loading user");

            for result in user_following_result {
                let mut i = 0;
                for followed_posts in result.posts {
                    if i == 4 {
                        break;
                    }
                    user_timeline.posts.push(followed_posts);
                    i += 1;
                }
            }
        });

    return Json(user_timeline);
}
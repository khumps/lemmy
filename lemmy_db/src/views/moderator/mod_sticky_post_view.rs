use crate::{
  limit_and_offset,
  schema::{community, mod_sticky_post, post, user_},
  source::{
    community::{Community, CommunitySafe},
    moderator::ModStickyPost,
    post::Post,
    user::{UserSafe, User_},
  },
  views::ViewToVec,
  ToSafe,
};
use diesel::{result::Error, *};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ModStickyPostView {
  pub mod_sticky_post: ModStickyPost,
  pub moderator: UserSafe,
  pub post: Post,
  pub community: CommunitySafe,
}

type ModStickyPostViewTuple = (ModStickyPost, UserSafe, Post, CommunitySafe);

impl ModStickyPostView {
  pub fn list(
    conn: &PgConnection,
    community_id: Option<i32>,
    mod_user_id: Option<i32>,
    page: Option<i64>,
    limit: Option<i64>,
  ) -> Result<Vec<Self>, Error> {
    let mut query = mod_sticky_post::table
      .inner_join(user_::table)
      .inner_join(post::table)
      .inner_join(community::table.on(post::community_id.eq(community::id)))
      .select((
        mod_sticky_post::all_columns,
        User_::safe_columns_tuple(),
        post::all_columns,
        Community::safe_columns_tuple(),
      ))
      .into_boxed();

    if let Some(community_id) = community_id {
      query = query.filter(post::community_id.eq(community_id));
    };

    if let Some(mod_user_id) = mod_user_id {
      query = query.filter(mod_sticky_post::mod_user_id.eq(mod_user_id));
    };

    let (limit, offset) = limit_and_offset(page, limit);

    let res = query
      .limit(limit)
      .offset(offset)
      .order_by(mod_sticky_post::when_.desc())
      .load::<ModStickyPostViewTuple>(conn)?;

    Ok(Self::to_vec(res))
  }
}

impl ViewToVec for ModStickyPostView {
  type DbTuple = ModStickyPostViewTuple;
  fn to_vec(mrp: Vec<Self::DbTuple>) -> Vec<Self> {
    mrp
      .iter()
      .map(|a| Self {
        mod_sticky_post: a.0.to_owned(),
        moderator: a.1.to_owned(),
        post: a.2.to_owned(),
        community: a.3.to_owned(),
      })
      .collect::<Vec<Self>>()
  }
}
use crate::domain::table::tables::User;
use crate::pojo::request::user_request::UserCreateRequest;
use actix_web::web::Data;
use rbatis::plugin::page::PageRequest;
use rbatis::{Page, RBatis};
use crate::error::Error;
use crate::pojo::dto::query::UserPageQuery;

pub async fn create(rb: &Data<RBatis>, data: &UserCreateRequest) {
    let user = User {
        common: Default::default(),
        username: data.username.clone(),
        password: data.password.clone(),
        nick_name: data.nick_name.clone(),
        phone: data.phone.clone(),
        email: data.email.clone(),
        sex: data.sex.clone(),
        enable: Some(true),
    };
    User::insert(&***rb, &user).await.unwrap();
}
//string_value.parse::<u32>().unwrap();
pub async fn pageList(rb: &Data<RBatis>, params: &UserPageQuery) -> Result<Page<User>, Error> {
    let page=  User::select_page_by_params(&***rb, &PageRequest::new(params.common.page_index.parse::<u64>().unwrap(), params.common.page_size.parse::<u64>().unwrap()),params).await?;
    Ok(page)
}
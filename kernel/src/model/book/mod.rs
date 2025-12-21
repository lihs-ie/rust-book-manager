use crate::model::{
    id::{BookId, UserId},
    user::BookOwner,
};

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
}

#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug)]
pub struct UpdateBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub requested_user: UserId,
}

#[derive(Debug)]
pub struct DeleteBook {
    pub book_id: BookId,
    pub requested_user: UserId,
}

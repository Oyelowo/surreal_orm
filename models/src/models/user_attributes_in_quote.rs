/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_orm::{
    Edge, LinkMany, LinkOne, Model, Node, Object, Relate, SurrealId, SurrealSimpleId,
};
use surrealdb::sql;

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[orm(table = user)]
pub struct User {
    pub id: SurrealId<Self, String>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub company: String,
    pub tags: Vec<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: User::create_id(sql::Id::rand().to_string()),
            name: Default::default(),
            created: Default::default(),
            company: Default::default(),
            tags: Default::default(),
        }
    }
}

#[derive(Edge, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = like)]
pub struct Like<In: Node, Out: Node> {
    pub id: SurrealSimpleId<Self>,
    #[serde(rename = "in", skip_serializing)]
    #[orm(link_many = "In")]
    pub in_: LinkOne<In>,
    #[serde(skip_serializing)]
    #[orm(link_one = "Out")]
    pub out: LinkOne<Out>,
    #[orm(nest_object = "Time")]
    pub time: Time,
}
pub type CompanyLikeUser = Like<Company, User>;

#[derive(surreal_orm::Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = company)]
pub struct Company {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[orm(link_many = "User")]
    pub users: LinkMany<User>,

    #[orm(relate(model = "CompanyLikeUser", connection = "->like->user"))]
    pub devs: Relate<User>,
}

#[derive(Object, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    // pub name: String,
    pub connected: DateTime<Utc>,
}

#[derive(Node, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[orm(table = organization)]
pub struct Organization {
    pub id: SurrealSimpleId<Self>,
    pub name: String,
    #[orm(link_many = "User")]
    pub users: LinkMany<User>,
    #[orm(nest_object = "Time")]
    pub time: Time,
    pub age: u8,
}

use mongodb::bson::Document;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;

use crate::db::DB;

#[derive(Clone)]
pub struct UserRepo {
    pub user_coll: Collection<User>,
    pub gen_coll: Collection<Document>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Pratham,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub gender: Gender,
    pub dob: String,
}

impl UserRepo {
    pub async fn init(db: DB) -> Result<Self, ()> {
        Ok(Self {
            user_coll: db.get_db().await.collection("users"),
            gen_coll: db.get_db().await.collection("users"),
        })
    }

    pub async fn add_user(&self, user: User) -> bool {
        let uname = user.username.clone();

        if self.user_exists(uname).await {
            println!("username taken");
            false
        } else {
            self.user_coll
                .insert_one(user, None)
                .await
                .expect("error adding user");
            println!("User added successfully");
            true
        }
    }

    pub async fn user_exists(&self, username: String) -> bool {
        let u = self
            .user_coll
            .find_one(doc! {"username": username}, None)
            .await
            .expect("error");

        match u {
            Some(_u) => true,
            None => false,
        }
    }

    pub async fn get_user_oid(&self, username: String) -> ObjectId {
        self.gen_coll
            .find_one(doc! {"username": username}, None)
            .await
            .unwrap()
            .expect("user not found")
            .get_object_id("_id")
            .unwrap()
    }

    pub async fn uid_exists(&self, oid: ObjectId) -> bool {
        let u = self
            .user_coll
            .find_one(doc! {"_id": oid}, None)
            .await
            .expect("error");

        match u {
            Some(_u) => true,
            None => false,
        }
    }

    pub async fn find_user_by_id(&self, oid: ObjectId) -> User {
        self.user_coll
            .find_one(doc! {"_id": oid}, None)
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        let mut users: Vec<User> = Vec::new();
        let mut cursor = self.user_coll.find(doc! {}, None).await.unwrap();

        while let Some(u) = cursor.try_next().await.unwrap() {
            users.push(u);
        }
        users
    }

}

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::prelude::{Identifiable, Queryable};

#[derive(Queryable, Clone, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub unique_id: String,
}

impl User {
    pub fn verify(&self, password: String) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.password).unwrap();
        let result = argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        return result;
    }
}

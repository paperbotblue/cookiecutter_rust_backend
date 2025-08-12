use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use serde::Serialize;
use std::fmt;
use std::io::Write;
use std::str::FromStr;
use uuid::Uuid;

pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub struct CreateRole {
    pub name: String,
    pub description: String,
}

pub struct UpdateRole {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum ClientType {
    Admin,
    User,
    Invalid,
}

impl FromStr for ClientType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(ClientType::Admin),
            "user" => Ok(ClientType::User),
            _ => Err(format!("Invalid client type: {}", s)),
        }
    }
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ClientType::Admin => "admin",
            ClientType::User => "user",
            ClientType::Invalid => "invalid",
        };
        write!(f, "{}", s)
    }
}

impl ToSql<Text, Pg> for ClientType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let s = match self {
            ClientType::Admin => "admin",
            ClientType::User => "user",
            ClientType::Invalid => "invalid",
        };
        out.write_all(s.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for ClientType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        match s {
            "admin" => Ok(ClientType::Admin),
            "user" => Ok(ClientType::User),
            _ => Ok(ClientType::Invalid),
        }
    }
}

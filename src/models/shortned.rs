use libsql::{de, params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Shortned {
    pub id: u32,
    pub name: String,
    pub link: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Shortned {
    pub fn new(
        id: u32,
        name: String,
        link: String,
        created_at: String,
        updated_at: String,
    ) -> Shortned {
        Shortned {
            id,
            name,
            link,
            created_at,
            updated_at,
        }
    }

    pub async fn get_by_name(
        db: &Connection,
        name: String,
    ) -> Result<Option<Shortned>, Box<dyn std::error::Error>> {
        let mut rows = db
            .query("SELECT * FROM links WHERE name = ?1", params![name])
            .await?;

        if let Some(row) = rows.next().await? {
            let res = de::from_row::<Shortned>(&row)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_links(
        db: &Connection,
    ) -> Result<Vec<Shortned>, Box<dyn std::error::Error>> {
        let mut rows = db.query("SELECT * FROM links", ()).await?;

        let mut links: Vec<Shortned> = Vec::new();

        while let Some(row) = rows.next().await? {
            links.push(de::from_row(&row)?)
        }

        Ok(links)
    }
}

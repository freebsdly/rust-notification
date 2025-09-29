mod entity;
mod sqlite;

/**
 * Database Repository
 */
pub trait DatabaseRepository<T, ID> {
    /**
     * Find all entities
     */
    async fn find_all(&self) -> Result<Vec<T>, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    /**
     * Find an entity by id
     */
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    /**
     * Save an entity
     */
    async fn save(&self, entity: T) -> Result<T, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    /**
     * Delete an entity
     */
    async fn delete(&self, entity: T) -> Result<T, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    /**
     * Delete an entity by id
     */
    async fn delete_by_id(&self, id: ID) -> Result<T, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    /**
     * Update an entity
     */
    async fn update(&self, entity: T) -> Result<T, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    /**
     * Save or update an entity
     */
    async fn save_or_update(&self, entity: T) -> Result<T, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

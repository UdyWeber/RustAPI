use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Return `true` to signify that Graphql server is reachable
    async fn healt(&self) -> bool {
        true
    }
}

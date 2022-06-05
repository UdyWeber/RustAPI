// I'm bringing in Empty Mutation and Empty Subscriptio cause to build our schema we need 3 parameters
// Query, Mutation and Subscription so async-graphql has Empty structs to help us build our Schema when we dont have proper Structs yet

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

mod health;

#[derive(MergedObject, Default)]
pub struct Query(health::HealthQuery);

///Builds the Grapqhl schema
pub fn build_graphql_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}

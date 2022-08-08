use async_graphql :: { http :: { playground_source , GraphQLPlaygroundConfig } , EmptyMutation , EmptySubscription , Schema , dataloader :: DataLoader } ; use async_graphql_poem :: GraphQL ; use poem :: { get , handler , listener :: TcpListener , web :: Html , IntoResponse , Route , Server } ; use sea_orm :: Database ; use generated :: * ; # [handler] async fn graphql_playground () -> impl IntoResponse { Html (playground_source (GraphQLPlaygroundConfig :: new ("/"))) } # [tokio :: main] async fn main () { tracing_subscriber :: fmt () . with_max_level (tracing :: Level :: DEBUG) . with_test_writer () . init () ; let database = Database :: connect ("sqlite://chinook.db") . await . unwrap () ; let orm_dataloader : DataLoader < OrmDataloader > = DataLoader :: new (OrmDataloader { db : database . clone () } , tokio :: spawn) ; let schema = Schema :: build (QueryRoot , EmptyMutation , EmptySubscription) . data (database) . data (orm_dataloader) . finish () ; let app = Route :: new () . at ("/" , get (graphql_playground) . post (GraphQL :: new (schema))) ; println ! ("Playground: http://localhost:8000") ; Server :: new (TcpListener :: bind ("0.0.0.0:8000")) . run (app) . await . unwrap () ; }
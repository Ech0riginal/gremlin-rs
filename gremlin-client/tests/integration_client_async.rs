mod common;

use common::io::*;
use common::REMOTE_HOST;
use gremlin_client::prelude::*;

#[tokio::test]
async fn test_client_connection_ok() {
    connect().await.expect("Cannot connect");
}

#[tokio::test]
async fn test_ok_credentials() {
    let client = GremlinClient::connect(
        ConnectionOptions::builder()
            .host(REMOTE_HOST)
            .port(8183)
            .credentials("stephen", "password")
            .ssl(true)
            .tls_options(TlsOptions::default())
            .build(),
    )
    .await
    .expect("Cannot connect");

    let result = client.execute("g.V().limit(1)", &[]).await;
    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_wrong_query() {
    let error = connect()
        .await
        .expect("it should connect")
        .execute("g.V", &[])
        .await
        .expect_err("it should return an error");

    match error {
        GremlinError::Request((code, message)) => {
            assert_eq!(597, code);
            assert_eq!("No such property: V for class: org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource",message)
        }
        _ => panic!("wrong error type"),
    }
}

#[tokio::test]
async fn test_wrong_alias() {
    let error = connect()
        .await
        .expect("it should connect")
        .alias("foo")
        .execute("g.V()", &[])
        .await
        .expect_err("it should return an error");

    match error {
        GremlinError::Request((code, message)) => {
            assert_eq!(499, code);
            assert_eq!("Could not alias [g] to [foo] as [foo] not in the Graph or TraversalSource global bindings",message)
        }
        _ => panic!("wrong error type"),
    }
}

#[tokio::test]
async fn test_vertex_query() {
    let graph = connect().await.expect("it should connect");
    let vertices = graph
        .execute(
            "g.V().hasLabel('person').has('name',name)",
            &[("name", &"marko")],
        )
        .await
        .expect("it should execute a query")
        .filter_map(Result::ok)
        .map(|f| f.take::<Vertex>())
        .collect::<Result<Vec<Vertex>, _>>()
        .await
        .expect("It should be ok");

    assert_eq!("person", vertices[0].label());
}
#[tokio::test]
async fn test_edge_query() {
    let graph = connect().await.expect("it should connect");
    let edges = graph
        .execute("g.E().hasLabel('knows').limit(1)", &[])
        .await
        .expect("it should execute a query")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .await
        .expect("It should be ok");

    assert_eq!("knows", edges[0].label());
}

#[tokio::test]
async fn test_vertex_creation() {
    let graph = connect().await.expect("it should connect");
    let mark = create_vertex(&graph, "mark").await;

    assert_eq!("person", mark.label());

    let value_map = graph
        .execute("g.V(identity).valueMap()", &[("identity", mark.id())])
        .await
        .expect("should fetch valueMap with properties")
        .filter_map(Result::ok)
        .map(|f| f.take::<Map>())
        .collect::<Result<Vec<Map>, _>>()
        .await
        .expect("It should be ok");

    assert_eq!(1, value_map.len());

    assert_eq!(
        Some(&GValue::List(vec![String::from("mark").into()].into())),
        value_map[0].get("name")
    );
}

#[tokio::test]
async fn test_edge_creation() {
    let graph = connect().await.expect("it should connect");
    let mark = create_vertex(&graph, "mark").await;
    let frank = create_vertex(&graph, "frank").await;

    let edge = create_edge(&graph, &mark, &frank, "knows").await;

    assert_eq!("knows", edge.label());

    assert_eq!(&mark, edge.out_v());
    assert_eq!(&frank, edge.in_v());

    let edges = graph
        .execute("g.V(identity).outE()", &[("identity", mark.id())])
        .await
        .expect("should fetch edge")
        .filter_map(Result::ok)
        .map(|f| f.take::<Edge>())
        .collect::<Result<Vec<Edge>, _>>()
        .await
        .expect("It should be ok");

    assert_eq!(1, edges.len());

    let edge = &edges[0];

    assert_eq!("knows", edge.label());

    assert_eq!(&mark, edge.out_v());
    assert_eq!(&frank, edge.in_v());
}

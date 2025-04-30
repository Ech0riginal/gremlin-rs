use gremlin_client::prelude::*;

pub const REMOTE_HOST: &'static str = "localhost";
pub const REMOTE_PORT: u16 = 8182;

pub fn assert_map_property(element_map: &Map, expected_key: &str, expected_value: &str) {
    let actual_prop_value: &String = element_map
        .get(expected_key)
        .unwrap_or_else(|| panic!("Didn't have expected key {}", expected_key))
        .get()
        .expect("Should be String");
    assert_eq!(expected_value, actual_prop_value);
}

#[allow(dead_code)]
pub mod io {
    use super::{REMOTE_HOST, REMOTE_PORT};
    use futures::stream::StreamExt;
    use gremlin_client::prelude::*;

    pub async fn graph() -> GremlinClient<V3> {
        let client = connect().await.expect("It should connect");

        client
    }

    pub fn connect() -> impl std::future::Future<Output = GremlinResult<GremlinClient<V3>>> {
        GremlinClient::connect((REMOTE_HOST, REMOTE_PORT))
    }

    pub fn connect_serializer<SD: GraphSON>(
        serializer: SD,
    ) -> impl std::future::Future<Output = GremlinResult<GremlinClient<SD>>> {
        GremlinClient::connect(
            ConnectionOptions::builder()
                .host(REMOTE_HOST)
                .port(REMOTE_PORT)
                .serde(serializer)
                .build(),
        )
    }



    pub async fn create_vertex<G: GraphSON>(graph: &GremlinClient<G>, name: &str) -> Vertex {
        create_vertex_with_label(graph, "person", name).await
    }

    pub async fn create_vertex_with_label<G: GraphSON>(
        graph: &GremlinClient<G>,
        label: &str,
        name: &str,
    ) -> Vertex {
        graph
            .execute(
                "g.addV(_label).property('name',name)",
                &[("_label", &label), ("name", &name)],
            )
            .await
            .expect("it should execute addV")
            .filter_map(|result| async { Result::ok(result) })
            .map(|f| f.take::<Vertex>())
            .collect::<Vec<GremlinResult<Vertex>>>()
            .await
            .pop()
            .expect("It should contain 1 element")
            .expect("It should be okay")
    }

    pub async fn drop_vertices<G: GraphSON>(
        graph: &GremlinClient<G>,
        label: &str,
    ) -> GremlinResult<()> {
        graph
            .execute("g.V().hasLabel(_label).drop()", &[("_label", &label)])
            .await
            .map(|_| ())
    }

    pub async fn create_edge<G: GraphSON>(
        graph: &GremlinClient<G>,
        v: &Vertex,
        v1: &Vertex,
        name: &str,
    ) -> Edge {
        graph
            .execute(
                "g.V(v1).as('a').V(v2).as('b').addE(rel).from('a').to('b')",
                &[("rel", &name), ("v1", v.id()), ("v2", v1.id())],
            )
            .await
            .expect("it should execute addE")
            .filter_map(|result| async { Result::ok(result) })
            .map(|f| f.take::<Edge>())
            .collect::<Vec<GremlinResult<Edge>>>()
            .await
            .pop()
            .expect("It should contain 1 element")
            .expect("It should be okay")
    }

    pub async fn drop_edges<G: GraphSON>(
        graph: &GremlinClient<G>,
        label: &str,
    ) -> GremlinResult<()> {
        graph
            .execute("g.E().hasLabel(_label).drop()", &[("_label", &label)])
            .await
            .map(|_| ())
    }
}

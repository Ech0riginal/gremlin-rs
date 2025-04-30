mod common;

use gremlin_client::prelude::ToGValue;
use common::assert_map_property;
use common::io::*;
use gremlin_client::prelude::*;

#[tokio::test]
async fn test_simple_vertex_traversal_with_multiple_id() {
    let client = connect().await.expect("a gremlin client");
    drop_vertices(&client, "test_simple_vertex_traversal_async")
        .await
        .unwrap();

    let vertex =
        create_vertex_with_label(&client, "test_simple_vertex_traversal_async", "Traversal").await;
    let vertex2 =
        create_vertex_with_label(&client, "test_simple_vertex_traversal_async", "Traversal").await;

    let g = traversal().with_remote(client);

    let results = g
        .v(vec![vertex.id(), vertex2.id()])
        .to_list()
        .await
        .unwrap();

    assert_eq!(2, results.len());

    assert_eq!(vertex.id(), results[0].id());
    assert_eq!(vertex2.id(), results[1].id());

    let has_next = g
        .v(())
        .has_label("test_simple_vertex_traversal_async")
        .has_next()
        .await
        .expect("It should return");

    assert_eq!(true, has_next);

    let next = g
        .v(())
        .has_label("test_simple_vertex_traversal_async")
        .next()
        .await
        .expect("It should execute one traversal")
        .expect("It should return one element");

    assert_eq!("test_simple_vertex_traversal_async", next.label());

    let vertices = g
        .v(())
        .has_label("test_simple_vertex_traversal_async")
        .iter()
        .await
        .expect("It should get the iterator")
        .collect::<Result<Vec<Vertex>, _>>()
        .await
        .expect("It should collect elements");

    assert_eq!(2, vertices.len());
}

use gremlin_client::prelude::{
    traversal::{GraphTraversalSource},
    Direction, Merge,
    Edge, GValue,
};
use std::collections::HashMap;
use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::test]
async fn test_merge_v_no_options() {
    let client = graph().await;
    let test_vertex_label = "test_merge_v_no_options";
    drop_vertices(&client, test_vertex_label)
        .await
        .expect("Failed to drop vertices in case of rerun");
    let g = traversal().with_remote(client);

    let mut injection_map: HashMap<GKey, GValue> = HashMap::new();
    let mut lookup_map: HashMap<GKey, GValue> = HashMap::new();
    lookup_map.insert(T::Label.into(), test_vertex_label.into());
    let mut property_map: HashMap<GKey, GValue> = HashMap::new();
    property_map.insert("propertyKey".into(), "propertyValue".into());
    injection_map.insert("lookup".into(), lookup_map.into());
    injection_map.insert("properties".into(), property_map.into());

    let vertex_properties = g
        .inject(vec![injection_map.into()])
        .unfold()
        .as_("payload")
        .merge_v(__.select("lookup"))
        .property(
            "propertyKey",
            __.select("payload")
                .select("properties")
                .select("propertyKey"),
        )
        .element_map(())
        .next()
        .await
        .expect("Should get response")
        .expect("Should have returned a vertex");

    assert_map_property(&vertex_properties, "propertyKey", "propertyValue");
}

#[tokio::test]
async fn test_merge_v_options() {
    let std_out_filter = Targets::new()
        .with_target("hedwig", Level::TRACE)
        .with_target("gremlin", Level::TRACE)
        .with_target("hyper_util::client::legacy::pool", Level::INFO);
    let stdout_layer = tracing_subscriber::fmt::layer()
        // .json()
        .with_target(false)
        .with_filter(std_out_filter);
    tracing_subscriber::registry().with(stdout_layer).init();


    let client = graph().await;
    let expected_label = "test_merge_v_options";

    drop_vertices(&client, expected_label).await.expect("Failed to drop vertices");

    let g = traversal().with_remote(client);
    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(T::Label.into(), expected_label.into());
    start_step_map.insert("identifing_prop".into(), "some_Value".into());

    let prop_key = "some_prop";
    let mut on_create_map: HashMap<GKey, GValue> = HashMap::new();
    let expected_on_create_prop_value = "on_create_value";
    on_create_map.insert(prop_key.into(), expected_on_create_prop_value.into());

    let mut on_match_map: HashMap<GKey, GValue> = HashMap::new();
    let expected_on_match_prop_value = "on_match_value";
    on_match_map.insert(prop_key.into(), expected_on_match_prop_value.into());

    let result = g
        .merge_v(start_step_map.clone())
        .option((Merge::OnCreate, on_create_map.clone()))
        .option((Merge::OnMatch, on_match_map.clone()))
        .element_map(())
        .next()
        .await;
    let on_create_vertex_map = result
        .expect("Should get a response")
        .expect("Should return a vertex");

    assert_map_property(&on_create_vertex_map, "label", expected_label);

    assert_map_property(
        &on_create_vertex_map,
        prop_key,
        expected_on_create_prop_value,
    );

    //Now run the traversal again, and confirm the OnMatch applied this time
    let on_match_vertex_map = g
        .merge_v(start_step_map)
        .option((Merge::OnCreate, on_create_map.clone()))
        .option((Merge::OnMatch, on_match_map.clone()))
        .element_map(())
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    assert_map_property(&on_match_vertex_map, "label", expected_label);

    assert_map_property(&on_match_vertex_map, prop_key, expected_on_match_prop_value);
}

#[tokio::test]
async fn test_merge_v_start_step() {
    tracing_subscriber::fmt().init();

    let client = graph().await;
    let expected_label = "test_merge_v_start_step";
    drop_vertices(&client, &expected_label).await.expect("Failed to drop vertices");
    let g = traversal().with_remote(client);
    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(T::Label.into(), expected_label.into());
    let actual_vertex = g
        .merge_v(start_step_map)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    assert_eq!(expected_label, actual_vertex.label())
}

#[tokio::test]
async fn test_merge_v_anonymous_traversal() {
    let client = graph().await;
    let expected_label = "test_merge_v_anonymous_traversal";
    drop_vertices(&client, &expected_label).await.expect("Failed to drop vertiecs");
    let g = traversal().with_remote(client);
    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(T::Label.into(), expected_label.into());
    let actual_vertex = g
        .inject(1)
        .unfold()
        .coalesce::<Vertex, _>([__.merge_v(start_step_map)])
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");
    assert_eq!(expected_label, actual_vertex.label())
}

#[tokio::test]
async fn test_merge_e_start_step() {
    let std_out_filter = Targets::new()
        .with_target("hedwig", Level::TRACE)
        .with_target("gremlin", Level::TRACE)
        .with_target("hyper_util::client::legacy::pool", Level::INFO);
    let stdout_layer = tracing_subscriber::fmt::layer()
        // .json()
        .with_target(false)
        .with_filter(std_out_filter);
    tracing_subscriber::registry().with(stdout_layer).init();


    let client = graph().await;
    let expected_vertex_label = "test_merge_e_start_step_vertex";
    let expected_edge_label = "test_merge_e_start_step_edge";
    let expected_edge_property_key = "test_merge_e_start_step_edge_prop";
    let expected_edge_property_value = "test_merge_e_start_step_edge_value";
    drop_vertices(&client, &expected_vertex_label).await.expect("Failed to drop vertices");
    let g = traversal().with_remote(client);

    let vertex_a = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let vertex_b = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let mut start_step_map: HashMap<GKey, GValue> = HashMap::new();
    start_step_map.insert(Direction::In.into(), vertex_a.id().into());
    start_step_map.insert(Direction::Out.into(), vertex_b.id().into());
    start_step_map.insert(T::Label.into(), expected_edge_label.into());
    start_step_map.insert(
        expected_edge_property_key.into(),
        expected_edge_property_value.into(),
    );
    let merged_edge_properties = g
        .merge_e(start_step_map)
        // .element_map(())
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a edge properties");

    println!("{:?}", merged_edge_properties);

    // assert_map_property(&merged_edge_properties, "label", expected_edge_label);

    // assert_map_property(
    //     &merged_edge_properties,
    //     expected_edge_property_key,
    //     expected_edge_property_value,
    // );

    // let incoming_vertex: &Map = merged_edge_properties
    //     .get(Direction::In)
    //     .expect("Should have returned incoming vertex info")
    //     .get()
    //     .unwrap();
    //
    // let incoming_vertex_id = incoming_vertex
    //     .get("id")
    //     .expect("Should have returned vertex id");
    // assert_eq!(incoming_vertex_id, &vertex_a.id().to_gvalue());
    //
    // let outgoing_vertex: &Map = merged_edge_properties
    //     .get(Direction::Out)
    //     .expect("Should have returned outgoing vertex info")
    //     .get()
    //     .unwrap();
    // let outgoing_vertex_id = outgoing_vertex
    //     .get("id")
    //     .expect("Should have returned vertex id");
    // assert_eq!(outgoing_vertex_id, &vertex_b.id().to_gvalue());
}

#[tokio::test]
async fn test_merge_e_no_options() {
    let client = graph().await;
    let expected_vertex_label = "test_merge_e_no_options_vertex";
    let expected_edge_label = "test_merge_e_no_options_edge";
    let expected_edge_property_key = "test_merge_e_no_options_edge_prop";
    let expected_edge_property_value = "test_merge_e_no_options_edge_value";
    drop_vertices(&client, &expected_vertex_label).await.expect("Failed to drop vertiecs");
    let g = traversal().with_remote(client);

    let vertex_a = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let vertex_b = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let mut assignment_map: HashMap<GKey, GValue> = HashMap::new();
    assignment_map.insert(Direction::In.into(), vertex_a.id().into());
    assignment_map.insert(Direction::Out.into(), vertex_b.id().into());
    assignment_map.insert(T::Label.into(), expected_edge_label.into());
    assignment_map.insert(
        expected_edge_property_key.into(),
        expected_edge_property_value.into(),
    );

    let merged_edge_properties = g
        .inject(vec![assignment_map.into()])
        .unfold()
        .as_("payload")
        .merge_e(__.select("payload"))
        .element_map(())
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return edge properties");

    assert_map_property(&merged_edge_properties, "label", expected_edge_label);
    assert_map_property(
        &merged_edge_properties,
        expected_edge_property_key,
        expected_edge_property_value,
    );

    let incoming_vertex: &Map = merged_edge_properties
        .get(Direction::In)
        .expect("Should have returned incoming vertex info")
        .get()
        .unwrap();
    let incoming_vertex_id = incoming_vertex
        .get("id")
        .expect("Should have returned vertex id");
    assert_eq!(incoming_vertex_id, &vertex_a.id().to_gvalue());

    let outgoing_vertex: &Map = merged_edge_properties
        .get(Direction::Out)
        .expect("Should have returned outgoing vertex info")
        .get()
        .unwrap();
    let outgoing_vertex_id = outgoing_vertex
        .get("id")
        .expect("Should have returned vertex id");
    assert_eq!(outgoing_vertex_id, &vertex_b.id().to_gvalue());
}

#[tokio::test]
async fn test_merge_e_options() {
    let client = graph().await;
    let expected_vertex_label = "test_merge_e_options_vertex";
    let expected_edge_label = "test_merge_e_options_edge";
    let expected_edge_property_key = "test_merge_e_options_edge_prop";

    drop_vertices(&client, &expected_vertex_label).await.expect("Failed to drop vertiecs");

    let g = traversal().with_remote(client);

    let vertex_a = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let vertex_b = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let mut assignment_map: HashMap<GKey, GValue> = HashMap::new();
    assignment_map.insert(Direction::In.into(), vertex_a.id().into());
    assignment_map.insert(Direction::Out.into(), vertex_b.id().into());
    assignment_map.insert(T::Label.into(), expected_edge_label.into());

    let mut on_create_map: HashMap<GKey, GValue> = HashMap::new();
    on_create_map.insert(expected_edge_property_key.into(), "on_create".into());

    let mut on_match_map: HashMap<GKey, GValue> = HashMap::new();
    on_match_map.insert(expected_edge_property_key.into(), "on_match".into());

    let mut injection_map: HashMap<GKey, GValue> = HashMap::new();
    injection_map.insert("merge_params".into(), assignment_map.into());
    injection_map.insert("create_params".into(), on_create_map.into());
    injection_map.insert("match_params".into(), on_match_map.into());

    let do_merge_edge = async |g: GraphTraversalSource<AsyncTerminator<V3>>| -> Map {
        g.inject(vec![injection_map.clone().into()])
            .unfold()
            .as_("payload")
            .merge_e(__.select("payload").select("merge_params"))
            .option((
                Merge::OnCreate,
                __.select("payload").select("create_params"),
            ))
            .option((Merge::OnMatch, __.select("payload").select("match_params")))
            .element_map(())
            .next()
            .await
            .expect("Should get a response")
            .expect("Should return a edge properties")
    };

    let on_create_edge_properties = do_merge_edge(g.clone()).await;

    //Initially the edge should be the on create value
    assert_map_property(
        &on_create_edge_properties,
        expected_edge_property_key,
        "on_create",
    );

    let on_match_edge_properties = do_merge_edge(g).await;
    assert_map_property(
        &on_match_edge_properties,
        expected_edge_property_key,
        "on_match",
    );
}

#[tokio::test]
async fn test_merge_e_anonymous_traversal() {
    let client = graph().await;
    let expected_vertex_label = "test_merge_e_options_vertex";
    let expected_edge_label = "test_merge_e_options_edge";

    drop_vertices(&client, &expected_vertex_label).await.expect("Failed to drop vertiecs");

    let g = traversal().with_remote(client);

    let vertex_a = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let vertex_b = g
        .add_v(expected_vertex_label)
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a vertex");

    let mut assignment_map: HashMap<GKey, GValue> = HashMap::new();
    assignment_map.insert(Direction::In.into(), vertex_a.id().into());
    assignment_map.insert(Direction::Out.into(), vertex_b.id().into());
    assignment_map.insert(T::Label.into(), expected_edge_label.into());

    let anonymous_merge_e_properties = g
        .inject(1)
        .unfold()
        .coalesce::<Edge, _>([__.merge_e(assignment_map)])
        .element_map(())
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a edge properties");

    let incoming_vertex: &Map = anonymous_merge_e_properties
        .get(Direction::In)
        .expect("Should have returned incoming vertex info")
        .get()
        .unwrap();
    let incoming_vertex_id = incoming_vertex
        .get("id")
        .expect("Should have returned vertex id");
    assert_eq!(incoming_vertex_id, &vertex_a.id().to_gvalue());

    let outgoing_vertex: &Map = anonymous_merge_e_properties
        .get(Direction::Out)
        .expect("Should have returned outgoing vertex info")
        .get()
        .unwrap();
    let outgoing_vertex_id = outgoing_vertex
        .get("id")
        .expect("Should have returned vertex id");
    assert_eq!(outgoing_vertex_id, &vertex_b.id().to_gvalue());
}

#[tokio::test]
async fn test_merge_v_into_merge_e() {
    //Based on the reference doc's combo example
    let client = graph().await;
    let expected_vertex_label = "test_merge_v_into_merge_e_vertex";
    let expected_edge_label = "test_merge_v_into_merge_e_edge";

    drop_vertices(&client, &expected_vertex_label).await.expect("Failed to drop vertiecs");

    let g = traversal().with_remote(client);

    let expected_toby_id = 100_001i64;
    let expected_brandy_id = 200_001i64;

    let mut vertex_a_map: HashMap<GKey, GValue> = HashMap::new();
    vertex_a_map.insert(T::Label.into(), expected_vertex_label.into());
    vertex_a_map.insert(T::Id.into(), expected_toby_id.into());
    vertex_a_map.insert("name".into(), "Toby".into());

    let mut vertex_b_map: HashMap<GKey, GValue> = HashMap::new();
    vertex_b_map.insert(T::Label.into(), expected_vertex_label.into());
    vertex_b_map.insert(T::Id.into(), expected_brandy_id.into());
    vertex_b_map.insert("name".into(), "Brandy".into());

    let mut edge_map: HashMap<GKey, GValue> = HashMap::new();
    edge_map.insert(T::Label.into(), expected_edge_label.into());
    edge_map.insert("some_key".into(), "some_value".into());
    edge_map.insert(Direction::From.into(), Merge::OutV.into());
    edge_map.insert(Direction::To.into(), Merge::InV.into());

    let combo_merge_edge_properties = g
        .merge_v(vertex_a_map)
        .as_("Toby")
        .merge_v(vertex_b_map)
        .as_("Brandy")
        .merge_e(edge_map)
        .option((Merge::OutV, __.select("Toby")))
        .option((Merge::InV, __.select("Brandy")))
        .element_map(())
        .next()
        .await
        .expect("Should get a response")
        .expect("Should return a edge properties");

    let brandy_vertex: &Map = combo_merge_edge_properties
        .get(Direction::In)
        .expect("Should have returned incoming vertex info")
        .get()
        .unwrap();
    let brandy_vertex_id = brandy_vertex
        .get("id")
        .expect("Should have returned vertex id");
    assert_eq!(*brandy_vertex_id, GValue::Int64(expected_brandy_id));

    let toby_vertex: &Map = combo_merge_edge_properties
        .get(Direction::Out)
        .expect("Should have returned outgoing vertex info")
        .get()
        .unwrap();
    let toby_vertex_id = toby_vertex
        .get("id")
        .expect("Should have returned vertex id");
    assert_eq!(*toby_vertex_id, GValue::Int64(expected_toby_id));

    assert_map_property(&combo_merge_edge_properties, "label", expected_edge_label);
}
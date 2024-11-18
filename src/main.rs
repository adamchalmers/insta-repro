#[derive(serde::Serialize, serde::Deserialize)]
struct Node {
    nodes: Vec<Node>,
    data: f64,
}

fn main() {}

#[test]
fn test_recurse() {
    let data = Node {
        nodes: vec![
            Node {
                nodes: vec![Node {
                    nodes: vec![],
                    data: f64::tan(4.0),
                }],
                data: f64::tan(2.0),
            },
            Node {
                nodes: vec![],
                data: f64::tan(3.0),
            },
        ],
        data: f64::tan(1.0),
    };
    insta::assert_yaml_snapshot!("data", data, {
        ".**[]" => insta::rounded_redaction(3),
        ".**" => insta::rounded_redaction(3),
    });
}

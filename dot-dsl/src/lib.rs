pub mod graph {
    use std::collections::HashMap;

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use maplit::hashmap;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            use maplit::hashmap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub node_id: String,
                pub other_node_id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node_id: &str, other_node_id: &str) -> Self {
                    Self {
                        node_id: node_id.to_string(),
                        other_node_id: other_node_id.to_string(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            use maplit::hashmap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_string(),
                        attrs: hashmap! {},
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    self
                }

                pub fn get_attr<'a>(&'a self, attr: &str) -> Option<&'a str> {
                    let value = self.attrs.get(attr)?;
                    Some(value)
                }
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            self
        }

        pub fn get_node(&self, node_id: &str) -> Option<Node> {
            self.nodes.iter().find(|&x| x.id == node_id).cloned()
        }
    }

    impl Default for Graph {
        fn default() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap! {},
            }
        }
    }
}

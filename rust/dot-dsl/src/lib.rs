pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub start: String,
                pub end: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Self {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .iter()
                            .map(|(attr1, attr2)| ((*attr1).to_string(), (*attr2).to_string())),
                    );
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub id: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    Self {
                        id: id.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .iter()
                            .map(|(attr1, attr2)| ((*attr1).to_string(), (*attr2).to_string())),
                    );
                    self
                }

                pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
                    self.attrs.get(attr_key).map(String::as_str)
                }
            }
        }
    }
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs.extend(
                attrs
                    .iter()
                    .map(|(attr1, attr2)| ((*attr1).to_string(), (*attr2).to_string())),
            );
            self
        }

        pub fn get_node(mut self, id: &str) -> Option<Node> {
            self.nodes.dedup();
            self.nodes.iter().find(|n| n.id == id).cloned()
        }
    }
}

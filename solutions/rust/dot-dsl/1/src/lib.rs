pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<&'a str, &'a str>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node<'a>>) -> Self {
            nodes.iter().for_each(|n| self.nodes.push(n.clone()));

            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge<'a>>) -> Self {
            edges.iter().for_each(|e| self.edges.push(e.clone()));

            self
        }

        pub fn with_attrs(mut self, list: &[(&'a str, &'a str)]) -> Self {
            list.iter().for_each(|&(key, val)| {
                self.attrs.insert(key, val);
            });

            self
        }

        pub fn node(&self, label: &str) -> Option<Node> {
            self.nodes.iter().find(|&n| n.label() == label).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Edge {
                        from,
                        to,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, list: &[(&'a str, &'a str)]) -> Self {
                    list.iter().for_each(|&(key, val)| {
                        self.attrs.insert(key, val);
                    });

                    self
                }

                pub fn attr(&self, key: &'a str) -> Option<&str> {
                    self.attrs.get(key).cloned()
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node<'a> {
                label: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(label: &'a str) -> Self {
                    Node {
                        label,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, list: &[(&'a str, &'a str)]) -> Self {
                    list.iter().for_each(|&(key, val)| {
                        self.attrs.insert(key, val);
                    });

                    self
                }

                pub fn label(&self) -> &str {
                    self.label
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).copied()
                }
            }
        }
    }
}

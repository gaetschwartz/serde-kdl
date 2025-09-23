use serde_kdl_macro::kdl;


#[cfg(test)]
mod basic_usage_tests {
    use super::*;

    #[test]
    fn test_simple_node() {
        let doc = kdl! {
            simple_node 42
        };

        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "simple_node");
        assert_eq!(node.entries().len(), 1);
        assert_eq!(node.entries()[0].value().as_i64().unwrap(), 42);
    }

    #[test]
    fn test_node_with_string() {
        let doc = kdl! {
            config "my-app"
        };

        assert_eq!(doc.nodes().len(), 1);
        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "config");
        assert_eq!(node.entries().len(), 1);
        assert_eq!(node.entries()[0].value().as_string().unwrap(), "my-app");
    }

    #[test]
    fn test_node_with_boolean() {
        let doc = kdl! {
            debug true
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "debug");
        assert_eq!(node.entries()[0].value().as_bool().unwrap(), true);
    }

    #[test]
    fn test_node_with_float() {
        let doc = kdl! {
            version 1.5
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "version");
        assert_eq!(node.entries()[0].value().as_f64().unwrap(), 1.5);
    }

    #[test]
    fn test_multiple_arguments() {
        let doc = kdl! {
            connect "localhost" 5432 true
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "connect");
        assert_eq!(node.entries().len(), 3);
        assert_eq!(node.entries()[0].value().as_string().unwrap(), "localhost");
        assert_eq!(node.entries()[1].value().as_i64().unwrap(), 5432);
        assert_eq!(node.entries()[2].value().as_bool().unwrap(), true);
    }

    #[test]
    fn test_node_with_properties() {
        let doc = kdl! {
            server host="localhost" port=8080
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "server");
        assert_eq!(node.entries().len(), 2);

        // Properties should have names
        let host_entry = &node.entries()[0];
        assert_eq!(host_entry.name().unwrap().value(), "host");
        assert_eq!(host_entry.value().as_string().unwrap(), "localhost");

        let port_entry = &node.entries()[1];
        assert_eq!(port_entry.name().unwrap().value(), "port");
        assert_eq!(port_entry.value().as_i64().unwrap(), 8080);
    }

    #[test]
    fn test_mixed_args_and_properties() {
        let doc = kdl! {
            database "postgres" version=13 ssl=true
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "database");
        assert_eq!(node.entries().len(), 3);

        // First should be argument
        let arg_entry = &node.entries()[0];
        assert!(arg_entry.name().is_none());
        assert_eq!(arg_entry.value().as_string().unwrap(), "postgres");

        // Rest should be properties
        let version_entry = &node.entries()[1];
        assert_eq!(version_entry.name().unwrap().value(), "version");
        assert_eq!(version_entry.value().as_i64().unwrap(), 13);

        let ssl_entry = &node.entries()[2];
        assert_eq!(ssl_entry.name().unwrap().value(), "ssl");
        assert_eq!(ssl_entry.value().as_bool().unwrap(), true);
    }
}

#[cfg(test)]
mod nested_structure_tests {
    use super::*;

    #[test]
    fn test_empty_children_block() {
        let doc = kdl! {
            parent {}
        };

        let node = &doc.nodes()[0];
        assert_eq!(node.name().value(), "parent");
        assert!(node.children().is_some());
        assert_eq!(node.children().unwrap().nodes().len(), 0);
    }

    #[test]
    fn test_simple_nested_structure() {
        let doc = kdl! {
            config {
                name "my-app"
                debug true
            }
        };

        let parent = &doc.nodes()[0];
        assert_eq!(parent.name().value(), "config");

        let children = parent.children().unwrap();
        assert_eq!(children.nodes().len(), 2);

        let name_child = &children.nodes()[0];
        assert_eq!(name_child.name().value(), "name");
        assert_eq!(name_child.entries()[0].value().as_string().unwrap(), "my-app");

        let debug_child = &children.nodes()[1];
        assert_eq!(debug_child.name().value(), "debug");
        assert_eq!(debug_child.entries()[0].value().as_bool().unwrap(), true);
    }

    #[test]
    fn test_nested_with_parent_properties() {
        let doc = kdl! {
            server host="localhost" port=8080 {
                ssl true
                timeout 30
            }
        };

        let server = &doc.nodes()[0];
        assert_eq!(server.name().value(), "server");
        assert_eq!(server.entries().len(), 2);

        // Check parent properties
        assert_eq!(server.entries()[0].name().unwrap().value(), "host");
        assert_eq!(server.entries()[0].value().as_string().unwrap(), "localhost");
        assert_eq!(server.entries()[1].name().unwrap().value(), "port");
        assert_eq!(server.entries()[1].value().as_i64().unwrap(), 8080);

        // Check children
        let children = server.children().unwrap();
        assert_eq!(children.nodes().len(), 2);

        let ssl_child = &children.nodes()[0];
        assert_eq!(ssl_child.name().value(), "ssl");
        assert_eq!(ssl_child.entries()[0].value().as_bool().unwrap(), true);

        let timeout_child = &children.nodes()[1];
        assert_eq!(timeout_child.name().value(), "timeout");
        assert_eq!(timeout_child.entries()[0].value().as_i64().unwrap(), 30);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_negative_numbers() {
        let doc = kdl! {
            negative -42
            negative_float -3.14
        };

        assert_eq!(doc.nodes()[0].entries()[0].value().as_i64().unwrap(), -42);
        assert_eq!(doc.nodes()[1].entries()[0].value().as_f64().unwrap(), -3.14);
    }

    #[test]
    fn test_empty_string() {
        let doc = kdl! {
            empty_string ""
        };

        assert_eq!(doc.nodes()[0].entries()[0].value().as_string().unwrap(), "");
    }

    #[test]
    fn test_zero_values() {
        let doc = kdl! {
            zero 0
            zero_float 0.0
        };

        assert_eq!(doc.nodes()[0].entries()[0].value().as_i64().unwrap(), 0);
        assert_eq!(doc.nodes()[1].entries()[0].value().as_f64().unwrap(), 0.0);
    }
}
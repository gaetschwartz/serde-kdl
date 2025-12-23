use insta::assert_snapshot;
use serde::{Deserialize, Serialize};
use serde_kdl::{from_str, to_string_pretty};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    origin: (Point, Point),
    background_color: (Color, f32),
    foreground_color: (Color, f32),
    axis_config: (AxisConfig, AxisConfig),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AxisConfig {
    label: String,
    range: (f64, f64),
}

#[test]
fn test_complex_tuple() {
    let color = Config {
        origin: (Point { x: 0, y: 0 }, Point { x: 100, y: 100 }),
        background_color: (Color::Red, 1.0),
        foreground_color: (Color::Green, 0.5),
        axis_config: (
            AxisConfig {
                label: "X-Axis".to_string(),
                range: (0.0, 10.0),
            },
            AxisConfig {
                label: "Y-Axis".to_string(),
                range: (-5.0, 5.0),
            },
        ),
    };
    let serialized = to_string_pretty(&color).unwrap();
    assert_snapshot!(serialized, @r#"
    origin {
        - {
            x 0
            y 0
        }
        - {
            x 100
            y 100
        }
    }
    background_color {
        (Red)-
        - 1.0
    }
    foreground_color {
        (Green)-
        - 0.5
    }
    axis_config {
        - {
            label X-Axis
            range 0.0 10.0
        }
        - {
            label Y-Axis
            range -5.0 5.0
        }
    }
    "#);

    let deserialized: Config = from_str(&serialized).unwrap();
    assert_eq!(color, deserialized);
}

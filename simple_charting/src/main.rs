extern crate rasciigraph;

use rasciigraph::{plot, Config};
// println!(
//     "{}",
//     plot(
//         vec![
//             0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 9.0, -3.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0,
//             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 8.0, -3.0, 0.0, 0.0, 1.0,
//             2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 10.0, -3.0, 0.0,
//             0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0
//         ],
//         Config::default()
//             .with_offset(10)
//             .with_height(10)
//             .with_width(80)
//             .with_caption("I'm a doctor, not an engineer.".to_string())
//     )
// );
fn main() {
    let cities = vec![
        "Lisbon",
        "Madrid",
        "Paris",
        "Berlin",
        "Copenhagen",
        "Stockholm",
        "Moscow",
    ];
    let distance_traveled = vec![0.0, 502.56, 1053.36, 187.27, 2636.42, 3117.23, 4606.35];

    println!("{}", cities.join(" > "));

    println!(
        "{}",
        plot(
            distance_traveled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_width(80)
                .with_caption("Distance traveled in km".to_string())
        )
    );
}

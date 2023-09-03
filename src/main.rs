extern crate itertools;

use itertools::Itertools; // Import the itertools crate to perform permutations
use std::time::Instant;

struct City {
    name: String,
    x: f64,
    y: f64,
}

impl City {
    fn new(name: &str, x: f64, y: f64) -> City {
        City {
            name: name.to_string(),
            x,
            y,
        }
    }
}

fn euclidean_distance(city1: &City, city2: &City) -> f64 {
    let dx = city1.x - city2.x;
    let dy = city1.y - city2.y;
    (dx * dx + dy * dy).sqrt()
}

fn total_distance(tour: &[&City]) -> f64 {
    let mut distance = 0.0;
    for i in 0..tour.len() - 1 {
        distance += euclidean_distance(tour[i], tour[i + 1]);
    }
    distance += euclidean_distance(tour[tour.len() - 1], tour[0]); // Return to the starting city
    distance
}

fn main() {
    let start = Instant::now();

    let cities = vec![
        City::new("A", 0.0, 0.0),
        City::new("B", 2.0, 1.0),
        City::new("C", 2.0, 3.0),
        City::new("D", 1.0, 2.0),
        City::new("E", 0.5, 2.0),
        City::new("F", 0.0, 1.5),
    ];

    let mut shortest_tour = Vec::new();
    let mut shortest_distance = f64::INFINITY;

    for perm in cities.iter().permutations(cities.len()) {
        let tour: Vec<&City> = perm.into_iter().collect();
        let distance = total_distance(&tour);

        if distance < shortest_distance {
            shortest_distance = distance;
            shortest_tour = tour;
        }
    }

    println!(
        "Shortest tour: {:?}",
        shortest_tour.iter().map(|city| city.name.as_str()).collect::<Vec<&str>>()
    );
    println!("Shortest distance: {:.2}", shortest_distance);

    let elapsed = start.elapsed();

    println!("Alrorithm runtime with an exponential time complexity: {:?}", elapsed);
}


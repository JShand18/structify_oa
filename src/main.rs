use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let rads = vec![0.73, 1.47, 1.77, 3.92];
    let ids = vec![String::from("s1"), String::from("s2"), String::from("e1"), String::from("e2")];

//     intersecting_chords_tuple(&rads, &ids);
    intersecting_chords_hashmap(&rads, &ids);
}

fn intersecting_chords_tuple(point_rads: &Vec<f32>, point_ids: &Vec<String>) -> i32{

    // Mapping the identifiers and the radian measurement of the points into a tuple
    let points: Vec<(&String, &f32)> = std::iter::zip(point_ids, point_rads).collect();

    for point in points{
        println!("{}, {}", point.0, point.1)
    }


    0
}

fn intersecting_chords_hashmap(point_rads: &Vec<f32>, point_ids: &Vec<String>) -> i32{

    // Mapping the identifiers and radian measures of the points into a hashmap
    let points: HashMap<&String, &f32> = point_ids.into_iter().zip(point_rads.iter()).collect();
    // Initializing Vector to store tuples representing the chords
    let mut chords: Vec<(&f32, &f32)> = Vec::new();

    // Iterating through the map of points to group the start and end of the chords in a tuple
    for (id, rad) in &points{
        // Only care about finding the start points for each chord
        if id[0..1] == String::from("s") {
            // String concatenation to form key for endpoint
            let mut end = String::from("e");
            end.push_str(&id[1..2]);

            // HashMap quicker retrieval
            chords.push((rad, points.get(&end).unwrap()));

            // Sanity check to ensure that the points are matched properly
            println!("{}, {}",&rad, points.get(&end).unwrap());
        }
    }


    0
}

// #[test]
// fn unit_test(){
//     assert_eq!(intersecting_chords(), 0);
// }


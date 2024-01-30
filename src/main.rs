use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let rads = vec![0.28, 0.73, 1.12, 1.47, 1.77, 3.92];
    let ids = vec![String::from("s1"), String::from("e1"), String::from("s2"), String::from("e2"), String::from("s3"), String::from("e3")];

    let chords: Vec<(&f32, &f32)> = connect_chords(&rads, &ids);
    count_intersections(chords);
}


fn count_intersections(chords: Vec<(&f32, &f32)>) -> i32 {
    // Store for the final count of intersections
    let mut count: i32 = 0;

    // Two pointers to check intersecting chords
    // Once the major chord has checked for intersection of minor chords
    // minor chords do not need to go back and check major chord
    for i in 0..chords.len()-1{
        // Sanity check of loop logic
        println!("----{}", &i);
        println!("start:{}, end:{}", chords[i].0, chords[i].1);
        for j in &i+1..chords.len(){
            println!("\tstart:{}, end:{}", chords[j].0, chords[j].1);
            let check: bool = is_intersection(chords[i], chords[j]);
            if check{
                count += 1;
            }
            // Sanity check of divsor logic
            println!("{}", check);
            println!("{}", count);
        }
    }
    count
}

fn is_intersection(major: (&f32, &f32), minor: (&f32, &f32)) -> bool{
    // Interpreting the major chord as dividing the circle in half
    // Checking if the points of the minor chord both exists on the same half circle
    if (major.0 < minor.0 && minor.0 < major.1) && (major.0 < minor.1 && minor.1 < major.1){
        false
    } else if (minor.0 < major.0 || minor.0 > major.1) && (minor.1 < major.0 || minor.1 > major.1){
        false
    } else{
        true
    }
}


// Mapping of radian measures and identifiers using a hashmap for quick retrieval and search
fn connect_chords<'a>(point_rads: &'a Vec<f32>, point_ids: &'a Vec<String>) -> Vec<(&'a f32, &'a f32)>{
    // Mapping the identifiers and radian measures of the points into a hashmap
    let points: HashMap<&String, &f32> = point_ids.into_iter().zip(point_rads.into_iter()).collect();
    // Initializing Vector to store tuples representing the chords
    let mut chords: Vec<(&f32, &f32)> = Vec::new();

    // Iterating through the map of points to group the start and end of the chords in a tuple
    for (id, rad) in &points{
        // Only care about finding the start points for each chord
        if id[0..1] == String::from("s") {
            // String concatenation to form key for end point
            let mut end = String::from("e");
            end.push_str(&id[1..2]);
            let end_rad: &f32 = points.get(&end).unwrap();
            // Ordering the tuple so the lower radian measure is first
            if rad > &end_rad{
                chords.push((end_rad, rad));
                println!("{}, {}",&end_rad, &rad);
            } else{
                chords.push((rad, end_rad));
                println!("{}, {}",&rad, &end_rad);
            }
        }
    }
    // returning the vector of chords
    chords
}

// #[test]
// fn unit_test(){
//     assert_eq!(intersecting_chords(), 0);
// }

// fn intersecting_chords_tuple(point_rads: &Vec<f32>, point_ids: &Vec<String>) -> i32{
//
//     // Mapping the identifiers and the radian measurement of the points into a tuple
//     let points: Vec<(&String, &f32)> = std::iter::zip(point_ids, point_rads).collect();
//
//     for point in points{
//         println!("{}, {}", point.0, point.1)
//     }
//     0
// }



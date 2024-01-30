use std::collections::HashMap;

fn main() {
    let rads = vec![0.28, 0.73, 1.12, 1.47, 1.77, 3.92];
    let ids = vec![String::from("s1"), String::from("e1"), String::from("s2"), String::from("e2"), String::from("s3"), String::from("e3")];

    println!("{} Intersections", how_many_intersections((rads, ids)));
}

fn how_many_intersections(list_of_chords: (Vec<f32>, Vec<String>)) -> i32{
    count_intersections(connect_chords(&list_of_chords.0, &list_of_chords.1))
}

fn count_intersections(chords: Vec<(&f32, &f32)>) -> i32 {
    // Store for the final count of intersections
    let mut count: i32 = 0;
    // Two pointers to check intersecting chords
    // Once the major chord has checked for intersection of minor chords
    // minor chords do not need to go back and check major chord
    for i in 0..chords.len()-1{
        for j in i+1..chords.len(){
            if is_intersection(chords[i], chords[j]){ count += 1; }
        }
    }
    // returning the count of intersections
    return count;
}

// Identifying any intersections with the current "major" chord with the rest of the "minor" chords
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
            }
        }
    }
    // returning the vector of chords
    return chords;
}

#[test]
fn unit_test(){
    let rads = vec![0.28, 0.73, 1.12, 1.47, 1.77, 3.92];
    let ids = vec![String::from("s1"), String::from("e1"), String::from("s2"), String::from("e2"), String::from("s3"), String::from("e3")];
    assert_eq!(how_many_intersections((rads, ids)), 0);
}




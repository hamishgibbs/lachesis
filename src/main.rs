use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Record {
    id: i64,
    time: i64,
    point: Point,   
} 

struct Visit {
    id: i64,
    start_time: i64,
    end_time: i64,
    point: Point,
}

fn read_stdin_data() -> Vec<Record> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();
    let mut data = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        let mut parts = line.split(",");
        let record = Record {
            id: parts.next().unwrap().parse::<i64>().unwrap(),
            time: parts.next().unwrap().parse::<i64>().unwrap(),
            point: Point {
                x: parts.next().unwrap().parse::<f64>().unwrap(),
                y: parts.next().unwrap().parse::<f64>().unwrap(),
            },
        };
        data.push(record);
        line.clear();
    }
    data
}

/// Divide vector of records into a vector of vectors of records, by id.
fn divide_id_records(data: &Vec<Record>) -> Vec<Vec<Record>> {
    let mut id_records = Vec::new();

    for id in data[0].id..=data[data.len()-1].id {
        let records = data.iter().filter(|record| record.id == id).cloned().collect::<Vec<Record>>();
        if !records.is_empty() {
            id_records.push(records);
        }
    }
    id_records
}

fn max(values: &Vec<f64>) -> f64 {
    values.iter().copied().fold(f64::NAN, f64::max)
}

fn min(values: &Vec<f64>) -> f64 {
    values.iter().copied().fold(f64::NAN, f64::min)
}

/// Calculate median of an f64 vector
fn median(values: &Vec<f64>) -> f64 {
    let mut values = values.clone();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[mid] + values[mid - 1]) / 2.0
    } else {
        values[mid]
    }
}

/// Calculate euclidean distance between two points
fn calculate_distance(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn merge_records_to_visit(records: Vec<Record>) -> Visit {
    Visit { id: records[0].id, 
        start_time: records[0].time, 
        end_time: records[records.len()-1].time, 
        point: Point { 
            x: median(&records.iter().map(|record| record.point.x).collect()), 
            y: median(&records.iter().map(|record| record.point.y).collect())} }
}

/// Detect sequential stay points for a vector of records
fn detect_stay_points(
    records: Vec<Record>, 
    max_distance: f64,
    min_time: i64) -> Vec<Visit> {

    let mut visits = Vec::new();

    let mut i = 0;
    let mut j = 1;

    while j <= records.len() {

        let x_coords: Vec<f64> = records[i..j].iter().map(|record| record.point.x).collect();
        let y_coords: Vec<f64> = records[i..j].iter().map(|record| record.point.x).collect();

        let visit_diameter = calculate_distance(
            &Point { x: min(&x_coords), y: min(&y_coords) },
            &Point { x: max(&x_coords), y: max(&y_coords) }
        );

        if visit_diameter < max_distance {
            // Check whether the final points in the sequence constitute a valid visit
            if j == records.len() {
                let visit_duration = records[j-1].time - records[i].time;
                if visit_duration >= min_time {
                    visits.push(merge_records_to_visit(records[i..j].to_vec()));
                }
                j += 1;
            } else {
                j += 1;
            }
        } else {
            let visit_duration = records[j-2].time - records[i].time;
            if visit_duration >= min_time {
                visits.push(merge_records_to_visit(records[i..j-1].to_vec()));
            } 
            i = j - 1;
        }

    }
    
    visits
}


fn main() {

    // how do you capture parameters and also stream in from stdin?
    //let max_distance = std::env::args().nth(1).unwrap().parse::<f64>().unwrap();
    //let min_time = std::env::args().nth(2).unwrap().parse::<i64>().unwrap();

    let mut visits = Vec::new();

    let data = read_stdin_data();

    let id_records = divide_id_records(&data);

    for records in id_records {
        visits.push(detect_stay_points(records, 2.0, 1));
    }
    
    let mut stdout = io::stdout();
    for visit in visits.into_iter().flatten().collect::<Vec<Visit>>() {
        writeln!(stdout, "{},{},{},{},{}\n", visit.id, visit.start_time, visit.end_time, visit.point.x, visit.point.y).unwrap();
    }
}

#[test]
fn test_median() {
    let values = vec![1.0, 2.0, 3.0, 5.0];
    let median = median(&values);
    assert_eq!(median, 2.5);
}

#[test]
fn test_calculate_distance() {
    let x = Point{x: 1.0, y: 1.0};
    let y = Point{x: 5.0, y: 5.0};

    let distance = calculate_distance(&x, &y);
    assert_eq!(distance.round(), 6.0);
}

#[test]
fn test_calculate_distance_same_point() {
    let x = Point{x: 1.0, y: 1.0};
    let y = Point{x: 1.0, y: 1.0};

    let distance = calculate_distance(&x, &y);
    assert_eq!(distance.round(), 0.0);
}

#[test]
fn test_divide_id_records() {
    let records = vec![
        Record{id: 1, time: 1, point: Point{x: 1.0, y: 1.0}}, 
        Record{id: 1, time: 2, point: Point{x: 2.0, y: 2.0}}, 
        Record{id: 2, time: 1, point: Point{x: 3.0, y: 3.0}}];

    let id_records = divide_id_records(&records);
    
    assert_eq!(id_records.len(), 2);
    assert_eq!(id_records[0].len(), 2);
    assert_eq!(id_records[1].len(), 1);
}

#[test]
fn test_detect_stay_points_one_visit_trailing_pt() {
    let records = vec![
        Record{id: 1, time: 1, point: Point{x: 1.0, y: 1.0}}, 
        Record{id: 1, time: 2, point: Point{x: 2.0, y: 2.0}}, 
        Record{id: 1, time: 3, point: Point{x: 5.0, y: 5.0}}];
    
    let visits = detect_stay_points(records, 2.0, 1);
    assert_eq!(visits.len(), 1);
    assert_eq!(visits[0].start_time, 1);
    assert_eq!(visits[0].end_time, 2);
    assert_eq!(visits[0].point.x, 1.5);
    assert_eq!(visits[0].point.y, 1.5);
}

#[test]
fn test_detect_stay_points_two_visits_no_trailing_pt() {
    let records = vec![
        Record{id: 1, time: 1, point: Point{x: 1.0, y: 1.0}}, 
        Record{id: 1, time: 2, point: Point{x: 2.0, y: 2.0}}, 
        Record{id: 1, time: 3, point: Point{x: 5.0, y: 5.0}},
        Record{id: 1, time: 4, point: Point{x: 10.0, y: 10.0}},
        Record{id: 1, time: 5, point: Point{x: 11.0, y: 11.0}},
        Record{id: 1, time: 6, point: Point{x: 10.0, y: 10.0}}];
    
    let visits = detect_stay_points(records, 2.0, 1);
    assert_eq!(visits.len(), 2);
    assert_eq!(visits[0].start_time, 1);
    assert_eq!(visits[0].end_time, 2);
    assert_eq!(visits[0].point.x, 1.5);
    assert_eq!(visits[0].point.y, 1.5);
    assert_eq!(visits[1].start_time, 4);
    assert_eq!(visits[1].end_time, 6);
    assert_eq!(visits[1].point.x, 10.0);
    assert_eq!(visits[1].point.y, 10.0);
}
use std::io;
use std::io::BufRead;
use std::io::Write;
use chrono::NaiveDateTime;
use clap::Parser;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Record {
    id: String,
    time: i64,
    point: Point,   
} 

struct Visit {
    id: String,
    start_time: i64,
    end_time: i64,
    point: Point,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Maximum spatial roam of a stop point (i.e. 200m)
    #[arg(short, long)]
    distance: f64,

    /// Minimum time duration of a stop point (i.e. 300 seconds)
    #[arg(short, long)]
    time: i64,

    /// Format of dates in the input file (i.e. "%Y-%m-%d %H:%M:%S")
    #[arg(short, long)]
    fmt_time: String
}

fn read_stdin_data<R>(mut reader: R, date_fmt: &String) -> Vec<Record> where R: BufRead {
    let mut line = String::new();
    let mut data = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        line.pop();

        let parts = line.split(",").collect::<Vec<&str>>();

        let x = parts[2].parse::<f64>();

        // Assume that if x is not a number, this is the header row
        if let Err(_err) = x {
            line.clear();
            continue;
        };

        let record = Record {
            id: parts[0].parse::<String>().unwrap(),
            time: NaiveDateTime::parse_from_str(&parts[1].parse::<String>().unwrap(), date_fmt).unwrap().timestamp(),
            point: Point {
                x: x.unwrap(),
                y: parts[3].parse::<f64>().unwrap(),
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
    
    let mut i = 0;
    for j in 0..data.len() {
        if data[i].id != data[j].id {
            id_records.push(data[i..j].to_vec());
            i = j;
        }
    }
    id_records.push(data[i..data.len()].to_vec());
    
    id_records
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
    Visit { id: records[0].id.clone(), 
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

    let mut visits: Vec<Visit> = Vec::new();

    let mut i = 0;
    let mut j = 1;
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (records[i].point.x, records[i].point.x, records[i].point.y, records[i].point.y);

    while j <= records.len() {

        if records[j-1].point.x < min_x {
            min_x = records[j-1].point.x;
        }
        if records[j-1].point.x > max_x {
            max_x = records[j-1].point.x;
        }
        if records[j-1].point.y < min_y {
            min_y = records[j-1].point.y;
        }
        if records[j-1].point.y > max_y {
            max_y = records[j-1].point.y;
        }

        let visit_diameter = calculate_distance(
            &Point { x: min_x, y: min_y },
            &Point { x: max_x, y: max_y }
        );

        if visit_diameter < max_distance {
            if j == records.len() {
                let visit_duration = records[j-1].time - records[i].time;
                if visit_duration >= min_time {
                    visits.push(merge_records_to_visit(records[i..j].to_vec()));
                }
            }
            j += 1;
            
        } else {
            let visit_duration = records[j-2].time - records[i].time;
            if visit_duration >= min_time {
                visits.push(merge_records_to_visit(records[i..j-1].to_vec()));
            }
            i = j - 1;
            min_x = records[i].point.x;
            max_x = records[i].point.x;
            min_y = records[i].point.y;
            max_y = records[i].point.y;
        }

    }
    
    visits
}

fn fmt_visits_csv(visits: Vec<Visit>, date_fmt: &String) -> Vec<String> {
    let mut output = Vec::new();
    for visit in visits {
        output.push(format!("{},{},{},{},{},{}", 
            visit.id, 
            NaiveDateTime::from_timestamp_opt(visit.start_time, 1).unwrap().format(date_fmt), 
            NaiveDateTime::from_timestamp_opt(visit.end_time, 1).unwrap().format(date_fmt), 
            visit.point.x, 
            visit.point.y, 
            visit.end_time - visit.start_time));
    }
    output
}

fn main() {
    let args = Args::parse();

    let stdin = io::stdin();
    let reader = stdin.lock();

    let data = read_stdin_data(reader, &args.fmt_time);

    let id_records = divide_id_records(&data);

    let mut visits = Vec::new();

    for records in id_records {
        visits.push(detect_stay_points(
                records, 
                args.distance, 
                args.time));
    }
    
    let mut stdout = io::stdout();

    writeln!(stdout, "id,start,end,x,y,duration").unwrap();
    for ln in fmt_visits_csv(visits.into_iter().flatten().collect(), &args.fmt_time) {
        writeln!(stdout, "{}", ln).unwrap(); 
    }
}

#[test]
fn test_read_multiline_stdin_data() {
    let mut input = String::new();
    input.push_str("a,2020-01-01 10:00:00,1.0,1.0\nb,2020-01-01 16:00:00,1.0,1.0");
    let date_fmt = String::from("%Y-%m-%d %H:%M:%S");

    let data = read_stdin_data(
        &mut input.as_bytes(),
        &date_fmt
    );
    
    assert_eq!(data.len(), 2);
    assert_eq!(data[0].id, String::from("a"));
    assert_eq!(data[0].time, 1577872800);
    assert_eq!(data[0].point.x, 1.0);
    assert_eq!(data[0].point.y, 1.0);
    assert_eq!(data[1].id, String::from("b"));
    assert_eq!(data[1].time, 1577894400);
    assert_eq!(data[1].point.x, 1.0);
    assert_eq!(data[1].point.y, 1.0);
}

#[test]
fn test_read_multiline_stdin_data_header() {
    let mut input = String::new();
    input.push_str("id,time,x,y\nb,2020-01-01 16:00:00,1.0,1.0");
    let date_fmt = String::from("%Y-%m-%d %H:%M:%S");

    let data = read_stdin_data(
        &mut input.as_bytes(),
        &date_fmt
    );
    
    assert_eq!(data.len(), 1);
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
        Record{id: String::from("a"), time: 1, point: Point{x: 1.0, y: 1.0}}, 
        Record{id: String::from("a"), time: 2, point: Point{x: 2.0, y: 2.0}}, 
        Record{id: String::from("b"), time: 1, point: Point{x: 3.0, y: 3.0}},
        Record{id: String::from("c"), time: 1, point: Point{x: 3.0, y: 3.0}},
        Record{id: String::from("c"), time: 2, point: Point{x: 3.0, y: 3.0}}];

    let id_records = divide_id_records(&records);
    
    assert_eq!(id_records.len(), 3);
    assert_eq!(id_records[0].len(), 2);
    assert_eq!(id_records[1].len(), 1);
    assert_eq!(id_records[2].len(), 2);
    assert_eq!(id_records[0][0].id, String::from("a"));
    assert_eq!(id_records[1][0].id, String::from("b"));
    assert_eq!(id_records[2][0].id, String::from("c"));
}

#[test]
fn test_detect_stay_points_one_visit_trailing_pt() {
    let records = vec![
        Record{id: String::from("a"), time: 1, point: Point{x: 1.0, y: 1.0}}, 
        Record{id: String::from("a"), time: 2, point: Point{x: 2.0, y: 2.0}}, 
        Record{id: String::from("a"), time: 3, point: Point{x: 5.0, y: 5.0}}];
    
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
        Record{id: String::from("a"), time: 1, point: Point{x: 1.0, y: 1.0}}, 
        Record{id: String::from("a"), time: 2, point: Point{x: 2.0, y: 2.0}}, 
        Record{id: String::from("a"), time: 3, point: Point{x: 5.0, y: 5.0}},
        Record{id: String::from("a"), time: 4, point: Point{x: 10.0, y: 10.0}},
        Record{id: String::from("a"), time: 5, point: Point{x: 11.0, y: 11.0}},
        Record{id: String::from("a"), time: 6, point: Point{x: 10.0, y: 10.0}}];
    
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

#[test]
fn test_fmt_visits_csv() {
    let visits = vec![
        Visit{id: String::from("a"), start_time: 1577872800, end_time: 1577894400, point: Point{x: 1.5, y: 1.5}}, 
        Visit{id: String::from("b"), start_time: 1577872800, end_time: 1577894400, point: Point{x: 1.5, y: 1.5}}];
    let date_fmt = String::from("%Y-%m-%d %H:%M:%S");

    let csv = fmt_visits_csv(visits, &date_fmt);
    assert_eq!(csv.len(), 2);
    assert_eq!(csv[0], String::from("a,2020-01-01 10:00:00,2020-01-01 16:00:00,1.5,1.5,21600"));
    assert_eq!(csv[1], String::from("b,2020-01-01 10:00:00,2020-01-01 16:00:00,1.5,1.5,21600"));
}
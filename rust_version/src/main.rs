use std::error::Error;
use plotters::prelude::*;
use serde::Deserialize;


#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)] // <-- Add this
struct Record {
    Employee_Name: Option<String>,
    EmpID: Option<String>,  // This could be f64 or i32 if it's a numerical ID
    MarriedID: Option<i32>,
    MaritalStatusID: Option<i32>,
    GenderID: Option<i32>,
    EmpStatusID: Option<i32>,
    DeptID: Option<i32>,
    PerfScoreID: Option<i32>,
    FromDiversityJobFairID: Option<i32>,
    Salary: Option<f64>,
    Termd: Option<i32>,
    PositionID: Option<i32>,
    Position: Option<String>,
    State: Option<String>,
    Zip: Option<String>,  // This could be i32 if it's a numerical Zip code
    DOB: Option<String>,
    Sex: Option<String>,
    MaritalDesc: Option<String>,
    CitizenDesc: Option<String>,
    HispanicLatino: Option<String>,
    RaceDesc: Option<String>,
    DateofHire: Option<String>,
    DateofTermination: Option<String>,
    TermReason: Option<String>,
    EmploymentStatus: Option<String>,
    Department: Option<String>,
    ManagerName: Option<String>,
    ManagerID: Option<i32>,
    RecruitmentSource: Option<String>,
    PerformanceScore: Option<String>,
    EngagementSurvey: Option<f64>,
    EmpSatisfaction: Option<f64>,
    SpecialProjectsCount: Option<i32>,
    LastPerformanceReview_Date: Option<String>,
    DaysLateLast30: Option<i32>,
    Absences: Option<i32>
}


fn calculate_statistics(records: &Vec<Record>) -> Result<(), Box<dyn Error>> {
    let mut sum = 0.0;
    let mut count = 0;

    for record in records {
        if let Some(salary) = record.Salary {
            sum += salary;
            count += 1;
        }
    }

    let mean = sum / count as f64;

    println!("Mean Salary: {}", mean);

    Ok(())
}


fn visualize_data(records: &Vec<Record>) -> Result<(), Box<dyn Error>> {
    // Visualize `Salary`
    let data_points: Vec<(usize, f64)> = records.iter()
        .enumerate()
        .filter_map(|(index, record)| {
            record.Salary.map(|value| (index, value))
        })
        .collect();

    // Create a root drawing area
    let root = BitMapBackend::new("salary_plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine min and max for x and y axis
    let max_x = data_points.len();
    let max_y = data_points.iter().map(|(_, y)| *y).fold(f64::NAN, f64::max);
    let min_y = data_points.iter().map(|(_, y)| *y).fold(f64::NAN, f64::min);

    // Setting up the chart context, axes, etc.
    let mut chart = ChartBuilder::on(&root)
        .caption("Line Plot for Salary", ("Arial", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Plotting data
    chart.draw_series(LineSeries::new(data_points, &RED))?;

    root.present()?; // Save the plot

    Ok(())
}

use csv::Reader;

fn main() {
    // Absolute path to your CSV file
    let filepath = "/workspaces/ZT-DE-week8-miniproject8/HRDataset_v14.csv";

    // Attempt to read the CSV data
    let records = match read_csv_into_records(filepath) {
        Ok(r) => r,
        Err(e) => {
            println!("Error reading CSV: {}", e);
            return;
        }
    };

    // Visualization
    if let Err(err) = visualize_data(&records) {
        println!("Visualization Error: {}", err);
    }

    // Statistics
    if let Err(err) = calculate_statistics(&records) {  // Corrected here
        println!("Statistics Error: {}", err);
    }
}


fn read_csv_into_records(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }

    Ok(records)
}



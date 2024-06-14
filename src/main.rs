
use serde::Deserialize;
use rustlearn::prelude::*; use rustlearn::datasets::iris;
use rustlearn::svm::libsvm::svc::{Hyperparameters, KernelType};
use std::{
    error::Error,
    process,
};

// struct representing dataset
#[derive(Debug, Deserialize)]
struct Iris {
    #[serde(rename = "sepal.length")]
    sepal_length: f32,
    #[serde(rename = "sepal.width")]
    sepal_width: f32,
    #[serde(rename = "petal.length")]
    petal_lenght: f32, 
    #[serde(rename = "petal.width")]
    petal_width: f32,
    variety: String
}

// function to read data and train data
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("iris.csv")?;

    let mut x: Vec<Vec<f32>> = Vec::new();
    let mut y: Vec<f32> = Vec::new();

    let mut count = 0;
    let setosa = "Setosa".to_string();
    let versicolor = "Versicolor".to_string();

    // reading data and adding rows to vec
    for result in rdr.deserialize() {
        let record: Iris = result?;
        x.push(Vec::new());
        x[count].push(record.sepal_length);
        x[count].push(record.sepal_width);
        x[count].push(record.petal_lenght);
        x[count].push(record.petal_width);
       
       if record.variety == setosa{
            y.push(0.0);
       }else if record.variety == versicolor{
            y.push(1.0);
       }else{
            y.push(2.0)
       }

       count += 1;

        println!("{:?}", record);
    }

    // train model
    let (x, y) = (Array::from(&x), Array::from(y));

    let mut model = Hyperparameters::new(4, KernelType::Linear, 3)
        .C(0.3)
        .build();

    model.fit(&x, &y).unwrap();

    let prediction = model.predict(&x).unwrap();

    println!("acc = {}", rustlearn::metrics::accuracy_score(&y, &prediction));

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
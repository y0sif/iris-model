# Iris model
iris dataset columns look like this:

|sepal.length|sepal.width|petal.length|petal.width|variety|
| :----------| :-------: | :--------: | :-------: | ----: |

# How to use it Rust?

using csv crate will read iris.csv file

```
let mut rdr = csv::Reader::from_path("iris.csv")?;
```

make Iris struct to hold each record

```
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
```

```
for result in rdr.deserialize() {
        let record: Iris = result?;
        // add each record to vec
}
```

finally, will use rustlearn crate to train the model

# dependencies used

```
[dependencies]
rustlearn = "*"
csv = "1.3"
serde = {version = "*", features = ["derive"]}
```

# Apply what you have learned

[checkout thyroid template](https://github.com/y0sif/thyroid-model-template)

# More info?

[check csv crate docs](https://docs.rs/csv/latest/csv/)

[check rustlearn crate docs](https://docs.rs/rustlearn/latest/rustlearn/)



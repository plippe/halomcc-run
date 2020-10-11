use serde::de::DeserializeOwned;

pub fn csv<A: DeserializeOwned>(path: &str) -> Vec<A> {
    csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)
        .map(|mut it| it.deserialize::<A>().flatten().collect())
        .unwrap_or_else(|err| panic!("Failed parsing CSV file: {}, {}", path, err))
}

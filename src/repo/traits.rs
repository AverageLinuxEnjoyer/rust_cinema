pub trait Serializable {
    fn to_csv(&self) -> String;
    fn from_csv_to_obj(s: &str) -> Self;
}

pub fn mat_data() {
    let mut df = DataFrame::with_header(vec!["r0", "E", "EN"]);

    df.write_nc("../../data/test.nc").expect("can'tw rite");
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

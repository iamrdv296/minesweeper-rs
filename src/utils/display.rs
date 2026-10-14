pub fn add_zeros(num: usize, digits: usize) -> String {
    if format!("{}", num).len() < digits {
        format!("{:0width$}", num, width = digits)
    } else {
        format!("{}", num)
    }
}
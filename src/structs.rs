

#[derive(Debug)]
pub struct Choice {
    pub value: usize,
    pub lable: String,
}

impl Choice {
    pub(crate) fn new() -> Self {
        Self {
            value: 0,
            lable: String::new(),
        }
    }
}


#[derive(Debug)]
pub struct Response<T> {
    count: usize,
    result: Vec<T>,
}
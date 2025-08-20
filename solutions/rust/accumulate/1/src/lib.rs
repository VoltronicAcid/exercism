pub fn map<T, F, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut output: Vec<U> = Vec::new();

    for val in input.into_iter() {
        output.push(function(val));
    }

    output
}

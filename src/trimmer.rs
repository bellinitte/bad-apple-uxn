pub fn trim_frames(mut data: Vec<Vec<u8>>, size: usize) -> Vec<Vec<u8>> {
    data = data
        .into_iter()
        .skip_while(|frame| is_empty(frame))
        .collect();

    if let Some(position) = data.iter().rposition(|frame| !is_empty(frame)) {
        data.resize(position + 1, Vec::new());
        data.push(generate_empty(size));
        return data;
    }

    return Vec::new();
}

fn is_empty(data: &[u8]) -> bool {
    data.iter()
        .skip(1)
        .step_by(2)
        .map(|x| *x as usize)
        .sum::<usize>()
        == 0
}

fn generate_empty(mut size: usize) -> Vec<u8> {
    let mut result = Vec::new();

    while size > u8::MAX as usize {
        result.push(u8::MAX);
        result.push(0);
        size -= u8::MAX as usize;
    }

    result.push(size as u8);

    result
}

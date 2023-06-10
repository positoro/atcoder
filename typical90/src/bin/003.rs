fn main() {
    let n = input_u32();
    let a_b_tuple_vector: Vec<(u32, u32)> = input_u32_tuple_vector(n);
    let tuple_vector: Vec<(u32, u32)> = set_tuple_vector_zero_index(a_b_tuple_vector);
    let graph = create_graph(n, tuple_vector);
    let distance_from_zero_vector: Vec<u32> = dfs(&graph, 0);

    let (max_distance_node_from_zero, max_distance_from_zero): (u32, u32) =
        get_max_distance_node(distance_from_zero_vector);

    let distance_from_max_distance_vector: Vec<u32> = dfs(&graph, max_distance_node_from_zero);
    let (max_distance_node_from_max_distance, max_distance_from_max_distance): (u32, u32) =
        get_max_distance_node(distance_from_max_distance_vector);

    println!("{}", max_distance_from_max_distance);
}

////////////////////////////////////////////////////////////////////////////////
fn dfs(graph: &Vec<Vec<u32>>, from_node: u32) -> Vec<u32> {
    let return_vector: Vec<u32> = Vec::new();
    return return_vector;
}
fn get_max_distance_node(distance_vector: Vec<u32>) -> (u32, u32) {
    let mut max_node_index: i32 = -1;
    let mut max_distance: u32 = 0;

    for (index, value) in distance_vector.iter().enumerate() {
        if max_distance < *value {
            max_distance = *value;
            max_node_index = index as i32;
        }
    }
    return (max_node_index as u32, max_distance);
}

fn set_tuple_vector_zero_index(a_b_tuple_vector: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut return_tuple_vector: Vec<(u32, u32)> = Vec::new();

    for v in a_b_tuple_vector.iter() {
        return_tuple_vector.push((v.0 - 1, v.1 - 1));
    }

    return return_tuple_vector;
}

fn create_graph(n: u32, a_b_tuple_vector: Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    let mut return_vec_vec: Vec<Vec<u32>> = vec![Vec::new(); n as usize];

    for v in a_b_tuple_vector.iter() {
        return_vec_vec[v.0 as usize].push(v.1);
        return_vec_vec[v.1 as usize].push(v.0);
    }

    return return_vec_vec;
}

fn input_u32_tuple_vector(n: u32) -> Vec<(u32, u32)> {
    let mut input_strings = String::new();
    let mut return_vector = Vec::new();

    for _ in 0..(n - 1) {
        std::io::stdin().read_line(&mut input_strings).ok();
        let v: Vec<u32> = input_strings
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        input_strings.clear();
        return_vector.push((v[0], v[1]));
    }

    return return_vector;
}

fn input_u32() -> u32 {
    let mut input_strings = String::new();
    std::io::stdin().read_line(&mut input_strings).ok();
    let v: Vec<u32> = input_strings
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    return v[0];
}

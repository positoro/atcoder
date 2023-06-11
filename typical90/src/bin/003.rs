fn main() {
    let n = input_u32();
    let a_b_tuple_vector: Vec<(u32, u32)> = input_u32_tuple_vector(n);
    let tuple_vector: Vec<(u32, u32)> = set_tuple_vector_zero_index(a_b_tuple_vector);
    let graph = create_graph(n, tuple_vector);
    let distance_from_zero_vector: Vec<i32> = dfs(&graph, 0);

    let (max_distance_node_from_zero, max_distance_from_zero): (i32, i32) =
        get_max_distance_node(distance_from_zero_vector);

    let distance_from_max_distance_vector: Vec<i32> = dfs(&graph, max_distance_node_from_zero);
    let (max_distance_node_from_max_distance, max_distance_from_max_distance): (i32, i32) =
        get_max_distance_node(distance_from_max_distance_vector);

    println!("{}", max_distance_from_max_distance + 1);
}

////////////////////////////////////////////////////////////////////////////////

fn dfs(graph: &Vec<Vec<i32>>, from_node: i32) -> Vec<i32> {
    let graph_length: usize = graph.len();
    let mut distance_vector: Vec<i32> = vec![-1; graph_length];
    distance_vector[from_node as usize] = 0;

    let mut stack: Vec<i32> = vec![from_node];

    while stack.is_empty() == false {
        let v = stack.pop().unwrap();
        for node_index in graph[v as usize].iter() {
            if distance_vector[*node_index as usize] == -1 {
                stack.push(*node_index);
                distance_vector[*node_index as usize] = distance_vector[v as usize] + 1;
            }
        }
    }
    return distance_vector;
}

fn get_max_distance_node(distance_vector: Vec<i32>) -> (i32, i32) {
    let mut max_node_index: i32 = -1;
    let mut max_distance: i32 = -1;

    for (index, value) in distance_vector.iter().enumerate() {
        if max_distance < *value {
            max_distance = *value;
            max_node_index = index as i32;
        }
    }
    return (max_node_index, max_distance);
}

fn set_tuple_vector_zero_index(a_b_tuple_vector: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut return_tuple_vector: Vec<(u32, u32)> = Vec::new();

    for v in a_b_tuple_vector.iter() {
        return_tuple_vector.push((v.0 - 1, v.1 - 1));
    }

    return return_tuple_vector;
}

fn create_graph(n: u32, a_b_tuple_vector: Vec<(u32, u32)>) -> Vec<Vec<i32>> {
    let mut return_vec_vec: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

    for v in a_b_tuple_vector.iter() {
        return_vec_vec[v.0 as usize].push(v.1 as i32);
        return_vec_vec[v.1 as usize].push(v.0 as i32);
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

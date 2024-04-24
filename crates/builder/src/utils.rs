use rand::Rng;

pub fn choose<'a, T>(items: &'a [T], weights: &[f32]) -> Option<&'a T> {
    if items.len() != weights.len() {
        panic!("Items and weights are different lengths!")
    }

    let mut cumulative = 0.0;
    let weighted_items: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            cumulative += weights[i];
            (item, cumulative)
        })
        .collect();

    if cumulative == 0.0 {
        return None;
    }

    let rand_num = rand::thread_rng().gen_range(0.0..cumulative);

    let chosen_item = weighted_items
        .iter()
        .find(|(_, weight)| rand_num < *weight)
        .map(|(id, _)| *id)
        .unwrap();

    return Some(chosen_item);
}

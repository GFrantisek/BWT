pub fn count_runs(bwt: &str) -> i32 {
    let mut run_count = 1;
    let mut prev_char = bwt.chars().next().unwrap();

    for c in bwt.chars().skip(1) {
        if c != prev_char {
            run_count += 1;
        }
        prev_char = c;
    }

    return run_count;
}


/*

fn find_minimal_runs(tree: &SuffixTree<'_>, node: &Node) -> (usize, HashMap<Vec<u8>, usize>) {
    let children: Vec<_> = node.children().collect();
    let mut dp: HashMap<Vec<u8>, usize> = HashMap::new();

    if children.is_empty() {
        let label = tree.label(node);
        let mut chars: Vec<u8> = label.to_vec();
        chars.sort();
        dp.insert(chars, 1);
        return (1, dp);
    }

    let mut total = 0;
    for &child in &children {
        let (child_total, child_dp) = find_minimal_runs(tree, child);
        total += child_total;
        for (k, v) in child_dp {
            *dp.entry(k).or_insert(0) += v;
        }
    }

    let mut min_runs = usize::MAX;
    let mut chars: Vec<u8> = Vec::new();

    for p in (0..children.len()).permutations(children.len()) {
        let mut runs = 0;
        let mut last: Option<Vec<u8>> = None;
        chars.clear();

        for &i in &p {
            let child = children[i];
            let label = tree.label(child);
            let child_chars: Vec<u8> = label.to_vec();
            chars.extend(&child_chars);
            if Some(&child_chars) != last.as_ref() {

                runs += 1;
                last = Some(child_chars);
            }
        }

        min_runs = min_runs.min(runs);
    }

    chars.sort();  // Sort the chars vector before inserting it into the map
    dp.insert(chars, min_runs);
    (total + min_runs, dp)
}

 */

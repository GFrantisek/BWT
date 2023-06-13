extern crate core;

mod bwt;
mod rotate;
mod count_runs;
mod cmp_by_spec_char;

use std::cmp::min;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::bwt::bwt;
use crate::count_runs::count_runs;
use suffix_tree::SuffixTree;
use suffix::SuffixTable;
use suffix_tree::Node;

const START_OF_TEXT: char = '\u{02}';
const END_OF_TEXT: char = '\u{03}';
fn main() {
    let mut input = String::new();
    let mut node_counter = 0;

    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut alphabet: Vec<_> = input.chars().collect::<HashSet<_>>().into_iter().collect();
    //alphabet.push(END_OF_TEXT);
    alphabet.sort_unstable();
    println!("Alphabet: {:?}", alphabet);

    let bwt = bwt(&input);
    println!("Input: {}", input);
    println!("\tBWT: {}", bwt.replace(START_OF_TEXT, "^").replace(END_OF_TEXT, "|"));
    println!("Number of runs with BWT: {}",count_runs(&*bwt));
    println!();

    let mut input_with_end = format!("{}{}", input, END_OF_TEXT);
    let tree = SuffixTree::new(input);
    println!("{:?}", tree);

    let mut dp: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let total = find_minimal_runs(&tree, tree.root(), &alphabet, &mut dp, &mut node_counter);
    println!("Minimum runs: {}", total);
    print_dp(&dp, &alphabet);

}

fn find_minimal_runs(tree: &SuffixTree<'_>, node: &Node, alphabet: &Vec<char>, dp: &mut HashMap<(usize, usize, usize), usize>, id: &mut usize) -> usize {
    *id += 1; // increment id as we start processing this node

    let label = tree.label(node);
    let label_string: String = label.iter().map(|&c| c as char).collect();
    println!("Current node value: {}", label_string);

    let children: Vec<_> = node.children().collect();

    // If node is a leaf
    if children.is_empty() {
        let label_char: char = label[0] as char;
        let label_index = alphabet.iter().position(|&c| c == label_char).unwrap();
        for i in 0..alphabet.len() {
            for j in 0..alphabet.len() {
                dp.insert((*id, i, j), if i == label_index && j == label_index { 1 } else { usize::MAX  }); //usize::max -> causing error with rest of code
            }
        }
        println!("I HATE LIFE!");

        return 1;
    }


    // p-> pre abc , aa = 1, bb =1, ostatok inf ak aplikuejem dalsi step co je zly step tak sa to zvysi for some reason
    for &child in &children {
        find_minimal_runs(tree, child, alphabet, dp, id);
    }

    /*
    // hen -> chyba
    //implement id to suffix tree ?
    for i in 0..alphabet.len() {
        for j in 0..alphabet.len() {
            let mut min_runs = usize::MAX;
            for &child in &children {
                for k in 0..alphabet.len() {
                    let previous_runs = *dp.get(&(*id, i, k)).unwrap_or(&usize::MAX);
                    let current_runs = *dp.get(&(*id, k, j)).unwrap_or(&usize::MAX);
                    min_runs = min(min_runs, previous_runs + current_runs - if k == j { 1 } else { 0 });
                }
            }
            dp.insert((*id, i, j), min_runs);
        }
    }


     */
    // Find the minimum number of runs for this node
    let mut total = usize::MAX;
    for i in 0..alphabet.len() {
        for j in 0..alphabet.len() {
            total = min(total, *dp.get(&(*id, i, j)).unwrap_or(&usize::MAX));
        }
    }



    total


}

fn print_dp(dp: &HashMap<(usize, usize, usize), usize>, x: &Vec<char>) {
    for ((node_id, i, j), v) in dp {
        let char_i = x[*i];
        let char_j = x[*j];
        println!("Node ID: {}, Chars: ({}, {}), Min Runs: {}", node_id, char_i, char_j, v);
    }
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

//pridaj dolar -> const START_OF_TEXT: char = '\u{02}';
//bw k vrhcolu stromu priradi mnozinu znakov substringu bwt co je pod tym vrhcolom
//T[v, x,y] ~ najmensi pocet runov v bwt segmente pod 'v' v tvare xNIECOy
// poziem sa na vseteky znaky
//prechadzaj len znaky v abecede

//tu je chyba asi
//tabulka ma pre kazdy vrchol -> kluc -> trojica vrchol symbol symbol
//tabulka je pre kazdy vrchol a pre pre kazdu dvojicu znakov [vrchol, znak, znak
// //T[v, a, b]
// T[v, a, b] = najmensi pocet runov spomedzi vserkych feasible arrangmentov zacinajucich a-ckom a konciacich b-ckom v podstrome v
// // mulltiple entries pre vrchol]
// list -> pozri ci = cj 1 a ci != cj inf
// nacitaj vstupny string premapuj znaky na interval a vytvor novu abecedu
// https://brenden.github.io/ukkonen-animation/


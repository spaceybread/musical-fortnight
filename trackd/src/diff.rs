use std::fs::read_to_string;
use std::io::{self, Read};
use std::fs::File;

pub fn diff(file_one: &str, file_two: &str) -> io::Result<()> {
    let mut changes: Vec<(i32, i32, char, char)> = Vec::new();

    let text1 = read_to_string(file_one)?;
    let text2 = read_to_string(file_two)?;
    
    let words1: Vec<&str> = text1.split_whitespace().collect();
    let words2: Vec<&str> = text2.split_whitespace().collect();

    let lcs = compute_lcs(&words1, &words2);

    let mut i = 0;
    let mut j = 0;

    for &(lcs_i, lcs_j) in &lcs {
        while i < lcs_i {
            // deletion
            println!("- {} : {} {}", words1[i], i, j);
            i += 1;
        }
        while j < lcs_j {
            // insertion
            println!("+ {} : {} {}", words2[j], i, j);
            j += 1;
        }
        i += 1;
        j += 1;
    }

    // remaining words
    while i < words1.len() {
        println!("- {} : {} {}", words1[i], "END", j);
        i += 1;
    }
    while j < words2.len() {
        println!("+ {} : {} {}", words2[j], i, "END");
        j += 1;
    }

    Ok(())
}

fn compute_lcs(a: &[&str], b: &[&str]) -> Vec<(usize, usize)> {
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut result = Vec::new();
    let (mut i, mut j) = (a.len(), b.len());

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push((i - 1, j - 1));
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result
}
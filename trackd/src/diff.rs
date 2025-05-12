use std::fs::read_to_string;
use std::fs::write;
use std::io;

pub fn diff(file_one: &str, file_two: &str) -> io::Result<(Vec<(i32, String, String)>)> {
    let mut changes: Vec<(i32, String, String)> = Vec::new();

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
            println!("- {} : {}", words1[i], i);
            changes.push((i as i32, words1[i].to_string(), "".to_string()));
            i += 1;
        }
        while j < lcs_j {
            // insertion
            println!("+ {} : {}", words2[j], j);

            let mut flag = true; 
            for idx in 0..changes.len() {
                if changes[idx].0 == j as i32 {
                    changes[idx].2 = words2[j].to_string(); 
                    flag = false; 
                }
            }
            if flag {
                changes.push((j as i32, "".to_string(), words2[j].to_string()));
            }

            j += 1;
        }

        i += 1;
        j += 1;
    }

    // remaining words
    while i < words1.len() {
        println!("- {} : {}", words1[i], i);
        changes.push((i as i32, words1[i].to_string(), "".to_string()));
        i += 1;
        
    }
    while j < words2.len() {
        println!("+ {} : {}", words2[j], j);
        let mut flag = true; 
        for idx in 0..changes.len() {
            if changes[idx].0 == j as i32 {
                changes[idx].2 = words2[j].to_string(); 
                flag = false; 
            }
        }
        if flag {
            changes.push((j as i32, "".to_string(), words2[j].to_string()));
        }
        j += 1;
    }

    Ok(changes)
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

pub fn apply_changes(file1: &str, changes: &[(i32, String, String)]) -> std::io::Result<(String)> {

    let text1 = read_to_string(file1)?;
    let mut words: Vec<String> = text1.split_whitespace().map(|s| s.to_string()).collect();
    
    let og_len = words.len() - 1; 
    for cha in changes {
        let wrd = cha.2.clone();
        if cha.0 as usize >= og_len {
            words.push(wrd); 
        } else {
            if cha.1 == "" {
                words.insert(cha.0 as usize, wrd); 
            } else {
                words[cha.0 as usize] = wrd; 
            }
        }
    }
    let mut output = words.join(" "); 
    words = output.split_whitespace().map(|s| s.to_string()).collect();
    output = words.join(" ");

    if text1.ends_with('\n') {
        output.push('\n');
    }

    write("testing_changes.txt", output.clone())?;
    Ok(output)
}
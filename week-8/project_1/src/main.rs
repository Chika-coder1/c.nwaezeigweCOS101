use std::io;

fn main() {
    // 1. The columns (tracks) and rows (levels)[cite: 23]
    let tracks = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // 2. The table data grid[cite: 23]
    let table = vec![
        vec!["Intern", "-", "Paralegal", "Placement"],
        vec!["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        vec!["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"],
        vec!["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        vec!["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        vec!["CEO", "Dean", "Partner", "Principal"],
    ];

    // 3. Ask for the track
    println!("Enter Track (Office Admin, Academic, Lawyer, Teacher):");
    let mut track_input = String::new();
    io::stdin().read_line(&mut track_input).unwrap();
    let track = track_input.trim();

    // 4. Ask for the role
    println!("Enter Role (e.g. Associate):");
    let mut role_input = String::new();
    io::stdin().read_line(&mut role_input).unwrap();
    let role = role_input.trim();

    // 5. Find the column number
    let mut col = 0;
    for i in 0..tracks.len() {
        if tracks[i] == track {
            col = i;
        }
    }

    // 6. Find the row level
    let mut ans = "Not Found";
    for r in 0..table.len() {
        if table[r][col] == role {
            ans = levels[r];
        }
    }

    // 7. Print the answer[cite: 23]
    println!("A {} in {} is {}", role, track, ans);
}
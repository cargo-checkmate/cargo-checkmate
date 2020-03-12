pub const PHASES: &[(&str, &[&str])] = &[
    ("fmt", &["--", "--check"]),
    ("build", &[]),
    ("test", &[]),
    ("checkmate", &["audit"]),
];

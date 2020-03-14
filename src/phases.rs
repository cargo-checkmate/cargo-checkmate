pub const PHASES: &[(&str, &[&str])] = &[
    ("check", &[]),
    ("fmt", &["--", "--check"]),
    ("build", &[]),
    ("test", &[]),
    ("checkmate", &["audit"]),
];

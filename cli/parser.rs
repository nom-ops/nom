use strsim::levenshtein;

const VALID_COMMANDS: &[&str] = &[
    "install", "i", "add", "uninstall", "remove", "rm",
    "init", "run", "start", "test", "t", "exec", "x",
    "publish", "p", "link", "ln", "update", "up", "outdated",
    "audit", "fund", "help", "h", "version", "v", "config",
    "set", "get", "list", "ls", "ll", "la", "prune", "pack",
    "ping", "prefix", "profile", "rebuild", "rb", "repo",
    "restart", "root", "shrinkwrap", "star", "stars", "stop",
    "team", "token", "undeprecate", "unpublish", "unstar",
    "view", "whoami", "access", "bin", "bugs", "c", "cache",
    "ci", "cit", "clean-install", "clean-install-test",
    "completion", "ddp", "dedupe", "deprecate", "diff",
    "dist-tag", "docs", "doctor", "edit", "explore", "find-dupes",
    "fix", "git", "hook", "info", "install-ci-test", "install-test",
    "it", "login", "logout", "org", "owner", "pack", "pkg",
    "query", "rebuild", "republish", "s", "se", "search",
    "set-script", "show", "sit", "stage", "tst", "un",
    "unlink", "unstar", "v", "ver", "version", "view",
    "watch", "why", "wtf", "xmas",
];

pub fn find_suggestion(input: &str) -> Option<String> {
    let mut best_match: Option<(String, usize)> = None;
    
    for cmd in VALID_COMMANDS {
        let dist = levenshtein(input, cmd);
        if dist <= 2 {
            match &best_match {
                Some((_, current_dist)) if *current_dist <= dist => continue,
                _ => best_match = Some(cmd.to_string(), dist),
            }
        }
    }
    
    best_match.map(|(cmd, _)| cmd)
}

pub fn is_valid_command(input: &str) -> bool {
    VALID_COMMANDS.contains(&input)
}

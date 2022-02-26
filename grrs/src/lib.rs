pub fn find_matches(content: &str, pattern: &str, mut write: impl std::io::Write) {
  for line in content.lines() {
      if line.contains(pattern) {
          writeln!(write, "{}", line);
      }
  }
}
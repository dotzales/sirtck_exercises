use std::io;

/// Zbiera tematy z pojedynczej linii wejścia rozdzielonej przecinkami.
/// Zwraca błąd `"Brak tematów do przećwiczenia."`, jeśli po oczyszczeniu nie ma żadnych wpisów.
pub fn collect_topics(line: &str) -> Result<Vec<String>, String> {
    let topics: Vec<String> = line
        .split(',')
        .map(|chunk| chunk.trim())
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| chunk.to_string())
        .collect();

    if topics.is_empty() {
        Err("Brak tematów do przećwiczenia.".to_string())
    } else {
        Ok(topics)
    }
}

/// Buduje numerowaną checklistę w formacie `"{}. [ ] {}"` dla każdego tematu.
pub fn format_checklist(items: &[String]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(index, topic)| format!("{}. [ ] {}", index + 1, topic))
        .collect()
}

/// Tworzy linię podsumowania z liczbą elementów i liczbą znaków bez spacji.
pub fn build_summary(items: &[String]) -> String {
    let count = items.len();
    let char_count: usize = items
        .iter()
        .map(|topic| topic.chars().filter(|c| !c.is_whitespace()).count())
        .sum();
    format!("Podsumowanie: {} elementów, {} znaków bez spacji", count, char_count)
}

/// Orkiestruje cały raport: zbiera tematy, buduje checklistę i dodaje linię podsumowania.
pub fn generate_report(line: &str) -> Result<Vec<String>, String> {
    match collect_topics(line) {
        Err(e) => Err(e),
        Ok(topics) => {
            let mut lines = format_checklist(&topics);
            lines.push(build_summary(&topics));
            Ok(lines)
        }
    }
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Nie udało się odczytać danych.");
        return;
    }

    match generate_report(buffer.as_str()) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
        }
        Err(message) => println!("{message}"),
    }
}

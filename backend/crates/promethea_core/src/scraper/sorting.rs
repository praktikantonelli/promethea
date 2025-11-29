pub fn get_name_sort(author_name: &str) -> String {
    // Takes the full name of an author and produces a string according to which the name should
    // be sorted. General logic: Sort by last "word" in name and comma-separate it from everything
    // else in the name, e.g. `Guy Le Best => Best, Guy Le`
    let mut tokens = author_name.split_whitespace().collect::<Vec<&str>>();

    match tokens.len() {
        0 => String::new(),
        1 => String::from(tokens[0]),
        _ => {
            let determining_name = tokens.pop().unwrap();
            format!("{}, {}", determining_name, tokens.join(" "))
        }
    }
}

pub fn get_title_sort(title: &str) -> String {
    // Required patterns:
    // the everythingelse -> everythingelse, the e.g. The Hobbit
    // a everythingelse -> everythingelse, a e.g. A Game of Thrones
    // an everythingelse -> everythingelse, an e.g. An Echo of Thigns to Come
    if let Some(prefix) = title.split_whitespace().next() {
        if ["A", "An", "The"].contains(&prefix) {
            let remainder = title.replace(prefix, "");
            let trimmed_remainder = remainder.trim();
            return format!("{trimmed_remainder}, {prefix}");
        }
    }
    title.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_firstname_lastname() {
        let names = [
            String::from("Brandon Sanderson"),
            String::from("Robert Jordan"),
            String::from("Tad Williams"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        let expected = vec![
            String::from("Sanderson, Brandon"),
            String::from("Jordan, Robert"),
            String::from("Williams, Tad"),
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn test_firstname_middlename_lastname() {
        let names = [
            String::from("Guy Gavriel Kay"),
            String::from("Orson Scott Card"),
            String::from("Justin Lee Anderson"),
        ];

        let expected = vec![
            String::from("Kay, Guy Gavriel"),
            String::from("Card, Orson Scott"),
            String::from("Anderson, Justin Lee"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_firstname_m_lastname() {
        let names = [
            String::from("Michael J. Sullivan"),
            String::from("Arthur C. Clarke"),
            String::from("Philip K. Dick"),
            String::from("Ursula K. Le Guin"),
        ];

        let expected = vec![
            String::from("Sullivan, Michael J."),
            String::from("Clarke, Arthur C."),
            String::from("Dick, Philip K."),
            String::from("Guin, Ursula K. Le"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_f_m_lastname() {
        let names = [
            String::from("R. R. Virdi"),
            String::from("S. A. Chakraborty"),
            String::from("M. L. Wang"),
        ];

        let expected = vec![
            String::from("Virdi, R. R."),
            String::from("Chakraborty, S. A."),
            String::from("Wang, M. L."),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_firstname_m_m_lastname() {
        let names = [
            String::from("James S. A. Corey"),
            String::from("George R. R. Martin"),
        ];

        let expected = vec![
            String::from("Corey, James S. A."),
            String::from("Martin, George R. R."),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_f_middlename_lastname() {
        let names = [
            String::from("R. Scott Bakker"),
            String::from("F. Scott Fitzgerald"),
        ];

        let expected = vec![
            String::from("Bakker, R. Scott"),
            String::from("Fitzgerald, F. Scott"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_f_m_m_lastname() {
        let name = "J. R. R. Tolkien";

        let expected = String::from("Tolkien, J. R. R.");
        let result = get_name_sort(name);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_singlename() {
        let names = [String::from("Baoshu"), String::from("Madonna")];

        let expected = vec![String::from("Baoshu"), String::from("Madonna")];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn test_titles() {
        let titles = [
            String::from("A Game of Thrones"),
            String::from("An Echo of Things to Come"),
            String::from("The Hobbit"),
            String::from("Neverwhere"),
            String::from("I Am Not A Serial Killer"),
            String::from("Mr Monster"),
            String::from("The Hero of Ages"),
            String::from("The Great Hunt"),
        ];

        let expected = vec![
            String::from("Game of Thrones, A"),
            String::from("Echo of Things to Come, An"),
            String::from("Hobbit, The"),
            String::from("Neverwhere"),
            String::from("I Am Not A Serial Killer"),
            String::from("Mr Monster"),
            String::from("Hero of Ages, The"),
            String::from("Great Hunt, The"),
        ];

        let results: Vec<String> = titles.iter().map(|title| get_title_sort(title)).collect();

        assert_eq!(expected, results);
    }
}

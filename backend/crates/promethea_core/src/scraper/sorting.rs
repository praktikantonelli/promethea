fn get_name_sort(author_name: String) -> String {
    // Required patterns:
    // firstname lastname               ->      lastname, firstname   e.g. Brandon Sanderson
    // firstname m. lastname            ->      lastname, firstname m.  e.g. Peter V. Brett
    // firstname middlename lastname    ->      lastname, firtsname middlename  e.g. Robert Louis Stevenson
    // firstname lastname otherlastname ->      lastname otherlastname, firstname e.g. Lois McMaster Bujold
    // f. f. lastname                   ->      lastname, f. f. e.g. R. F. Kuang
    // f. f. f. lastname                ->      lastname, f. f. f. e.g. J. R. R. Tolkien
    // firstname f. f. lastname         ->      lastname, firstname f. f. e.g. George R. R. Martin
    // f. lastname otherlastname        ->      lastname otherlastname, f. e.g. R. Scott Bakker
    // singlename                       ->      singlename e.g. Baosu
    // firstname f. prefix lastname     ->      lastname, firstname f. prefix e.g. Ursula K. Le Guin
    //
    // INFO: firstname middlename lastname and firstname lastname otherlastname cannot be reliably
    // distinguished, e.g., Orson Scott Card -> Scott is middle name, but H. Jon Benjamin -> Jon is
    // middle name. And Scott can be both a first or last name, so you also can't just keep a list
    // of common first or last names either.
    // Solution: Assume the pattern "firstname name lastname" has a middle name because that's much
    // more common. This will lead to wrong results with names like Lois McMaster Bujold, but
    // that's okay. Before calling this function, check the database for records of a given author,
    // and only call this function as a fallback!

    String::new()
}

fn get_title_sort(title: String) -> String {
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
    title
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

        let results: Vec<String> = names
            .iter()
            .map(|name| get_name_sort(name.to_owned()))
            .collect();

        let expected = vec![
            String::from("Sanderson, Brandon"),
            String::from("Jordan, Robert"),
            String::from("Williams, Tad"),
        ];

        assert_eq!(results, expected);
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

        let results: Vec<String> = titles
            .iter()
            .map(|title| get_title_sort(title.to_owned()))
            .collect();

        assert_eq!(expected, results);
    }
}

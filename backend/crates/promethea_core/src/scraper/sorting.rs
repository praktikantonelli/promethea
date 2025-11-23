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

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

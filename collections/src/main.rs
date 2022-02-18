use std::collections::HashMap;

fn main() {
    {
        // gross and very java like
        let v: Vec<i32> = Vec::new();
    }

    {
        // better
        let mut v = vec![1, 2, 3];
        v.push(5);
    }

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}

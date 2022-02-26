pub fn langmap(lang: &str) -> u8 {
    match lang {
        "c" => 4,
        "go" => 3,
        "cpp" => 4,
        "lua" => 2,
        "php" => 3,
        "sql" => 3,
        "java" => 3,
        "objc" => 3,
        "perl" => 3,
        "ruby" => 3,
        "rust" => 3,
        "swift" => 3,
        "csharp" => 3,
        "elixir" => 3,
        "nodejs" => 3,
        "groovy" => 3,
        "kotlin" => 2,
        "lolcode" => 0,
        "octave" => 3,
        "python3" => 3,
        "clojure" => 2,
        "brainfuck" => 0,
        "coffeescript" => 3,
        _ => 1
    }
}

pub fn bondapp(app: &str) -> u64 {
    match app {
        "chess" => 832012774040141894,
        "poker" => 755827207812677713,
        "awkword" => 879863881349087252,
        "fishing" => 814288819477020702,
        "youtube" => 880218394199220334,
        "betrayal" => 773336526917861400,
        "checkers" => 832013003968348200,
        "puttparty" => 763133495793942528,
        "spellcast" => 852509694341283871,
        "wordsnack" => 879863976006127627,
        "doodlecrew" => 878067389634314250,
        "lettertile" => 879863686565621790,
        "sketchyartist" => 879864070101172255,
        _ => 0
    }
}
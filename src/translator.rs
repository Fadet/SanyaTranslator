use phf::phf_map;

const DICTIONARY: phf::Map<char, &str> = phf_map!{
    'а' => "FI", 'б' => "6", 'в' => "I3",
    'г' => "i-", 'д' => "D", 'е' => "E",
    'ё' => "E", 'ж' => "}I{", 'з' => "3",
    'и' => "IJ", 'й' => "IJ", 'к' => "K",
    'л' => "JI", 'м' => "M", 'н' => "H",
    'о' => "O", 'п' => "TT", 'р' => "P",
    'с' => "C", 'т' => "T", 'у' => "Y",
    'ф' => "oIo", 'х' => "X", 'ц' => "LL",
    'ч' => "4", 'ш' => "LLI", 'щ' => "LLL",
    'ъ' => "-io", 'ы' => "IoI", 'ь' => "Io",
    'э' => "-)", 'ю' => "IO", 'я' => "9I",
};

pub fn translate(buf: String) -> String {
    let buf = buf.to_lowercase();

    let mut res = String::new();
    for ch in buf.chars() {
        if let Some(string) = DICTIONARY.get(&ch) {
            res.push_str(*string);
        } else {
            res.push(ch);
        }
    }

    return res;
}
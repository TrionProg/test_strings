
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

fn main() {
    let strings = vec![
        "din",
        "recede",
        "Success",
        "(( @",
        "Мой вариант"
    ];

    for s in strings.iter() {
        let out = magik(s);
        println!("{} => {}", s, out);
    }

    println!("Hello, world!");
}

fn magik(src_string:&str) -> String {
    /*
    Мы поддерживаем символы почти изо всех языков. Из-за UTF-8 символы могут занимать разное кол-во байт
    Поэтому в стоке "gЛ" 3 байта: 1 на g и 2 на Л, поэтому к символам трудно обращаться по индексу.
    Поэтому сделаем массив букв, сразу в лоуеркейз. Интересно, что to_lowercase() для чего-то неведомого
    Может выдать несколько символов в lowercase, что странно, пусть выдаст хоть один в таком случае
    Eсли ничего не выдаст, то это уже unreachable, и unwrap завершит программу.
    */
    let chars:Vec<char> = src_string.chars().map(|c| c.to_lowercase().next().unwrap()).collect();

    /*
    Сразу напрашиваются фокусы с XOR, однако в задании не сказано, сколько раз встречаются символы,
    и надо искать конкретный символ, так что выйдет примитивнее
    Если для каждого символа искать, есть ли его напарник, то выйдет O^2, а если выписывать символы,
    касаемо которых уже принято решение(Встречается только 1 раз, или найден напарник),
    то выйдет уже O^2/2 в худшем случае, а часто напарник будет находиться куда быстрее, или вердикт
    касаемо данной буквы будет найден из списка.
    Поскольку мы поддерживаем все языки, надо сделать список букв,
    Если бы все символы были ascii, то можно было бы оформить список как массив, где индекс -- код символа,
    и поиск был бы очень быстрым
    Но придётся использовать BTreeMap, он быстр для поиска log2n если в нём содержится мало символов,
    а их вряд ли будет больше 30, но зато будут поддерживаются все языки.
    */

    let mut chars_result_list = BTreeMap::new();
    let mut result_string = String::with_capacity(chars.len()); //чтобы не реаллокал память

    for (start, c) in chars.iter().enumerate(){
        match chars_result_list.entry(*c) {
            Entry::Occupied(e) => {//вердикт найден
                let has_duplicate = e.get();
                result_string.push(result_symbol(*has_duplicate));
            }
            Entry::Vacant(e) => {//впервые видим этот символ
                let has_duplicate = {//ищем дубликат правее
                    let mut has_duplicate = false;

                    for i in start+1..chars.len() {
                        if chars[i] == *c {
                            has_duplicate = true;
                            break; //и что бы зря не работать
                        }
                    }

                    has_duplicate
                };

                e.insert(has_duplicate);
                result_string.push(result_symbol(has_duplicate));
            }
        }
    }

    result_string
}

fn result_symbol(has_duplicate:bool) -> char {
    if has_duplicate {')'} else {'('}
}
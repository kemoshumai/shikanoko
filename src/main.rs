use rand::Rng;

fn main() {

    print!("ぬん！");

    let mut input = "し";
    while !input.is_empty() {
        print!("{}", input);
        input = next(input);
    }
    
    println!("。");
}

fn next(input: &str) -> &'static str {
    let mut rng = rand::thread_rng();
    match input {
        "し" => {
            // 等確率で「か」、「た」を抽選
            let n = rng.gen_range(0..2);
            match n {
                0 => "か",
                _ => "た"
            }
        },
        "か" => "の",
        "の" => "こ",
        "こ" => {
            // 50%の確率で「の」、25%の確率でそれぞれ「こ」、「し」を抽選
            let n = rng.gen_range(0..4);
            match n {
                0 => "こ",
                1 => "し",
                _ => "の"
            }
        },
        "た" => "ん",
        "ん" => {
            // 等確率で「た」、「(終端)」を抽選
            let n = rng.gen_range(0..2);
            match n {
                0 => "た",
                _ => ""
            }
        },
        _ => "し"
    }
}
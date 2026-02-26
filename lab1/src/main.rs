use std::io::{self, Write};

use lab1::*;

fn main() {
    println!("Допустимий діапазон параметрів: [{MIN_PARAM}; {MAX_PARAM}].\n");

    println!("Ввід параметрів для 1-ї прямої (через 2 точки):");
    let l1 = read_two_point_line("перша");

    println!("\nВвід параметрів для 2-ї прямої (через 2 точки):");
    let l2 = read_two_point_line("друга");

    println!("\nВвід параметрів для 3-ї прямої (канонічне рівняння):");
    let l3 = read_canonical_line("третя");

    if let Err(e) = validate_inputs(l1, l2, l3) {
        print_input_error_and_exit(e);
    }

    let a1 = LineABC::from_two_points(l1.p1, l1.p2);
    let a2 = LineABC::from_two_points(l2.p1, l2.p2);
    let a3 = LineABC::from_canonical(l3.x0, l3.y0, l3.l, l3.m);

    let p0 = PointI { x: l3.x0, y: l3.y0 };
    if point_on_line(p0, a1) && point_on_line(p0, a2) {
        println!(
            "Єдина точка перетину прямих (x0, y0), x0= {}, y0= {}",
            p0.x, p0.y
        );
        return;
    }

    match classify(a1, a2, a3) {
        Placement::AllCoincident => println!("Прямі співпадають"),
        Placement::NoIntersections => println!("Прямі не перетинаються"),
        Placement::OnePoint(p) => {
            println!(
                "Єдина точка перетину прямих (x0, y0), x0= {}, y0= {}",
                fmt_rat(p.x),
                fmt_rat(p.y)
            );
        }
        Placement::TwoPoints(p1, p2) => {
            println!(
                "Дві точки перетину прямих (x1, y1) = {}, (x2, y2) = {}",
                p1.fmt(),
                p2.fmt()
            );
        }
        Placement::ThreePoints(p1, p2, p3) => {
            println!(
                "Три точки перетину прямих (x1, y1) = {}, (x2, y2) = {}, (x3, y3) = {}",
                p1.fmt(),
                p2.fmt(),
                p3.fmt()
            );
        }
    }
}

fn read_two_point_line(which: &'static str) -> TwoPointLineInput {
    let x1 = read_i32_in_range(&format!("Задайте x1 ({which}): "), "x1");
    let y1 = read_i32_in_range(&format!("Задайте y1 ({which}): "), "y1");

    loop {
        let x2 = read_i32_in_range(&format!("Задайте x2 ({which}): "), "x2");
        let y2 = read_i32_in_range(&format!("Задайте y2 ({which}): "), "y2");

        let p1 = PointI { x: x1, y: y1 };
        let p2 = PointI { x: x2, y: y2 };

        if p1 == p2 {
            let e = InputError::DegenerateTwoPointLine { which };
            print_input_error(e);
            println!("Повторіть введення ДРУГОЇ точки (x2,y2).\n");
            continue;
        }

        return TwoPointLineInput { p1, p2 };
    }
}

fn read_canonical_line(which: &'static str) -> CanonLineInput {
    let x0 = read_i32_in_range(&format!("Задайте x0 ({which}): "), "x0");
    let y0 = read_i32_in_range(&format!("Задайте y0 ({which}): "), "y0");

    let l = read_i32_in_range_nonzero(&format!("Задайте l ({which}, l != 0): "), "l", which);
    let m = read_i32_in_range_nonzero(&format!("Задайте m ({which}, m != 0): "), "m", which);

    CanonLineInput { x0, y0, l, m }
}

fn read_i32_raw(prompt: &str) -> Result<i32, String> {
    print!("{prompt}");
    io::stdout()
        .flush()
        .map_err(|e| format!("Помилка виводу: {e}"))?;
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .map_err(|e| format!("Помилка вводу: {e}"))?;
    let t = s.trim();
    t.parse::<i32>()
        .map_err(|_| "Введено неціле число.".to_string())
}

fn read_i32_in_range(prompt: &str, name: &'static str) -> i32 {
    loop {
        match read_i32_raw(prompt) {
            Ok(v) => {
                if v < MIN_PARAM || v > MAX_PARAM {
                    print_input_error(InputError::OutOfRange { name, value: v });
                    println!("Спробуйте ще раз.\n");
                    continue;
                }
                return v;
            }
            Err(_) => {
                println!("\"Введено неціле число.\"");
                println!("\"Введіть ціле число та повторіть введення.\"");
                println!("Спробуйте ще раз.\n");
            }
        }
    }
}

fn read_i32_in_range_nonzero(prompt: &str, component: &'static str, which: &'static str) -> i32 {
    loop {
        let v = read_i32_in_range(prompt, component);
        if v == 0 {
            print_input_error(InputError::ZeroDirectionComponent { which, component });
            println!("Спробуйте ще раз.\n");
            continue;
        }
        return v;
    }
}

fn print_input_error(e: InputError) {
    let (desc, fix) = e.user_message();
    println!("\"{desc}\"");
    println!("\"{fix}\"");
}

fn print_input_error_and_exit(e: InputError) -> ! {
    print_input_error(e);
    std::process::exit(2);
}

fn point_on_line(p: PointI, l: LineABC) -> bool {
    let x = p.x as i64;
    let y = p.y as i64;
    l.a * x + l.b * y + l.c == 0
}

use std::io::{self, Write};

use lab1::*;

fn read_i32(prompt: &str) -> Result<i32, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| format!("Помилка виводу: {e}"))?;
    let mut s = String::new();
    io::stdin().read_line(&mut s).map_err(|e| format!("Помилка вводу: {e}"))?;
    let t = s.trim();
    t.parse::<i32>().map_err(|_| "Введено неціле число.".to_string())
}

fn in_range(name: &'static str, v: i32) -> Result<i32, InputError> {
    if v < MIN_PARAM || v > MAX_PARAM {
        Err(InputError::OutOfRange { name, value: v })
    } else {
        Ok(v)
    }
}

fn point_on_line(p: PointI, l: LineABC) -> bool {
    let x = p.x as i64;
    let y = p.y as i64;
    l.a * x + l.b * y + l.c == 0
}

fn main() {
    println!("Допустимий діапазон параметрів: [{MIN_PARAM}; {MAX_PARAM}].");

    // 2 через 2 точки, 1 канонічна
    let read_all = || -> Result<(TwoPointLineInput, TwoPointLineInput, CanonLineInput), String> {
        let x11 = read_i32("Задайте x11: ")?;
        let y11 = read_i32("Задайте y11: ")?;
        let x12 = read_i32("Задайте x12: ")?;
        let y12 = read_i32("Задайте y12: ")?;

        let x21 = read_i32("Задайте x21: ")?;
        let y21 = read_i32("Задайте y21: ")?;
        let x22 = read_i32("Задайте x22: ")?;
        let y22 = read_i32("Задайте y22: ")?;

        let x0 = read_i32("Задайте x0: ")?;
        let y0 = read_i32("Задайте y0: ")?;
        let l = read_i32("Задайте l (l != 0): ")?;
        let m = read_i32("Задайте m (m != 0): ")?;

        Ok((
            TwoPointLineInput { p1: PointI { x: x11, y: y11 }, p2: PointI { x: x12, y: y12 } },
            TwoPointLineInput { p1: PointI { x: x21, y: y21 }, p2: PointI { x: x22, y: y22 } },
            CanonLineInput { x0, y0, l, m },
        ))
    };

    let (l1, l2, l3) = match read_all() {
        Ok(v) => v,
        Err(parse_err) => {
            println!("Введено некоректні дані.\n\"{}\"\n\"Введіть цілі числа та повторіть введення.\"",
                     parse_err);
            std::process::exit(2);
        }
    };

    if let Err(e) = validate_inputs(l1, l2, l3) {
        let (desc, fix) = e.user_message();
        println!("\"{}\"\n\"{}\"", desc, fix);
        std::process::exit(2);
    }

    let a1 = LineABC::from_two_points(l1.p1, l1.p2);
    let a2 = LineABC::from_two_points(l2.p1, l2.p2);
    let a3 = LineABC::from_canonical(l3.x0, l3.y0, l3.l, l3.m);

    let p0 = PointI { x: l3.x0, y: l3.y0 };
    if point_on_line(p0, a1) && point_on_line(p0, a2) {
        println!("Єдина точка перетину прямих (x0, y0), x0= {}, y0= {}",
                 p0.x, p0.y);
        return;
    }

    match classify(a1, a2, a3) {
        Placement::AllCoincident => {
            println!("Прямі співпадають");
        }
        Placement::NoIntersections => {
            println!("Прямі не перетинаються");
        }
        Placement::OnePoint(p) => {
            println!("Єдина точка перетину прямих (x0, y0), x0= {}, y0= {}",
                     format_rat(p.x), format_rat(p.y));
        }
        Placement::TwoPoints(p1, p2) => {
            println!("Дві точки перетину прямих (x1, y1) = {}, (x2, y2) = {}",
                     p1.fmt(), p2.fmt());
        }
        Placement::ThreePoints(p1, p2, p3) => {
            println!(
                "Три точки перетину прямих (x1, y1) = {}, (x2, y2) = {}, (x3, y3) = {}",
                p1.fmt(), p2.fmt(), p3.fmt()
            );
        }
    }
}

fn format_rat(r: Rat) -> String {
    if r.den == 1 { format!("{}", r.num) } else { format!("{}/{}", r.num, r.den) }
}

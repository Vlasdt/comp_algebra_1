use std::fmt;

#[derive(Clone, Debug)]
enum Expr {
    Num(f64),
    Sym(&'static str),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}
use Expr::*;

fn is(e: &Expr, v: f64) -> bool {
    matches!(e, Num(n) if *n == v)
}

fn add(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        _ if is(&a, 0.0) => b,
        _ if is(&b, 0.0) => a,
        (Num(x), Num(y)) => Num(x + y),
        _ => Add(Box::new(a), Box::new(b)),
    }
}

fn mul(a: Expr, b: Expr) -> Expr {
    match (&a, &b) {
        _ if is(&a, 0.0) || is(&b, 0.0) => Num(0.0),
        _ if is(&a, 1.0) => b,
        _ if is(&b, 1.0) => a,
        (Num(x), Num(y)) => Num(x * y),
        _ => Mul(Box::new(a), Box::new(b)),
    }
}

fn neg(a: Expr) -> Expr {
    match a {
        Num(x) => Num(-x),
        Neg(x) => *x,
        _ => Neg(Box::new(a)),
    }
}

fn sub(a: Expr, b: Expr) -> Expr {
    add(a, neg(b))
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num(n) => write!(f, "{}", n),
            Sym(s) => write!(f, "{}", s),
            Add(a, b) => match &**b {
                Neg(x) => write!(f, "({} - {})", a, x),
                _ => write!(f, "({} + {})", a, b),
            },
            Mul(a, b) => write!(f, "{}*{}", a, b),
            Neg(a) => write!(f, "-{}", a),
        }
    }
}

type Matrix = Vec<Vec<Expr>>;

fn minor(m: &Matrix, row: usize, col: usize) -> Matrix {
    m.iter()
        .enumerate()
        .filter(|(i, _)| *i != row)
        .map(|(_, r)| {
            r.iter()
                .enumerate()
                .filter(|(j, _)| *j != col)
                .map(|(_, x)| x.clone())
                .collect()
        })
        .collect()
}

fn det(m: &Matrix) -> Expr {
    if m.is_empty() {
        return Num(1.0);
    }
    let mut res = Num(0.0);
    for j in 0..m.len() {
        let t = mul(m[0][j].clone(), det(&minor(m, 0, j)));
        res = if j % 2 == 0 { add(res, t) } else { sub(res, t) };
    }
    res
}

fn inverse(m: &Matrix) -> Result<Vec<Vec<String>>, String> {
    let n = m.len();
    if n == 0 || m.iter().any(|r| r.len() != n) {
        return Err("матрица должна быть квадратной".into());
    }
    let d = det(m);
    if is(&d, 0.0) {
        return Err("определитель равен 0".into());
    }
    Ok((0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let c = det(&minor(m, j, i));
                    let c = if (i + j) % 2 == 0 { c } else { neg(c) };
                    if is(&c, 0.0) { "0".into() } else { format!("({})/({})", c, d) }
                })
                .collect()
        })
        .collect())
}

fn main() {
    let m = vec![
        vec![Sym("a"), Sym("b")],
        vec![Sym("c"), Sym("d")],
    ];
    match inverse(&m) {
        Ok(inv) => inv.iter().for_each(|r| println!("{}", r.join(" ; "))),
        Err(e) => println!("Ошибка: {}", e),
    }

    let m = vec![
        vec![Num(2.0), Sym("x"), Num(0.0)],
        vec![Num(1.0), Num(1.0), Sym("y")],
        vec![Num(0.0), Sym("z"), Num(1.0)],
    ];
    match inverse(&m) {
        Ok(inv) => inv.iter().for_each(|r| println!("{}", r.join(" ; "))),
        Err(e) => println!("Ошибка: {}", e),
    }
}

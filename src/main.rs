use symbolic_math::expr::Expr;

enum Cell {
    Num(f64),
    Sym(&'static str),
}

type Matrix = Vec<Vec<Expr>>;

fn val(x: f64) -> Expr {
    Expr::new_val(x) // если не компилируется, см. примечание ниже
}

fn to_expr(c: &Cell) -> Expr {
    match c {
        Cell::Num(n) => val(*n),
        Cell::Sym(s) => Expr::new_var(s),
    }
}

// минор: матрица без строки row и столбца col
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

// определитель разложением по первой строке
fn det(m: &Matrix) -> Expr {
    if m.len() == 1 {
        return m[0][0].clone();
    }
    let mut res = val(0.0);
    for j in 0..m.len() {
        let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
        res = res + val(sign) * m[0][j].clone() * det(&minor(m, 0, j));
    }
    res
}

// обратная матрица = (алгебраические дополнения)^T / det
fn inverse(cells: &Vec<Vec<Cell>>) -> Matrix {
    let m: Matrix = cells.iter().map(|r| r.iter().map(to_expr).collect()).collect();
    let n = m.len();
    let d_inv = det(&m).pow(val(-1.0)); // 1 / det

    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
                    (val(sign) * det(&minor(&m, j, i)) * d_inv.clone()).simplify()
                })
                .collect()
        })
        .collect()
}

fn main() {
    use Cell::*;
    let a = vec![
        vec![Sym("a"), Sym("b")],
        vec![Sym("c"), Sym("d")],
    ];
    for row in inverse(&a) {
        let line: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        println!("{}", line.join("   "));
    }
}

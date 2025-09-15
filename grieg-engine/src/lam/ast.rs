#[derive(Clone, Debug, PartialEq, Eq)]
pub enum L {
    Var(String),
    Lam(String, Box<L>),    // λx. body
    App(Box<L>, Box<L>),    // f x
}

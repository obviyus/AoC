#[allow(unused_macros)]
#[macro_export]
macro_rules! scanline {
        ($x:expr) => (std::io::stdin().read_line(&mut $x).unwrap());
    }

#[allow(unused_macros)]
#[macro_export]
macro_rules! content {
    ($path:expr) => (
        std::io::BufReader::new(std::fs::File::open($path).unwrap())
    )
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! parse {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap());
}
fn main() {
    let first = 20151125u64;
    let mul = 252533u64;
    let div = 33554393u64;

    // let result = (0u64..).scan(first, |val, n| {
    //     *val = (*val * mul) % div;
    //     Some((*val, n))
    // }).find(|(val, n)| *val == first);
    //
    // println!("Found repeat after {result:?}");

    let row: u64 = 2978;
    let col: u64 = 3083;

    let area = row * col;
    let area_below = ((col - 1) * col) / 2;
    let area_after = ((row - 2) * (row - 1)) / 2;

    // first index is 1
    let result = (1..area + area_below + area_after).fold(first, |val, _| (val * mul) % div);

    //
    // 7227017 too high
    println!("Found at r{row} c{col}: {result}");

}
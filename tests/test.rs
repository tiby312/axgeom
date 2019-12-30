extern crate axgeom;

#[test]
fn test() {
    use axgeom::Axis;

    let a = axgeom::XAXIS;
    let b = a.next();
    let c = b.next();

    assert_eq!(generic(a), 1);
    assert_eq!(generic(b), 0);
    assert_eq!(generic(c), 1);

    fn generic<T: Axis>(a: T) -> usize {
        //known at compile time
        if a.is_xaxis() {
            1
        } else {
            0
        }
    }
}

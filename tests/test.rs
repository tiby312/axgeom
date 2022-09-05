use axgeom::ray::*;
use axgeom::rect::*;
use axgeom::vec2::*;
#[test]
fn raytest() {
    let aabb = rect(0.0f64, 10.0, 0.0, 10.0);

    let r = ray(vec2(5.0f64, 15.0), vec2(0.0, -1.0));
    assert_eq!(r.cast_to_rect(&aabb), CastResult::Hit(5.0));

    let r = ray(vec2(-6.0f64, 5.0), vec2(1.0, 0.0));
    assert_eq!(r.cast_to_rect(&aabb), CastResult::Hit(6.0));

    let r = ray(vec2(0.0f64, -5.0), vec2(0.0, 1.0));
    assert_eq!(r.cast_to_rect(&aabb), CastResult::Hit(50.0));

    let r = ray(vec2(10.0f64, -5.0), vec2(0.0, 1.0));
    assert_eq!(r.cast_to_rect(&aabb), CastResult::Hit(5.0));
}

#[test]
fn rect_rest() {
    let rect = rect(0, 10, 0, 10);

    let k = rect.furthest_distance_squared_to_point(vec2(0, 0));
    assert_eq!(k, 10 * 10 + 10 * 10);

    let k = rect.furthest_distance_squared_to_point(vec2(10, 10));
    assert_eq!(k, 10 * 10 + 10 * 10);

    let k = rect.furthest_distance_squared_to_point(vec2(5, 5));
    assert_eq!(k, 5 * 5 + 5 * 5);
}

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

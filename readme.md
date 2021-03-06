# axgeom

A library that provides a way to easily extract 1d ranges from a 2d container based off of the x or y axis statically through
type parameters. This is useful if you have a function that operates on an axis that recursively calls itself but at the same time alternates its axis.

## Example 

```
extern crate axgeom;

#[test]
fn test(){
	use axgeom::AxisTrait;

	let a=axgeom::XAXISS;
	let b=a.next();
	let c=b.next();

	assert_eq!(generic(a),1);
	assert_eq!(generic(b),0);
	assert_eq!(generic(c),1);

	fn generic<T:AxisTrait>(a:T)->usize{
		//known at compile time
		if a.is_xaxis(){
			1
		}else{
			0
		}
	}
}
```
# axgeom

A library that provides a way to easily extract 1d ranges from a 2d container based off of the x or y axis statically through
type parameters. This is useful if you have a function that operates on an axis that recursively calls itself but at the same time alternates its axis. Also provides useful functions that operate on types that implement Ord such as grow_to_fit().

# fp-vec

Functional programming features for Vecs and slices.

## Pushing to a slice

```
use fp_vec::Fpushable;

assert_eq!(
  [1, 2].fpush(3),
  vec![1, 2, 3]
);
```

## Pushing to a Vec

```
use fp_vec::Fpushable;

assert_eq!(
  vec![1, 2].fpush(3),
  vec![1, 2, 3]
);
```

Following [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#diffusematerials/truelambertianreflection)

Result:

![](output.png)

## Todos
1. benchmark: move thread_rng init out of the inner loop and test if there's a performance improvement
1. benchmark: replace thread_rng with SmallRng and see if there's any difference between them;

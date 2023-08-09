# Julia_sets

### Multithreaded Benchmark
Time taken for (20000, 10000) was 3.32 s

### Singlethreaded Benchmark
Time taken for (20000, 10000) was 18.73 s

![Fractal Image example](/new_fractal_image.png)

## CLI usage

```
Usage: Julia_sets [OPTIONS]

Options:
  -f, --file-path <FILE_PATH>
          File where fractal image will be saved [default: Fractal_image.png]
  -w, --width <WIDTH>
          Width of the image in pixels [default: 512]
  -h, --height <HEIGHT>
          Height of the image in pixels [default: 512]
  -s, --scale <SCALE>
          Scale is sort of like the reciprocol of a Zooming into the fractal, Smaller values = More Zoomed in [default: 2]
  -m, --max-interations <MAX_INTERATIONS>
          Max number of iterations to perform when calulcating pixel value. This is will also control how bright the final image fractal may appear [default: 300]
  -f, --fractal-seed <FRACTAL_SEED>
          Define Complex Seed that determines the fractal from the Julia set. This should have an absolute value less than (R^2 - R), currently R is hardcoded as R=2. Therefore the absolute value of seed should be less than 2 [default: -0.74543+0.11301i]
  -h, --help
          Print help
  -V, --version
          Print version
```

To generate example image;
```
./Julia_sets -f new_fractal_image.png -w 7680 -h 4320 -m 1000
```

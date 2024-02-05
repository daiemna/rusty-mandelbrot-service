# Mandelbrot Service
 Service generates images of mandelbrot set at a given location in the complex plane.

![Sample generated image](https://github.com/daiemna/rusty-mandelbrot-service/blob/main/docs/img/mandalbrot_sample.png)

## Run
1. `make run` to run the server
2. use following curl to cal the service.
   ```
   curl --location --request GET 'localhost:8080/mdb' \
    --header 'Content-Type: application/json' \
    --data '
    {
        "v_size": 540,
        "h_size": 360,
        "upper_left": [-1,1],
        "lower_right": [0,0]
    }'
   ```
3. service config are in `src/config/conf.rs`.
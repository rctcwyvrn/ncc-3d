
# testing dx
# run with const TS: f64 = 0.001;
# sed -i "s/const TS: f64 = .*;/const TS: f64 = 0.001;/g" src/main.rs

# for h in 0.5 0.2 0.1
# do
#     echo "testing h = $h"
#     sed -i "s/const H: f64 = .*;/const H: f64 = $h;/g" src/main.rs
#     cargo build --release  && time target/release/prototype_finite_diff > result.txt
#     mkdir "images/numerical_test_h=$h"
#     mv images/*png "images/numerical_test_h=$h"
#     mv result.txt "images/numerical_test_h=$h"
# done

# testing dt
sed -i "s/const H: f64 = .*;/const H: f64 = 0.2;/g" src/main.rs

for dt in 0.001 0.0005 0.0001
do
    echo "testing dt = $dt"
    sed -i "s/const TS: f64 = .*;/const TS: f64 = $dt;/g" src/main.rs
    cargo build --release  && time target/release/prototype_finite_diff > result.txt
    mkdir "images/numerical_test_dt=$dt"
    mv images/*png "images/numerical_test_dt=$dt"
    mv result.txt "images/numerical_test_dt=$dt"
done
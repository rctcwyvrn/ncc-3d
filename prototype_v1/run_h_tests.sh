results="h_test_results_finer.txt"
for radius in 1 2 3 4 5
do
    echo "Radius = $radius" >> $results
    sed -i "s/r = .*/r = $radius/g" scripts/make_sphere.py
    sed -i "s/let r: f64 = .*;/let r: f64 = $radius.0;/g" src/main.rs
    for subdivides in 2 3 4
    do
        sed -i "s/num_subdivides = .*/num_subdivides = $subdivides/g" scripts/make_sphere.py
        python scripts/make_sphere.py > /dev/null
        echo "" >> $results
        for h in 0.01 0.05 0.1 0.15 0.2 0.25 0.30 0.35 0.4 0.45 0.5 0.55
        do
            sed -i "s/const H: f64 = .*;/const H: f64 = $h;/g" src/laplacian.rs
            cargo build --release 2> /dev/null && target/release/prototype_v1 > /dev/null 2>> $results
        done
    done
done
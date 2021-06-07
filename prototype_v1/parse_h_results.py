lines = None
# with open("h_test_results_auto.txt") as f:
with open("h_test_results_finer.txt") as f:
    lines = f.readlines()

best_for_epsilon = {}
rows = []

for line in lines:
    if "|" not in line:
        continue

    parts = line.split("|")
    xs = [part.split("=")[1].strip() for part in parts]
    rows.append(",".join(xs))

    h = float(xs[0])
    epsilon = float(xs[1])
    l2 = float(xs[3])

    if epsilon in best_for_epsilon:
        if best_for_epsilon[epsilon][0] > l2:
            best_for_epsilon[epsilon] = (l2, h)
    else:
        best_for_epsilon[epsilon] = (l2, h)


with open("parsed_h_res.csv", "w") as f:
    f.write("h,epsilon,number of verticies, l2 error, l infinity error\n")
    for row in rows:
        f.write(row + "\n")

NUM_SUBDIV = 3
with open("best_h_res.csv", "w") as f:
    r = 0
    for (i, (epsilon, (l2, h))) in enumerate(best_for_epsilon.items()):
        if (i % NUM_SUBDIV) == 0:
            r += 1
        f.write(f"{epsilon},{h},{l2},{r}\n")
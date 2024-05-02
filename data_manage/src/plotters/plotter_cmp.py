import matplotlib.pyplot as plt
import numpy as np
import sys

if __name__ == '__main__':
    args = sys.argv
    assert(len(args) > 3)

    raw_data = args[1].split(";")
    raw_data.sort(key=lambda x: x.split(',')[0].lower())
    out_file = args[2]
    metric = args[3]

    names = [item.split(',')[0] for item in raw_data]
    values = [float(item.split(',')[1]) for item in raw_data]
    colors = ['grey' if value >= 0 else '#A7E6BF' for value in values]

    arth_mean = np.sum(values) / len(values)
    print(f"mean: {arth_mean}")

    plt.figure(dpi=800, figsize=(4, 3))
    plt.bar(names, values, color=colors)
    plt.ylabel(f'Change rate of {metric} (%)', fontsize=6)
    plt.yticks(fontsize=7)
    plt.xticks(rotation=90, fontsize=6.5)

    plt.tight_layout()
    plt.savefig(out_file)
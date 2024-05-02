import matplotlib.pyplot as plt
import numpy as np
import sys

if __name__ == '__main__':
    args = sys.argv
    assert(len(args) > 5)

    raw_data_1 = args[1].split(";")
    raw_data_1.sort(key=lambda x: x.split(',')[0].lower())
    
    raw_data_2 = args[2].split(";")
    raw_data_2.sort(key=lambda x: x.split(',')[0].lower())

    metric_1 = args[3]
    metric_2 = args[4]

    out_file = args[5]

    names = [item.split(',')[0] for item in raw_data_1]
    values = [float(item.split(',')[1]) for item in raw_data_1]
    colors = ['grey' if value >= 0 else '#A7E6BF' for value in values]

    arth_mean_1 = np.sum(values) / len(values)

    plt.figure(dpi=800, figsize=(4, 3))
    plt.bar(names, values, color=colors, label=metric_1)

    names = [item.split(',')[0] for item in raw_data_2]
    values = [float(item.split(',')[1]) for item in raw_data_2]
    colors = ['red' if value >= 0 else 'blue' for value in values]
    plt.scatter(names, values, color=colors, s=2, marker='x', label=metric_2)

    arth_mean_2 = np.sum(values) / len(values)
    print(f"arithmetic mean: {arth_mean_1} | {arth_mean_2}")

    plt.yticks(fontsize=7)
    plt.xticks(rotation=90, fontsize=6.5)

    plt.legend(loc='lower right', prop={'size': 4})

    plt.tight_layout()
    plt.savefig(out_file)
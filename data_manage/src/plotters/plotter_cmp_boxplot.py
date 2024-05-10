import matplotlib.pyplot as plt
import numpy as np
import sys

if __name__ == '__main__':
    args = sys.argv
    assert(len(args) > 3)

    raw_data = args[1].split(";")
    raw_data.sort(key=lambda x: x.split(':')[0].lower())
    out_file = args[2]
    metric = args[3]

    names = [item.split(':')[0] for item in raw_data]
    values = [np.array(item.split(':')[1].split(','), dtype=float) for item in raw_data]

    plt.figure(dpi=800, figsize=(8, 6))
    plt.boxplot(values, labels=names, showfliers=False)
    plt.ylabel(f'Change rate of {metric} (%)', fontsize=10)
    plt.yticks(fontsize=7)
    plt.xticks(rotation=90, fontsize=10)
    
    # colors = ['grey' if value >= 0 else '#A7E6BF' for value in values
    # plt.bar(names, values, color=colors)

    plt.tight_layout()
    plt.savefig(out_file)

    name_value = [item for item in zip(names, values)]
    name_value.sort(key=lambda x: x[1].mean())
    mean_values = []
    for (name, value) in name_value:
        print(name, ":", round(value.mean(), 2))
        mean_values.append(value.mean())
    print(np.array(mean_values).mean())
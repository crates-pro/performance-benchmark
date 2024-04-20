import matplotlib.pyplot as plt
import sys

if __name__ == '__main__':
    args = sys.argv
    assert(len(args) > 2)

    raw_data = args[1].split(";")
    out_file = args[2]

    names = [item.split(',')[0] for item in raw_data]
    values = [float(item.split(',')[1]) for item in raw_data]
    colors = ['grey' if value >= 0 else '#A7E6BF' for value in values]

    plt.figure(dpi=800, figsize=(4, 3))
    plt.bar(names, values, color=colors)
    plt.ylabel('Change rate of binary size (%)', fontsize=7)
    plt.yticks(fontsize=7)
    plt.xticks(rotation=90, fontsize=7)

    plt.tight_layout()
    plt.savefig(out_file)
import matplotlib.pyplot as plt
import numpy as np
import sys

def annotate_interval(y_axis: list[float]) -> list[list[float, float]]:
    tolerance = 7.0
    cur_offset_left = 0
    cur_offset_right = 0
    cur_offset = 0
    intervals = []
    for (i, y) in enumerate(y_axis):
        if i % 2 == 0:
            cur_offset = cur_offset_left
        else:
            cur_offset = cur_offset_right

        if i - 2 >= 0:
            interval = y - y_axis[i-2]
            if interval < tolerance:
                cur_offset += tolerance - interval
            else:
                cur_offset = max(0, cur_offset - (interval - tolerance))
        
        if i % 2 == 0:
            cur_offset_left = cur_offset
            intervals.append([-80, cur_offset])
        else:
            cur_offset_right = cur_offset
            intervals.append([20, cur_offset])
    return intervals

if __name__ == '__main__':
    args = sys.argv
    assert(len(args) > 3)

    raw_data_1 = args[1]
    raw_data_2 = args[2]
    label_1 = args[3]
    label_2 = args[4]
    out_file = args[5]

    data_pair_1 = [[item.split(',')[0], float(item.split(',')[1])] for item in raw_data_1.split(';')]
    data_pair_2 = [[item.split(',')[0], float(item.split(',')[1])] for item in raw_data_2.split(';')]
    data_pair_1.sort(key=lambda d: d[1])
    data_pair_2.sort(key=lambda d: d[1])

    data_1 = [item[1] for item in data_pair_1]
    data_2 = [item[1] for item in data_pair_2]
    interval_1 = annotate_interval(data_1)
    interval_2 = annotate_interval(data_2)
    annotate_1 = [item[0] for item in data_pair_1]
    annotate_2 = [item[0] for item in data_pair_2]

    plt.figure(dpi=500)
    plt.boxplot([data_1, data_2], labels=[label_1, label_2])

    plt.scatter([1]*len(data_1), data_1, color='green', marker='o', s=2)
    plt.scatter([2]*len(data_2), data_2, color='green', marker='o', s=2)

    for i, d1 in enumerate(data_1):
        plt.annotate(annotate_1[i], (1, d1), textcoords="offset points", xytext=(interval_1[i][0], interval_1[i][1]), arrowprops=dict(headlength = 0.1, width = 0.15, headwidth = 0.1, shrink=0.99, linewidth=0.2, mutation_scale=0.1), fontsize=9)
    for i, d2 in enumerate(data_2):
        plt.annotate(annotate_2[i], (2, d2), textcoords="offset points", xytext=(interval_2[i][0], interval_2[i][1]), arrowprops=dict(headlength = 0.1, width = 0.15, headwidth = 0.1, shrink=0.99, linewidth=0.2, mutation_scale=0.1), fontsize=9)

    plt.ylabel('Binary Size (MB)')

    plt.savefig(out_file)

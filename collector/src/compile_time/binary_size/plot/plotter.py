import matplotlib.pyplot as plt
import numpy as np
import sys

def annotate_interval(y_axis: list[float], scale: int) -> list[list[float, float]]:
    tolerance = 35.0 / scale
    cur_offset_left = -2 * scale
    cur_offset_right = -2 * scale
    cur_offset = 0
    intervals = []
    for (i, y) in enumerate(y_axis):
        if i % 2 == 0:
            cur_offset = cur_offset_left
        else:
            cur_offset = cur_offset_right

        if i - 2 >= 0:
            interval = (y - y_axis[i-2]) * scale * 1.6  - cur_offset
            if interval < tolerance:
                cur_offset = tolerance - interval
            else:
                cur_offset = -interval + tolerance

        if i % 2 == 0:
            cur_offset_left = cur_offset
            intervals.append([-480 / scale, cur_offset])
        else:
            cur_offset_right = cur_offset
            intervals.append([210 / scale, cur_offset])
    return intervals

if __name__ == '__main__':
    args = sys.argv
    assert(len(args) > 3)

    scale = 7

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

    geo_mean_1 = np.prod(data_1) ** (1 / len(data_1))
    geo_mean_2 = np.prod(data_2) ** (1 / len(data_2))
    print(f"geomean: {geo_mean_1} | {geo_mean_2}")

    log_data_1 = [np.log(item) for item in data_1]
    log_data_2 = [np.log(item) for item in data_2]
    interval_1 = annotate_interval(log_data_1, scale)
    interval_2 = annotate_interval(log_data_2, scale)
    annotate_1 = [item[0] for item in data_pair_1]
    annotate_2 = [item[0] for item in data_pair_2]

    plt.figure(dpi=500,  figsize=[scale, scale])
    plt.xticks([1, 2], [label_1, label_2])
    plt.violinplot([log_data_1, log_data_2], showmeans=False, showmedians=True)

    plt.scatter([1]*len(log_data_1), log_data_1, color='green', marker='o', s=2)
    plt.scatter([2]*len(log_data_2), log_data_2, color='green', marker='o', s=2)

    for i, d1 in enumerate(log_data_1):
        plt.annotate(annotate_1[i], (1, d1), textcoords="offset points", xytext=(interval_1[i][0], interval_1[i][1]), arrowprops=dict(headlength = 0.1, width = 0.15, headwidth = 0.1, shrink=0.99, linewidth=0.2, mutation_scale=0.1), fontsize=9)
    for i, d2 in enumerate(log_data_2):
        plt.annotate(annotate_2[i], (2, d2), textcoords="offset points", xytext=(interval_2[i][0], interval_2[i][1]), arrowprops=dict(headlength = 0.1, width = 0.15, headwidth = 0.1, shrink=0.99, linewidth=0.2, mutation_scale=0.1), fontsize=9)

    plt.ylabel('log (Binary Size (MB))')
    plt.savefig(out_file)

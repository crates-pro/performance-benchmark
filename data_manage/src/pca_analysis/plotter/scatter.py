import plotly.graph_objects as go
import sys
from typing import List

class DatawithLabel:
    def __init__(self, x, y, label) -> None:
        self.x, self.y, self.label = float(x), float(y), str(label)
    def display(self) -> str:
        return '(' + str(self.x) + ',' + str(self.y) + ',' + str(self.label) + ')'

def parse_data(raw_datas: str) -> List[DatawithLabel]:
    return list(map(lambda raw_data: DatawithLabel(raw_data.split(',')[0], raw_data.split(',')[1], raw_data.split(',')[2]), raw_datas[0:-1].split(';')))

if __name__ == '__main__':
    argc, argv = len(sys.argv), sys.argv

    assert(argc >= 5)

    datas = parse_data(argv[1])
    pc_x = int(argv[2])
    pc_y = int(argv[3])
    out_path = str(argv[4])

    fig = go.Figure()

    fig.add_trace(go.Scatter(
            x=list(map(lambda data: data.x, datas)),
            y=list(map(lambda data: data.y, datas)),
            mode="markers+text",
            name="",
            text=list(map(lambda data: data.label, datas)),
            textposition="bottom center",
        ))
    
    fig.update_layout(
        xaxis=dict(
            title="Principle Component {}".format(pc_x),
            automargin=True,
            color="black"
        ),
        yaxis=dict(
            title="Principle Component {}".format(pc_y),
            automargin=True,
            color="black"
        ),
        plot_bgcolor="lightgrey",
        paper_bgcolor="white",
        width=400,
        height=400,
    )

    fig.write_image(out_path)
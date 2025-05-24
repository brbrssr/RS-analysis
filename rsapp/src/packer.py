import flet as ft


class Packer:
    # Packer class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.on_change_callback = on_change
        self.parameters = {}

        self.packer_button = ft.ElevatedButton("Analyse", on_click=lambda e: self.pack_n_send())


    def pack_n_send(self, date, time, trade_pair, interval, other_parameters):
        datetime = f"{date}T{time}Z"
        min_window = "50"

        self.parameters = {
            "pair": trade_pair,
            "interval": interval,
            "date": datetime,
            "min_window": min_window,
            "n_iter": other_parameters[0],
            "frequency": other_parameters[1],
            "alpha": other_parameters[2],
            "ub": other_parameters[3],
            "hybrid": "true" if other_parameters[4] else "false",
            "max_iters": other_parameters[5],
            "n_lags": other_parameters[6],
            "max_iters_grid": other_parameters[7]
        }

        print(self.parameters)

        if self.on_change_callback:
            self.on_change_callback(self.parameters)


    # def unpack_outer_data(self):
    #     return self.parameters
    

    def render(self):
        return self.packer_button

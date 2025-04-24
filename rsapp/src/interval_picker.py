import flet as ft


class IntervalPicker:
    # IntervalPicker class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.on_change_callback = on_change

        self.unit_dd = ft.Dropdown(label="Time interval", width=200, 
                                   options=[ft.dropdown.Option(text=u, key=u) for u in ['1s', '15m', '1h', '1d', '1w', '1mo', '1y']])

        self.controls = ft.Row([self.unit_dd])
    
        self.unit_dd.on_change = self._on_any_change
    

    def _on_any_change(self, e):
        if self.on_change_callback:
            self.on_change_callback(self)


    def get_interval(self):
        if self.unit_dd.value:
            return f"{self.unit_dd.value}"
        return ""
    

    def render(self):
        return self.controls

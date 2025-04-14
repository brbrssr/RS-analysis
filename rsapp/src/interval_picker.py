import flet as ft


class IntervalPicker:
    # IntervalPicker class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.on_change_callback = on_change

        self.amount_tf = ft.TextField(label="Amount", width=150)
        self.unit_dd = ft.Dropdown(label="Units", width=150, 
                                   options=[ft.dropdown.Option(text=u, key=u) for u in ['s', 'm', 'h', 'w', 'mo', 'y']])

        self.controls = ft.Row([self.amount_tf, self.unit_dd])
    
        self.amount_tf.on_change = self._on_any_change
        self.unit_dd.on_change = self._on_any_change
    

    def _on_any_change(self, e):
        if self.on_change_callback:
            self.on_change_callback(self)


    def get_interval(self):
        if self.amount_tf.value and self.unit_dd.value:
            return f"{self.amount_tf.value} {self.unit_dd.value}"
        return ""
    

    def render(self):
        return self.controls

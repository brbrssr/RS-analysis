import flet as ft

class TimePicker:
    # TimePicker class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.on_change_callback = on_change

        self.hour_dd = ft.Dropdown(label="Hour", width=150, 
                                   options=[ft.dropdown.Option(f"{h:02}") for h in range(24)])
        self.minute_dd = ft.Dropdown(label="Minute", width=150, 
                                     options=[ft.dropdown.Option(f"{m:02}") for m in range(60)])
        self.second_dd = ft.Dropdown(label="Second", width=150,
                                     options=[ft.dropdown.Option(f"{s:02}") for s in range(60)])

        self.controls = ft.Row([self.hour_dd, self.minute_dd, self.second_dd])
    
        self.hour_dd.on_change = self._on_any_change
        self.minute_dd.on_change = self._on_any_change
        self.second_dd.on_change = self._on_any_change
    

    def _on_any_change(self, e):
        if self.on_change_callback:
            self.on_change_callback(self)


    def get_time(self):
        if self.hour_dd.value and self.minute_dd.value and self.second_dd.value:
            return f"{self.hour_dd.value}:{self.minute_dd.value}:{self.second_dd.value}"
        return None
    

    def render(self):
        return self.controls

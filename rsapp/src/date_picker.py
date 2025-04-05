import flet as ft
from datetime import datetime


class DatePicker:
    # DatePicker class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.current_year = datetime.now().year
        self.on_change_callback = on_change

        self.day_dd = ft.Dropdown(label="Day", width=150)
        self.month_dd = ft.Dropdown(
            label="Month",
            options=[ft.dropdown.Option(str(m)) for m in range(1, 13)],
            width=150,
            on_change=self._on_month_or_year_change,
        )
        self.year_dd = ft.Dropdown(
            label="Year",
            options=[ft.dropdown.Option(str(y)) for y in range(2010, self.current_year + 1)],
            width=150,
            on_change=self._on_month_or_year_change,
        )

        self.day_dd.on_change = self._on_any_change
        self.month_dd.on_change = self._on_month_or_year_change
        self.year_dd.on_change = self._on_month_or_year_change

        self.controls = ft.Row([self.day_dd, self.month_dd, self.year_dd], spacing=10)


    def _on_month_or_year_change(self, e):
        self._update_day_options()
        self._on_any_change(e)
        

    def _on_any_change(self, e):
        if self.on_change_callback:
            self.on_change_callback(self)


    def _update_day_options(self):
        if not self.month_dd.value or not self.year_dd.value:
            return

        month = int(self.month_dd.value)
        year = int(self.year_dd.value)

        if month in [1, 3, 5, 7, 8, 10, 12]:
            max_day = 31
        elif month in [4, 6, 9, 11]:
            max_day = 30
        elif month == 2:
            is_leap = (year % 4 == 0 and year % 100 != 0) or (year % 400 == 0)
            max_day = 29 if is_leap else 28
        else:
            max_day = 31

        self.day_dd.options = [ft.dropdown.Option(str(d)) for d in range(1, max_day + 1)]
        self.day_dd.value = None


    def get_date(self):
        if self.day_dd.value and self.month_dd.value and self.year_dd.value:
            return f"{self.day_dd.value}.{self.month_dd.value}.{self.year_dd.value}"
        return None
    

    def render(self):
        return self.controls

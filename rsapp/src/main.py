import flet as ft
from datetime import datetime


def main(page: ft.Page):
    current_year = datetime.now().year

    # Creating dropdowns for date selection
    day_dd = ft.Dropdown(label="Day", width=150)
    month_dd = ft.Dropdown(
        label="Month",
        options=[ft.dropdown.Option(str(m)) for m in range(1, 13)],
        width=150
    )
    year_dd = ft.Dropdown(
        label="Year",
        options=[ft.dropdown.Option(str(y)) for y in range(2010, current_year + 1)],
        width=150
    )

    # Debug text field
    result = ft.Text()

    def update_days(e=None):
        # Checking for month and year been selected
        if not month_dd.value or not year_dd.value:
            return

        month = int(month_dd.value)
        year = int(year_dd.value)

        # Defining the number of days in dropdown
        if month in [1, 3, 5, 7, 8, 10, 12]:
            max_day = 31
        elif month in [4, 6, 9, 11]:
            max_day = 30
        elif month == 2:
            is_leap = (year % 4 == 0 and year % 100 != 0) or (year % 400 == 0)
            max_day = 29 if is_leap else 28
        else:
            max_day = 31  # fallback

        # Updating day_dd
        day_dd.options = [ft.dropdown.Option(str(d)) for d in range(1, max_day + 1)]
        day_dd.value = None  # recover day selection
        page.update()

    def show_result(e=None):
        if day_dd.value and month_dd.value and year_dd.value:
            result.value = f"You entered date: {day_dd.value}.{month_dd.value}.{year_dd.value}"
        else:
            result.value = "Enter full date"
        page.update()

    # Adding handlers
    month_dd.on_change = update_days
    year_dd.on_change = update_days
    day_dd.on_change = show_result
    month_dd.on_change = lambda e: [update_days(e), show_result(e)]
    year_dd.on_change = lambda e: [update_days(e), show_result(e)]

    page.add(
        ft.Row([day_dd, month_dd, year_dd], spacing=10),
        result
    )

ft.app(main)

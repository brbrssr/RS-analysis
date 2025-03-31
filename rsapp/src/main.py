import datetime
import flet as ft


def main(page: ft.Page):

    current_date = datetime.date().today()

    def get_day():
        options_day = []
        if get_month() in [1, 3, 5, 7, 8, 10, 12]:
            options_day = [x for x in range(1, 32)]
        if get_month in [4, 6, 9, 11]:
            options_day = [x for x in range(1, 31)]
        if get_month() == 2:
            leap_years = []
            leap_year = 2012
            while leap_year <= current_date[0]:
                leap_years.append(leap_year)
                leap_year += 4
            if current_date[0] in leap_years:
                options_day = [x for x in range(1, 30)]
            else:
                options_day = [x for x in range(1, 29)]
        
        output_day = []
        
        for day in options_day:
            output_day.append(ft.DropdownOption(key=day, content=ft.Text(value=day)))
        
        return output_day

    

    def get_month():
        options_month = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        output_month = []

        for month in options_month:
            output_month.append(ft.DropdownOption(key=month, content=ft.Text(value=month)))
        
        return output_month
    

    def get_year():
        options_year = [x for x in range(2010, current_date[0] + 1)]
        output_year = []

        for year in options_year:
            output_year.append(ft.DropdownOption(key=year, content=ft.Text(value=year)))
        
        return output_year


    page.add(ft.Dropdown(label="Day", options=get_day()))
    page.add(ft.Dropdown(label="Month", options=get_month()))
    page.add(ft.Dropdown(label="Year", options=get_year()))
    page.add(ft.Text(value="RS-analysis"))


ft.app(main)

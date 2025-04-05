import flet as ft
import date_picker
import time_picker


def main(page: ft.Page):
    result_date = ft.Text() # Text fields. Just for debug
    result_time = ft.Text()
    

    def on_date_change(date_picker):
        date = date_picker.get_date()
        result_date.value = f"Date: {date}" if date else "Date not selected"
        page.update()
    
    datePicker = date_picker.DatePicker(on_change=on_date_change)

    page.add(datePicker.render(), result_date)


    def on_time_change(time_picker):
        time = time_picker.get_time()
        result_time.value = f"Time: {time}" if time else "Time not selected"
        page.update()
    
    timePicker = time_picker.TimePicker(on_change=on_time_change)

    page.add(timePicker.render(), result_time)


ft.app(main)

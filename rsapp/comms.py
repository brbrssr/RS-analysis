day = ""
month = ""
year = ""

hour = ""
minute = ""
second = ""

trade_pair = ""

duration = ""
units = ""


def submit_form(day, month, year, hour, minuite, second, trade_pair, duration, units):
    datetime = year + "-" + month + "-" + day + "T" + hour + ":" + minuite + ":" + second + "Z"
    time_step = duration + units

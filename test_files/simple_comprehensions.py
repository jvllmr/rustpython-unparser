import random
import string

generator = (value for value in random.choices(string.ascii_letters))
set_ = {def_ for def_ in random.choices(string.ascii_letters)}
dict_ = {k: v for k, v in enumerate(random.choices(string.ascii_letters))}
list_ = [value for value in random.choices(string.ascii_letters)]

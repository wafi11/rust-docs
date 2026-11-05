import random
import string

def random_name():
    return ''.join(random.choices(string.ascii_uppercase, k=5))

cities = ["New York", "Los Angeles", "Chicago", "Boston", "Seattle", "Atlanta", "Denver", "San Francisco"]

with open("sample_data_large.csv", "w", encoding="utf-8") as f:
    f.write("id,name,age,city,salary\n")
    for i in range(1, 30000001):
        name = random_name()
        age = random.randint(20, 60)
        city = random.choice(cities)
        salary = random.randint(40000, 100000)
        f.write(f"{i},{name},{age},{city},{salary}\n")
# Converts .chn to chickens-per-line

with open("[DESTINATION FILENAME]", "r") as file:
    lines = file.readlines()

with open("[DESTINATION FILENAME]", "w") as file:
    for line in lines:
        num_chickens = line.count("chicken")
        file.write(str(num_chickens) + "\n")
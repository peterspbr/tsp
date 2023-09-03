import matplotlib.pyplot as plt

# Define your list of cities and their coordinates
cities = {
    "A": (0, 0),
    "B": (2, 1),
    "C": (2, 3),
    "D": (1, 2),
    "E": (0.5, 2),
    "F": (0, 1.5),
}

# Define the tour order (replace with your shortest_tour if needed)
tour_order = ["A", "B", "C", "D", "E", "F"]

# Create a plot
plt.figure(figsize=(8, 6))

# Plot the cities as points
for city, (x, y) in cities.items():
    plt.plot(x, y, 'ro')  # 'ro' for red circles

# Plot the tour as lines
for i in range(len(tour_order)):
    city1 = tour_order[i]
    city2 = tour_order[(i + 1) % len(tour_order)]  # Wrap around to the first city
    x1, y1 = cities[city1]
    x2, y2 = cities[city2]
    plt.plot([x1, x2], [y1, y2], 'b-')  # 'b-' for blue lines

# Set labels for the cities
for city, (x, y) in cities.items():
    plt.annotate(city, (x, y), textcoords="offset points", xytext=(0,10), ha='center')

# Set plot attributes
plt.title("Traveling Salesman Problem")
plt.xlabel("X-coordinate")
plt.ylabel("Y-coordinate")

# Show the plot
plt.grid()
plt.show()


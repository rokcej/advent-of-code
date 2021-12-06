import numpy as np

initial = np.loadtxt("input", dtype=np.uint64, delimiter=",")
initial_indices, initial_counts = np.unique(initial, return_counts=True)
counts = np.zeros(9, dtype=np.uint64)
counts[initial_indices] = initial_counts

for day in range(256):
	counts = np.roll(counts, -1)
	counts[6] += counts[-1]

print(np.sum(counts))


# # Original brute-force solution
# with open("input") as f:
# 	initial = [int(x) for x in f.read().split(",")]

# sim = [0]
# f = [1]
# for day in range(80):
# 	n = len(sim)
# 	for i in range(n):
# 		sim[i] -= 1
# 		if sim[i] < 0:
# 			sim[i] = 6
# 			sim.append(8)
# 	f.append(len(sim))

# result = 0
# for x in initial:
# 	result += f[-(x+1)]

# print(result)

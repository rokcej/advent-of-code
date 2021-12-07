with open("input") as f:
	crabs = list(map(int, f.read().split(",")))

def S(n): return n * (n+1) // 2

align_min = sum(map(S, crabs))

for x in range(min(crabs), max(crabs)+1):
	align = 0
	for crab in crabs:
		align += S(abs(crab - x))
	if align < align_min:
		align_min = align

print(align_min)

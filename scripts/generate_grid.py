import json
import random

def generate_grid(rows, cols, filename):
    nodes = []
    edges = []
    cpts = {}
    domains = {}

    for r in range(rows):
        for c in range(cols):
            node = f"N_{r}_{c}"
            nodes.append(node)
            domains[node] = ["low", "high"]
            
            parents = []
            if r > 0:
                parent = f"K_{r-1}_{c}"
                edges.append([parent, node])
                parents.append(parent)
            if c > 0:
                parent = f"K_{r}_{c-1}"
                edges.append([parent, node])
                parents.append(parent)
            
            # Generate CPT
            # 2 states per parent -> 2^len(parents) rows
            # Each row has 2 probabilities summing to 1
            num_rows = 2 ** len(parents)
            cpt_table = []
            
            def make_row():
                p = random.random()
                return [p, 1.0 - p]

            # We need to structure the CPT correctly
            # If 0 parents: [p, 1-p]
            # If 1 parent: [[p1, 1-p1], [p2, 1-p2]]
            # If 2 parents: [[[p1..], [p2..]], [[p3..], [p4..]]]
            
            if len(parents) == 0:
                cpts[node] = make_row()
            elif len(parents) == 1:
                cpts[f"{node}|{parents[0]}"] = [make_row() for _ in range(2)]
            elif len(parents) == 2:
                # Outer parent is parents[0], Inner is parents[1]
                table = []
                for _ in range(2): # Outer
                    inner = []
                    for _ in range(2): # Inner
                        inner.append(make_row())
                    table.append(inner)
                cpts[f"{node}|{','.join(parents)}"] = table

    data = {
        "name": f"Grid {rows}x{cols}",
        "nodes": nodes,
        "domains": domains,
        "edges": edges,
        "cpts": cpts,
        "queries": []
    }

    with open(filename, 'w') as f:
        json.dump(data, f, indent=2)

if __name__ == "__main__":
    generate_grid(500, 500, "fixtures/large_scale/grid_500x500.json")

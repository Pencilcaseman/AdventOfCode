# pyright: standard


def trace_to_svg(input_file, output_file="shape.svg"):
    points = []

    # 1. Parse Data
    try:
        with open(input_file, "r") as f:
            for line in f:
                parts = line.strip().split(",")
                if len(parts) == 2:
                    # (row, col) -> row is Y, col is X
                    points.append((int(parts[0]), int(parts[1])))
    except FileNotFoundError:
        print("File not found.")
        return

    if not points:
        return

    # 2. Configuration
    scale = 0.01  # Pixels per unit
    padding = 50  # Padding around the image

    # 3. Calculate Dimensions
    # We normalize coordinates so the top-left-most point is near (0,0)
    rows = [p[0] for p in points]
    cols = [p[1] for p in points]

    min_row, max_row = min(rows), max(rows)
    min_col, max_col = min(cols), max(cols)

    width = (max_col - min_col) * scale + (2 * padding)
    height = (max_row - min_row) * scale + (2 * padding)

    # 4. Generate SVG String
    # SVG coordinate system: (0,0) is top-left.
    # This matches matrix indexing (row=0 is top), so NO coordinate flipping needed!

    svg = []
    svg.append(
        f'<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg" style="background-color:white">'
    )

    # Build the path
    # Move to first point
    start_y = (points[0][0] - min_row) * scale + padding
    start_x = (points[0][1] - min_col) * scale + padding

    path_data = [f"M {start_x} {start_y}"]

    # Draw lines to subsequent points
    for r, c in points[1:]:
        y = (r - min_row) * scale + padding
        x = (c - min_col) * scale + padding
        path_data.append(f"L {x} {y}")

    # 'Z' closes the path
    path_data.append("Z")

    svg.append(
        f'  <path d="{" ".join(path_data)}" stroke="black" stroke-width="4" fill="lightgray" />'
    )

    # (Optional) Draw dots at vertices
    for r, c in points:
        y = (r - min_row) * scale + padding
        x = (c - min_col) * scale + padding
        svg.append(f'  <circle cx="{x}" cy="{y}" r="0.00001" fill="red" />')

    svg.append("</svg>")

    # 5. Write to file
    with open(output_file, "w") as f:
        f.write("\n".join(svg))

    print(f"Success! Open '{output_file}' in your browser to see the shape.")


# --- Run it ---
# Create dummy file
# with open("points.txt", "w") as f:
#     f.write("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3")

trace_to_svg("../../input/year2025/day09.txt")


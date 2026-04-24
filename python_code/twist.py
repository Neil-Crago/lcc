import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

def generate_hex_lattice(size=5):
    """Generates a hexagonal lattice for a graphene layer."""
    points = []
    for i in range(-size, size):
        for j in range(-size, size):
            x = i * 1.5
            y = (j + (i % 2) * 0.5) * np.sqrt(3)
            points.append([x, y, 0])
    return np.array(points)

def rotate_points(points, angle):
    """Rotates lattice points around the Z-axis."""
    c, s = np.cos(angle), np.sin(angle)
    rotation_matrix = np.array([[c, -s, 0], [s, c, 0], [0, 0, 1]])
    return points @ rotation_matrix.T

# Parameters from SpaceTCO Spec
delta_gap = 0.0929 * np.pi  # Angular shear 
layer_spacing = 3.35        # Representative spacing in Angstroms

# Generate Layers
layer_base = generate_hex_lattice()
layer_twisted = rotate_points(layer_base, delta_gap)
layer_twisted[:, 2] = layer_spacing

# Plotting
fig = plt.figure(figsize=(10, 8))
ax = fig.add_subplot(111, projection='3d')

# Scatter plot for lattice atoms
ax.scatter(layer_base[:,0], layer_base[:,1], layer_base[:,2], c='blue', alpha=0.6, label='Base Layer (Graphene)')
ax.scatter(layer_twisted[:,0], layer_twisted[:,1], layer_twisted[:,2], c='gold', alpha=0.8, label='Twisted Layer (Bismuth)')

ax.set_title(rf"Bismuth-Graphene Torsional Twist ($\Delta_{{gap}} = 0.0929\pi$)")
ax.set_zlabel("Z-axis (Lattice Spacing)")
ax.legend()
plt.savefig('torsional_twist_viz.png', dpi=300)